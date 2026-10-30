//! Which known hosts each switch port's forwarding table has learned.
//!
//! A port's `fdb_macs` are MACs seen behind it. Matched against the MACs the bundle's addresses
//! and interfaces carry, they say which devices are reached through that port, which is what lets
//! an IP be traced to the switch port it sits behind. One definition, read by the link-state
//! classification (a port with a known device behind it is linked) and by the search's node
//! associations, so the two cannot disagree about which ports face which hosts.

use std::collections::{HashMap, HashSet};

use mac_address::MacAddress;
use uuid::Uuid;

use crate::server::{interfaces::r#impl::base::Interface, ip_addresses::r#impl::base::IPAddress};

/// A known host a port's forwarding table learned, with the addresses and interfaces that carry
/// the learned MAC.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearnedHost {
    pub host_id: Uuid,
    /// The host's addresses whose MAC is the one learned, or, when the MAC is on one of the host's
    /// interfaces, the address that interface holds. Empty when that interface holds none.
    pub ip_address_ids: Vec<Uuid>,
    /// The host's interfaces the device is reached on: the one carrying the learned MAC, or the
    /// one holding an address that carries it. Empty when the host has no such interface, and the
    /// device is known only as a whole.
    pub interface_ids: Vec<Uuid>,
}

/// For every port that learned at least one known host's MAC, those hosts.
///
/// A MAC resolves the way `LldpResolver::find_host_by_mac` resolves it: an address carrying it
/// first, then an interface. A MAC carried by more than one host names none of them, and a port's
/// own host is never a device behind it (a switch learns its own MAC on some ports).
pub fn learned_hosts<'a>(
    interfaces: impl IntoIterator<Item = &'a Interface> + Clone,
    ip_addresses: impl IntoIterator<Item = &'a IPAddress>,
) -> HashMap<Uuid, Vec<LearnedHost>> {
    let mut by_ip_mac: HashMap<MacAddress, (HashSet<Uuid>, Vec<Uuid>)> = HashMap::new();
    for ip in ip_addresses {
        if let Some(mac) = &ip.base.mac_address {
            let entry = by_ip_mac.entry(mac.value().0).or_default();
            entry.0.insert(ip.base.host_id);
            entry.1.push(ip.id);
        }
    }
    let mut by_interface_mac: HashMap<MacAddress, (HashSet<Uuid>, Vec<Uuid>, Vec<Uuid>)> =
        HashMap::new();
    let mut interfaces_of_ip: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
    for interface in interfaces.clone() {
        if let Some(mac) = &interface.base.mac_address {
            let entry = by_interface_mac.entry(mac.value().0).or_default();
            entry.0.insert(interface.base.host_id);
            entry.1.extend(interface.base.ip_address_id);
            entry.2.push(interface.id);
        }
        if let Some(ip_id) = interface.base.ip_address_id {
            interfaces_of_ip
                .entry(ip_id)
                .or_default()
                .push(interface.id);
        }
    }
    // The one host a set names, or `None` when it names several.
    let only = |hosts: &HashSet<Uuid>| {
        let mut iter = hosts.iter();
        match (iter.next(), iter.next()) {
            (Some(host), None) => Some(*host),
            _ => None,
        }
    };
    let resolve = |mac: &MacAddress| -> Option<LearnedHost> {
        if let Some((hosts, ip_ids)) = by_ip_mac.get(mac) {
            return Some(LearnedHost {
                host_id: only(hosts)?,
                ip_address_ids: ip_ids.clone(),
                interface_ids: ip_ids
                    .iter()
                    .flat_map(|ip| interfaces_of_ip.get(ip).into_iter().flatten().copied())
                    .collect(),
            });
        }
        let (hosts, ip_ids, interface_ids) = by_interface_mac.get(mac)?;
        Some(LearnedHost {
            host_id: only(hosts)?,
            ip_address_ids: ip_ids.clone(),
            interface_ids: interface_ids.clone(),
        })
    };

    let mut learned = HashMap::new();
    for interface in interfaces {
        let Some(macs) = interface.base.fdb_macs() else {
            continue;
        };
        let mut hosts: Vec<LearnedHost> = Vec::new();
        for raw in macs {
            let Ok(mac) = raw.parse::<MacAddress>() else {
                continue;
            };
            let Some(host) = resolve(&mac) else { continue };
            if host.host_id == interface.base.host_id {
                continue;
            }
            match hosts.iter_mut().find(|h| h.host_id == host.host_id) {
                Some(existing) => {
                    existing.ip_address_ids.extend(host.ip_address_ids);
                    existing.interface_ids.extend(host.interface_ids);
                }
                None => hosts.push(host),
            }
        }
        if !hosts.is_empty() {
            learned.insert(interface.id, hosts);
        }
    }
    learned
}

