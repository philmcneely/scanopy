use std::net::Ipv4Addr;

use crate::daemon::discovery::integration::snmp::sim::mibs::{BridgeTable, FdbEntry};
use crate::daemon::discovery::integration::snmp::sim::tables::{IfRow, IfTable};
use crate::daemon::discovery::integration::snmp::sim::{AgentFaults, Purpose, SimDevice, Tables};
use crate::daemon::discovery::integration::snmp::types::{SystemInfo, VlanInfo};
use crate::server::credentials::r#impl::types::CredentialType;

use super::inline;

/// A device that answers the probe's `sysDescr` and then times out on every table.
///
/// The customer report this exists for: one credential, two devices, credential result
/// `Collected`, and on each device every walk group returned nothing. The warnings said each table
/// "stopped answering", named no interfaces, and added a summary line, eight lines a device. This
/// is the shape that produces exactly that set: the system group answers, and every request under
/// every table subtree, GETBULK and GETNEXT alike, is dropped. The GETNEXT fallback has nothing to
/// recover, which is what separates it from `switch-bulkdrop-01`.
///
/// `sysServices` has no bridge bit, as on a router or firewall. A switch that sets it gets one more
/// line, saying it advertised a bridge and served no bridge-port table.
pub fn device() -> SimDevice {
    SimDevice {
        name: "switch-silent-01",
        ip: Ipv4Addr::UNSPECIFIED,
        purpose: Purpose::Regression {
            issue: "the silent-devices customer report",
            defect: "every table read timed out, and the warnings said each table stopped answering, left out the interfaces, and repeated the device eight times",
        },
        credential: CredentialType::SnmpV2c {
            community: inline("netdefault"),
        },
        system: system("switch-silent-01"),
        tables: Tables {
            faults: AgentFaults {
                silence: AgentFaults::every_table(),
                ..Default::default()
            },
            ..tables(0x5a)
        },
        arp_handler: Default::default(),
        suppresses: Vec::new(),
        rejects_getbulk: None,
    }
}

pub(super) fn system(name: &'static str) -> SystemInfo {
    SystemInfo {
        sys_descr: Some("Scanopy SNMP simulator, system-group-only profile".into()),
        sys_object_id: Some("1.3.6.1.4.1.99999.6".into()),
        sys_name: Some(name.into()),
        sys_location: Some("Lab".into()),
        sys_contact: Some("netops@example.com".into()),
        sys_services: Some(72),
        sys_uptime: None,
        // Published from the ifTable at emission, never stored.
        if_number: None,
    }
}

/// An ordinary small switch's tables, so a fault that hides them hides something. `octet` keeps
/// each device's MACs its own.
pub(super) fn tables(octet: u8) -> Tables {
    let mac = |last: u8| {
        format!("00:1a:2b:00:{octet:02x}:{last:02x}")
            .parse()
            .unwrap()
    };
    Tables {
        if_table: Some(IfTable::new(vec![
            IfRow::port(1, "Gi0/1", Some(mac(1))).name("Gi0/1"),
            IfRow::port(2, "Gi0/2", Some(mac(2))).name("Gi0/2"),
            IfRow::port(3, "Gi0/3", Some(mac(3))).name("Gi0/3"),
        ])),
        bridge: BridgeTable::derived()
            .fdb(vec![
                FdbEntry::learned(mac(0x11), 1),
                FdbEntry::learned(mac(0x12), 2),
            ])
            .vlans(vec![VlanInfo {
                vlan_id: 10,
                name: "DATA".into(),
            }])
            .port_vlans(vec![(1, 10), (2, 10), (3, 10)]),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use crate::daemon::discovery::integration::snmp::sim::harness;
    use crate::daemon::discovery::service::warnings::snmp_walk_shortfalls;
    use crate::daemon::discovery::types::warnings::{
        DiscoveryWarning, SnmpWalkCause, SnmpWalkFailure, SnmpWalkGroup,
    };

    /// The reproduction: the groups that fall short are exactly the customer's, the bridge-port
    /// numbering among them, plus the interfaces, which the walk now reports and then did not.
    /// The derived bridge groups and the LLDP local ports stay quiet, as they did for the customer.
    #[tokio::test]
    async fn the_groups_that_fall_short_are_the_customers() {
        let scan = harness::scan("switch-silent-01").await;
        let mut groups: Vec<SnmpWalkGroup> =
            snmp_walk_shortfalls("192.0.2.1".parse().unwrap(), scan.outcome)
                .into_iter()
                .filter(|shortfall| !shortfall.returned_any)
                .map(|shortfall| shortfall.group)
                .collect();
        groups.sort();

        let mut customer = vec![
            SnmpWalkGroup::Lldp,
            SnmpWalkGroup::Cdp,
            SnmpWalkGroup::ArpTable,
            SnmpWalkGroup::DeviceInventory,
            SnmpWalkGroup::IpAddresses,
            SnmpWalkGroup::VlanNames,
            SnmpWalkGroup::BridgePortNumbering,
            SnmpWalkGroup::Interfaces,
        ];
        customer.sort();
        assert_eq!(groups, customer);
        assert!(scan.outcome.returned_nothing());
    }

    /// One line for the device, naming every table that failed and that each timed out,
    /// interfaces included. The customer got a line per table and a summary line beside them.
    #[tokio::test]
    async fn every_failed_table_is_named_once_with_its_cause() {
        let warnings = harness::walk_warnings("switch-silent-01").await;

        let [
            DiscoveryWarning::SnmpCollectedNothing {
                failures: Some(failures),
                ..
            },
        ] = warnings.as_slice()
        else {
            panic!("expected the one whole-device line, got {warnings:?}");
        };
        let groups: Vec<SnmpWalkGroup> = failures.iter().map(|f| f.group).collect();
        assert_eq!(
            groups,
            vec![
                SnmpWalkGroup::Interfaces,
                SnmpWalkGroup::Lldp,
                SnmpWalkGroup::Cdp,
                SnmpWalkGroup::BridgePortNumbering,
                SnmpWalkGroup::ArpTable,
                SnmpWalkGroup::DeviceInventory,
                SnmpWalkGroup::IpAddresses,
                SnmpWalkGroup::VlanNames,
            ],
            "the customer's groups plus the interfaces, and neither group keyed by the bridge-port \
             numbering that failed before them"
        );
        assert!(
            failures.iter().all(|f| matches!(
                f,
                SnmpWalkFailure {
                    cause: SnmpWalkCause::TimedOut,
                    ..
                }
            )),
            "{failures:?}"
        );
    }
}
