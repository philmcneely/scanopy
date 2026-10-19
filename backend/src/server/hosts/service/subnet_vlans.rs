//! Subnet↔VLAN evidence from a switch's forwarding database.
//!
//! The direct evidence is one interface carrying both an address on the subnet and a native VLAN.
//! A firewall's routed interfaces carry the address and no VLAN, and a switch's access ports carry
//! the VLAN and no address, so on the commonest small network the two never meet on one row. The
//! end hosts join them: a host's address puts its MAC on the subnet, and the access port that
//! learned that MAC puts it in the port's VLAN.

use std::collections::{HashMap, HashSet};

use mac_address::MacAddress;
use uuid::Uuid;

use super::*;
use crate::server::shared::storage::filter::StorableFilter;

/// The VLANs `subnet_id` is on, by way of the access ports that learned its hosts' MACs.
///
/// A link needs every step to hold, because a wrong link is worse than a missing one:
///
/// - The MAC has addresses on this subnet and on no other. A router or firewall answering for
///   several subnets from one MAC sits in whichever VLAN its uplink is in, and that says nothing
///   about any one of its subnets.
/// - The port that learned it has a native VLAN and no tagged ones. A trunk carries every VLAN it
///   is tagged with, so a MAC learned there could be in any of them.
///
/// `subnets_by_mac` is every subnet each MAC has a live address on, site-wide.
pub(crate) fn access_port_vlans(
    subnet_id: Uuid,
    subnet_ip_addresses: &[IPAddress],
    subnets_by_mac: &HashMap<MacAddress, HashSet<Uuid>>,
    learning_ports: &[Interface],
) -> Vec<Uuid> {
    let only_here: HashSet<MacAddress> = subnet_ip_addresses
        .iter()
        .filter_map(|ip| mac_of(&ip.base.mac_address))
        .filter(|mac| {
            subnets_by_mac
                .get(mac)
                .is_some_and(|subnets| subnets.len() == 1 && subnets.contains(&subnet_id))
        })
        .collect();

    let mut vlans: Vec<Uuid> = learning_ports
        .iter()
        .filter(|port| port.base.vlan_ids.as_ref().is_none_or(Vec::is_empty))
        .filter_map(|port| {
            let learned_here = port.base.fdb_macs()?.iter().any(|learned| {
                learned
                    .parse::<MacAddress>()
                    .is_ok_and(|mac| only_here.contains(&mac))
            });
            learned_here.then_some(port.base.native_vlan_id).flatten()
        })
        .collect();
    vlans.sort();
    vlans.dedup();
    vlans
}

impl HostService {
    /// The subnets whose hosts' MACs `fdb_macs` names, site-wide. A switch's scan changes what its
    /// ports learned, so these are the subnets whose access-port evidence it may have changed.
    pub(crate) async fn subnets_learned_by(
        &self,
        site_id: Uuid,
        fdb_macs: &HashSet<MacAddress>,
    ) -> Result<HashSet<Uuid>> {
        if fdb_macs.is_empty() {
            return Ok(HashSet::new());
        }
        let macs: Vec<MacAddress> = fdb_macs.iter().copied().collect();
        let filter = StorableFilter::<IPAddress>::new_from_site_ids(&[site_id])
            .mac_address_in(&macs)
            .live();
        Ok(self
            .ip_address_service
            .get_all(filter)
            .await?
            .iter()
            .map(|ip| ip.base.subnet_id)
            .collect())
    }

    /// [`access_port_vlans`] for one subnet, with what it needs loaded.
    pub(crate) async fn access_port_vlans_for_subnet(
        &self,
        subnet_id: Uuid,
        subnet_ip_addresses: &[IPAddress],
    ) -> Result<Vec<Uuid>> {
        let Some(site_id) = subnet_ip_addresses.first().map(|ip| ip.base.site_id) else {
            return Ok(Vec::new());
        };
        let macs: Vec<MacAddress> = subnet_ip_addresses
            .iter()
            .filter_map(|ip| mac_of(&ip.base.mac_address))
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        if macs.is_empty() {
            return Ok(Vec::new());
        }

        let mut subnets_by_mac: HashMap<MacAddress, HashSet<Uuid>> = HashMap::new();
        let filter = StorableFilter::<IPAddress>::new_from_site_ids(&[site_id])
            .mac_address_in(&macs)
            .live();
        for ip in self.ip_address_service.get_all(filter).await? {
            if let Some(mac) = mac_of(&ip.base.mac_address) {
                subnets_by_mac
                    .entry(mac)
                    .or_default()
                    .insert(ip.base.subnet_id);
            }
        }

        let filter = StorableFilter::<Interface>::new_from_site_ids(&[site_id])
            .learned_any_mac(&macs)
            .live();
        let learning_ports = self.interface_service.get_all(filter).await?;

        Ok(access_port_vlans(
            subnet_id,
            subnet_ip_addresses,
            &subnets_by_mac,
            &learning_ports,
        ))
    }
}