/// The interfaces some port's forwarding table reached: the far ends of the ports in `learned`.
pub fn learned_interfaces(learned: &HashMap<Uuid, Vec<LearnedHost>>) -> HashSet<Uuid> {
    learned
        .values()
        .flatten()
        .flat_map(|host| host.interface_ids.iter().copied())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::interfaces::r#impl::base::{FdbMacs, FdbMacsValue, InterfaceBase};
    use crate::server::ip_addresses::r#impl::base::{IPAddressBase, MacEvidence, MacEvidenceValue};
    use crate::server::shared::attribution::AttributeSource;

    const SWITCH: Uuid = Uuid::from_u128(1);
    const PLC: Uuid = Uuid::from_u128(2);
    const PLC_MAC: &str = "00:80:63:12:ab:cd";

    fn mac(raw: &str) -> MacEvidence {
        MacEvidence::new(
            MacEvidenceValue(raw.parse().unwrap()),
            AttributeSource::ArpReply,
        )
    }

    fn port(host_id: Uuid, learned: &[&str]) -> Interface {
        Interface::new(InterfaceBase {
            host_id,
            fdb_macs: Some(FdbMacs::new(
                FdbMacsValue(learned.iter().map(|m| m.to_string()).collect()),
                AttributeSource::ForwardingTable,
            )),
            ..Default::default()
        })
    }

    fn address(host_id: Uuid, raw_mac: &str) -> IPAddress {
        IPAddress::new(IPAddressBase {
            host_id,
            mac_address: Some(mac(raw_mac)),
            ..Default::default()
        })
    }

    #[test]
    fn a_port_learns_the_host_and_address_holding_the_mac() {
        let access = port(SWITCH, &[PLC_MAC]);
        let plc_ip = address(PLC, PLC_MAC);
        let learned = learned_hosts(std::slice::from_ref(&access), std::slice::from_ref(&plc_ip));
        assert_eq!(
            learned.get(&access.id),
            Some(&vec![LearnedHost {
                host_id: PLC,
                ip_address_ids: vec![plc_ip.id],
                interface_ids: vec![],
            }])
        );
    }

    #[test]
    fn a_mac_two_hosts_carry_names_neither_and_a_switchs_own_mac_is_not_behind_it() {
        let shared = "00:80:63:00:00:09";
        let own = "00:80:63:00:00:01";
        let access = port(SWITCH, &[shared, own]);
        let addresses = [
            address(PLC, shared),
            address(Uuid::from_u128(3), shared),
            address(SWITCH, own),
        ];
        assert!(learned_hosts(&[access], &addresses).is_empty());
    }

    #[test]
    fn a_mac_on_an_interface_names_its_host_when_no_address_carries_it() {
        let access = port(SWITCH, &[PLC_MAC]);
        let plc_port = Interface::new(InterfaceBase {
            host_id: PLC,
            mac_address: Some(mac(PLC_MAC)),
            ..Default::default()
        });
        let learned = learned_hosts(&[access.clone(), plc_port], &[]);
        assert_eq!(learned.get(&access.id).map(|h| h[0].host_id), Some(PLC));
    }
}
