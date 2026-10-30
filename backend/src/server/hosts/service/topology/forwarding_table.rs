//! Unconfirmed hosts for MACs a switch learned on an access port that no host on the site holds.
//!
//! A device that answers nothing (an industrial switch, a printer behind a filter) is still seen
//! by the switch it is plugged into. Each such MAC becomes a host with
//! `source = Inferred { via: ForwardingTable }`: listed, drawn on its port and labelable, and
//! unconfirmed until a person confirms it or a scan reaches it.

use mac_address::MacAddress;

use super::*;
use crate::server::hosts::service::mac_identity::{self, MacQuality};
use crate::server::interfaces::r#impl::base::InterfaceBase;
use crate::server::ip_addresses::r#impl::base::{MacEvidence, MacEvidenceValue};

/// One switch port's forwarding table, as far as minting is concerned.
pub(super) struct LearnedPort<'a> {
    pub interface_id: Uuid,
    pub macs: &'a [String],
    /// No LLDP/CDP evidence and no tagged VLANs: the port faces a device rather than another
    /// switch. A switch-to-switch link learns every MAC behind the far switch, so minting from it
    /// would put each of those devices on the wrong port.
    pub is_edge: bool,
}

/// The MACs to mint a host for, each with the port it was learned on.
///
/// A MAC qualifies when it is learned on an edge port, its value can anchor a host
/// ([`MacQuality::Strong`]), and no live row on the site carries it (`known`, which includes the
/// MACs of dismissed hosts, so a dismissal holds). A MAC on several edge ports is placed on the one
/// that learned the fewest MACs, the port closest to the device.
pub(super) fn mint_candidates(
    ports: &[LearnedPort<'_>],
    known: &HashSet<MacAddress>,
) -> Vec<(MacAddress, Uuid)> {
    let mut best: HashMap<MacAddress, (usize, Uuid)> = HashMap::new();
    for port in ports.iter().filter(|p| p.is_edge) {
        for raw in port.macs {
            let Ok(mac) = raw.parse::<MacAddress>() else {
                continue;
            };
            if known.contains(&mac) || mac_identity::classify(&mac) != MacQuality::Strong {
                continue;
            }
            let entry = best
                .entry(mac)
                .or_insert((port.macs.len(), port.interface_id));
            if port.macs.len() < entry.0 {
                *entry = (port.macs.len(), port.interface_id);
            }
        }
    }
    let mut candidates: Vec<(MacAddress, Uuid)> = best
        .into_iter()
        .map(|(mac, (_, interface_id))| (mac, interface_id))
        .collect();
    candidates.sort_by_key(|(mac, _)| mac.bytes());
    candidates
}

impl HostService {
    /// Mint an unconfirmed host for every MAC [`mint_candidates`] selects on this site.
    ///
    /// Runs before `resolve_fdb_links`, which then places each new host on its port the way it
    /// places any host. A host that cannot be created is one missing row, logged, not a failed
    /// pass.
    pub(super) async fn mint_forwarding_table_hosts(
        &self,
        site_id: Uuid,
        scan_time: DateTime<Utc>,
    ) -> Result<u32> {
        let interfaces = self
            .interface_service
            .get_all(StorableFilter::<Interface>::new_from_site_ids(&[site_id]).live())
            .await?;
        if !interfaces
            .iter()
            .any(|i| i.base.fdb_macs().is_some_and(|m| !m.is_empty()))
        {
            return Ok(0);
        }

        let with_neighbour_evidence: HashSet<Uuid> = self
            .interface_neighbor_service
            .candidates_for_site(site_id)
            .await?
            .into_iter()
            .map(|c| c.base.interface_id)
            .collect();

        // Every MAC a live row on the site already carries, on an address or an interface. A
        // dismissed host's rows stay live, which is what stops its MAC being minted again.
        let mut known: HashSet<MacAddress> = interfaces
            .iter()
            .filter_map(|i| i.base.mac_address.as_ref().map(|m| m.value().0))
            .collect();
        known.extend(
            self.ip_address_service
                .get_all(StorableFilter::<IPAddress>::new_from_site_ids(&[site_id]).live())
                .await?
                .into_iter()
                .filter_map(|ip| ip.base.mac_address.map(|m| m.value().0)),
        );

        let ports: Vec<LearnedPort<'_>> = interfaces
            .iter()
            .filter_map(|i| {
                let macs = i.base.fdb_macs()?;
                Some(LearnedPort {
                    interface_id: i.id,
                    macs,
                    is_edge: !with_neighbour_evidence.contains(&i.id)
                        && i.base.vlan_ids.as_ref().is_none_or(|v| v.is_empty()),
                })
            })
            .collect();

        let mut minted = 0;
        for (mac, interface_id) in mint_candidates(&ports, &known) {
            let mut host = Host::new(HostBase {
                site_id,
                source: EntitySource::Inferred {
                    via: InferredVia::ForwardingTable,
                },
                ..Default::default()
            });
            host.last_seen_at = scan_time;
            // The MAC is the device's only identity, so it rides on an interface: the tier
            // `select_matching_host_by_mac` matches when a scan later reaches the device.
            let interfaces = vec![Interface::new(InterfaceBase {
                site_id,
                host_id: Uuid::nil(), // Server assigns.
                if_descr: Some(mac.to_string()),
                mac_address: Some(MacEvidence::new(
                    MacEvidenceValue(mac),
                    AttributeSource::ForwardingTable,
                )),
                interface_source: AttributeSource::ForwardingTable,
                ..Default::default()
            })];

            match self
                .create_with_children(
                    host,
                    vec![],
                    vec![],
                    vec![],
                    interfaces,
                    vec![],
                    ConflictBehavior::Upsert,
                    AuthenticatedEntity::System,
                    None,
                    // A switch's table says nothing about the device's own ifTable.
                    false,
                    InterfaceDataComplete::none(),
                    &crate::server::ports::r#impl::coverage::PortCoverage::default(),
                )
                .await
            {
                Ok(_) => minted += 1,
                Err(e) => tracing::warn!(
                    site_id = %site_id,
                    %mac,
                    %interface_id,
                    error = %e,
                    "Could not mint a host for a MAC learned in a forwarding table"
                ),
            }
        }

        if minted > 0 {
            tracing::debug!(site_id = %site_id, minted, "Minted forwarding-table hosts");
        }
        Ok(minted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEVICE: &str = "00:80:63:12:ab:cd"; // Hirschmann OUI

    fn port(macs: &[String], is_edge: bool) -> (Uuid, Vec<String>, bool) {
        (Uuid::new_v4(), macs.to_vec(), is_edge)
    }

    fn select(
        ports: &[(Uuid, Vec<String>, bool)],
        known: &HashSet<MacAddress>,
    ) -> Vec<(MacAddress, Uuid)> {
        let learned: Vec<LearnedPort<'_>> = ports
            .iter()
            .map(|(id, macs, is_edge)| LearnedPort {
                interface_id: *id,
                macs,
                is_edge: *is_edge,
            })
            .collect();
        mint_candidates(&learned, known)
    }

    #[test]
    fn an_unknown_burned_in_mac_on_an_access_port_is_minted_on_that_port() {
        let access = port(&[DEVICE.to_string()], true);
        let picked = select(std::slice::from_ref(&access), &HashSet::new());
        assert_eq!(picked, vec![(DEVICE.parse().unwrap(), access.0)]);
    }

    #[test]
    fn a_mac_any_live_row_carries_is_not_minted_again() {
        let known = HashSet::from([DEVICE.parse::<MacAddress>().unwrap()]);
        assert!(select(&[port(&[DEVICE.to_string()], true)], &known).is_empty());
    }

    #[test]
    fn macs_learned_only_on_uplinks_and_trunks_are_not_minted() {
        assert!(select(&[port(&[DEVICE.to_string()], false)], &HashSet::new()).is_empty());
    }

    #[test]
    fn rotating_shared_and_group_macs_are_not_minted() {
        let unanchorable = [
            "aa:bb:cc:dd:ee:03", // locally administered
            "00:00:5e:00:01:0a", // VRRP
            "01:00:5e:00:00:01", // multicast
        ]
        .map(String::from);
        assert!(select(&[port(&unanchorable, true)], &HashSet::new()).is_empty());
    }

    #[test]
    fn a_mac_on_several_access_ports_is_minted_once_on_the_closest() {
        let crowded = port(&[DEVICE.to_string(), "00:80:63:00:00:01".to_string()], true);
        let direct = port(&[DEVICE.to_string()], true);
        let picked = select(&[crowded, direct.clone()], &HashSet::new());
        let device: MacAddress = DEVICE.parse().unwrap();
        assert_eq!(
            picked
                .iter()
                .filter(|(mac, _)| *mac == device)
                .collect::<Vec<_>>(),
            vec![&(device, direct.0)]
        );
    }
}