/// The MACs these interfaces' forwarding databases learned.
pub(crate) fn learned_macs(interfaces: &[Interface]) -> HashSet<MacAddress> {
    interfaces
        .iter()
        .filter_map(|i| i.base.fdb_macs())
        .flatten()
        .filter_map(|mac| mac.parse().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::interfaces::r#impl::base::{FdbMacs, FdbMacsValue, InterfaceBase};
    use crate::server::ip_addresses::r#impl::base::{IPAddressBase, MacEvidence, MacEvidenceValue};
    use crate::server::services::r#impl::patterns::ClientProbe;
    use crate::server::shared::attribution::AttributeSource;

    const SNMP: AttributeSource = AttributeSource::Probe(ClientProbe::Snmp);

    fn host_ip(subnet_id: Uuid, mac: &str) -> IPAddress {
        IPAddress::new(IPAddressBase {
            subnet_id,
            mac_address: Some(MacEvidence::new(
                MacEvidenceValue(mac.parse().unwrap()),
                SNMP,
            )),
            ..IPAddressBase::default()
        })
    }

    fn port(native: Option<Uuid>, tagged: Option<Vec<Uuid>>, learned: &[&str]) -> Interface {
        Interface::new(InterfaceBase {
            native_vlan_id: native,
            vlan_ids: tagged,
            fdb_macs: Some(FdbMacs::new(
                FdbMacsValue(learned.iter().map(|m| m.to_string()).collect()),
                SNMP,
            )),
            ..InterfaceBase::default()
        })
    }

    fn subnets_by_mac(ips: &[IPAddress]) -> HashMap<MacAddress, HashSet<Uuid>> {
        let mut map: HashMap<MacAddress, HashSet<Uuid>> = HashMap::new();
        for ip in ips {
            map.entry(mac_of(&ip.base.mac_address).unwrap())
                .or_default()
                .insert(ip.base.subnet_id);
        }
        map
    }

    const HOST: &str = "00:1a:2b:00:60:01";

    /// A host on the subnet, learned on an access port: the subnet is on that port's VLAN.
    #[test]
    fn an_access_port_that_learned_a_host_links_its_subnet_to_the_port_vlan() {
        let (subnet, vlan) = (Uuid::new_v4(), Uuid::new_v4());
        let ips = [host_ip(subnet, HOST)];

        let vlans = access_port_vlans(
            subnet,
            &ips,
            &subnets_by_mac(&ips),
            &[port(Some(vlan), None, &[HOST])],
        );

        assert_eq!(vlans, vec![vlan]);
    }

    /// The forwarding database writes MACs however its source does; a lower-case entry is the
    /// same host.
    #[test]
    fn the_learned_mac_matches_whatever_its_case() {
        let (subnet, vlan) = (Uuid::new_v4(), Uuid::new_v4());
        let ips = [host_ip(subnet, HOST)];

        let vlans = access_port_vlans(
            subnet,
            &ips,
            &subnets_by_mac(&ips),
            &[port(Some(vlan), Some(Vec::new()), &[&HOST.to_uppercase()])],
        );

        assert_eq!(vlans, vec![vlan]);
    }

    /// A trunk carries every VLAN it is tagged with, so the MAC could be in any of them.
    #[test]
    fn a_trunk_port_is_no_evidence() {
        let (subnet, native, tagged) = (Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4());
        let ips = [host_ip(subnet, HOST)];

        let vlans = access_port_vlans(
            subnet,
            &ips,
            &subnets_by_mac(&ips),
            &[port(Some(native), Some(vec![tagged]), &[HOST])],
        );

        assert!(vlans.is_empty());
    }

    /// A router's one MAC answering for two subnets sits in its uplink's VLAN, which says nothing
    /// about either subnet.
    #[test]
    fn a_mac_with_addresses_on_two_subnets_is_no_evidence() {
        let (here, elsewhere, vlan) = (Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4());
        let ips = [host_ip(here, HOST), host_ip(elsewhere, HOST)];

        let vlans = access_port_vlans(
            here,
            &ips[..1],
            &subnets_by_mac(&ips),
            &[port(Some(vlan), None, &[HOST])],
        );

        assert!(vlans.is_empty());
    }
}
