//! The controller's networks (`rest/networkconf`) as the VLANs and subnets a site holds.
//!
//! Pure and synchronous, like [`super::mapping`]. `execute` records what [`site_networks`]
//! returns and hands the result back to [`gateway_interfaces`] and [`super::ports`].
//!
//! Which networks count:
//!
//! - `corporate` and `guest` are LAN segments the gateway routes: each is a VLAN and a subnet.
//! - `vlan-only` is a VLAN the switches carry with no gateway address, so a VLAN and no subnet.
//! - `wan` is the uplink to the ISP. Its `wan_vlan` is the provider's tag, not a VLAN on this
//!   site, and its address comes from the provider.
//! - `remote-user-vpn`, `site-vpn` and `vpn-client` are tunnel address pools. They are not VLANs,
//!   and `stat/sta` does not report VPN clients, so a subnet for one would hold nothing.
//! - A network with `enabled: false` is not running.

use std::net::IpAddr;

use cidr::IpCidr;
use pnet::ipnetwork::IpNetwork;
use uuid::Uuid;

use crate::server::interfaces::r#impl::base::{Interface, InterfaceBase, if_type};
use crate::server::ip_addresses::r#impl::base::{IPAddress, IPAddressBase};
use crate::server::services::r#impl::patterns::ClientProbe;
use crate::server::shared::attribution::AttributeSource;
use crate::server::shared::storage::traits::Storable;
use crate::server::shared::types::entities::EntitySource;
use crate::server::subnets::r#impl::base::{
    Subnet, SubnetBase, SubnetCidr, SubnetCidrValue, SubnetName, SubnetNameValue,
    SubnetTypeAttributed, SubnetTypeValue,
};
use crate::server::subnets::r#impl::types::SubnetType;

use super::types::UnifiNetwork;

/// The controller reporting how it is configured.
const UNIFI: AttributeSource = AttributeSource::Probe(ClientProbe::UnifiController);

/// 802.1Q's default VLAN, which a UniFi network without `vlan_enabled` carries untagged.
const UNTAGGED_VLAN: u16 = 1;

/// A network the controller defines that is a VLAN on this site.
#[derive(Debug, Clone)]
pub struct SiteNetwork {
    /// The networkconf `_id`, which port profiles reference.
    pub id: Option<String>,
    pub name: String,
    pub vlan_number: u16,
    /// The gateway's address on the network and the range it routes. `None` for `vlan-only`.
    pub range: Option<GatewayRange>,
    pub subnet_type: SubnetType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayRange {
    pub gateway: IpAddr,
    pub cidr: IpCidr,
}

impl SiteNetwork {
    /// The subnet to create for this network when the site holds nothing that covers it. Named as
    /// the controller names the network; `SubnetService::create` keeps the name of a subnet the
    /// site already holds at the same range.
    pub fn to_subnet(&self, site_id: Uuid) -> Option<Subnet> {
        let range = self.range.as_ref()?;
        // An administrator named the network and chose its purpose in the controller.
        let authored = AttributeSource::Authored(ClientProbe::UnifiController);
        Some(Subnet::new(SubnetBase {
            cidr: SubnetCidr::new(SubnetCidrValue(range.cidr), UNIFI),
            site_id,
            name: SubnetName::new(SubnetNameValue(self.name.clone()), authored),
            description: None,
            subnet_type: SubnetTypeAttributed::new(SubnetTypeValue(self.subnet_type), authored),
            virtualization_service_id: None,
            source: EntitySource::Discovery,
            vlan_ids: Vec::new(),
            tags: Vec::new(),
        }))
    }
}

/// The networks in a `rest/networkconf` payload that are VLANs on this site, in payload order.
pub fn site_networks(networks: &[UnifiNetwork]) -> Vec<SiteNetwork> {
    networks.iter().filter_map(site_network).collect()
}

fn site_network(network: &UnifiNetwork) -> Option<SiteNetwork> {
    if network.enabled.is_some_and(|e| !e.as_bool()) {
        return None;
    }
    let (subnet_type, routed) = match network.purpose.as_deref()? {
        "corporate" => (SubnetType::Lan, true),
        "guest" => (SubnetType::Guest, true),
        "vlan-only" => (SubnetType::Lan, false),
        _ => return None,
    };
    let vlan_number = vlan_number(network)?;
    let name = network
        .name
        .as_deref()
        .map(str::trim)
        .filter(|n| !n.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| format!("VLAN {vlan_number}"));
    let range = routed
        .then(|| network.ip_subnet.as_deref().and_then(gateway_range))
        .flatten();

    Some(SiteNetwork {
        id: network.id.clone(),
        name,
        vlan_number,
        range,
        subnet_type,
    })
}

/// The 802.1Q number a network carries. A network not tagged (`vlan_enabled: false`, or neither
/// field present, as on the captured default network) rides the untagged VLAN 1. One marked
/// tagged with no usable number is not recorded as a VLAN at all.
fn vlan_number(network: &UnifiNetwork) -> Option<u16> {
    let tagged = network.vlan_enabled.map(|v| v.as_bool());
    let number = network
        .vlan
        .map(|v| v.as_i64())
        .and_then(|v| u16::try_from(v).ok())
        .filter(|v| (1..=4094).contains(v));
    match (tagged, number) {
        (Some(false), _) | (None, None) => Some(UNTAGGED_VLAN),
        (_, Some(number)) => Some(number),
        (Some(true), None) => None,
    }
}

/// `"10.20.0.1/24"` → the gateway `10.20.0.1` on `10.20.0.0/24`. IPv4 only, matching
/// `Subnet::from_discovery`; a `/32` describes an address, not a segment.
fn gateway_range(ip_subnet: &str) -> Option<GatewayRange> {
    let IpNetwork::V4(network) = ip_subnet.trim().parse::<IpNetwork>().ok()? else {
        return None;
    };
    if network.prefix() == 32 {
        return None;
    }
    let cidr = IpCidr::V4(cidr::Ipv4Cidr::new(network.network(), network.prefix()).ok()?);
    Some(GatewayRange {
        gateway: IpAddr::V4(network.ip()),
        cidr,
    })
}

/// A network after recording: the VLAN's id from the server, and the subnet the site holds for its
/// range.
#[derive(Debug, Clone)]
pub struct RecordedNetwork {
    pub network: SiteNetwork,
    pub vlan_id: Option<Uuid>,
    pub subnet: Option<Subnet>,
}

/// The gateway's address on each routed network, and one interface per network that carries the
/// address on its VLAN.
///
/// This is the evidence the server links a subnet to its VLAN from
/// (`reconcile_subnet_vlans_for_host`): the native VLAN of an interface bound to an address on the
/// subnet. An SNMP walk of a router contributes the same pair through its VLAN interfaces.
///
/// `management` is the gateway's own address and its row. A network whose gateway address is the
/// management address reuses that row instead of submitting the address twice.
pub fn gateway_interfaces(
    recorded: &[RecordedNetwork],
    site_id: Uuid,
    management: (IpAddr, Uuid),
) -> (Vec<IPAddress>, Vec<Interface>) {
    let mut addresses = Vec::new();
    let mut interfaces = Vec::new();

    for entry in recorded {
        let (Some(range), Some(subnet), Some(vlan_id)) =
            (&entry.network.range, &entry.subnet, entry.vlan_id)
        else {
            continue;
        };
        // The range the site holds may be wider than the network's own (a hand-made /16 over the
        // controller's /24), but it must hold the gateway for the address to belong on it.
        if !subnet.base.cidr.contains(&range.gateway) {
            continue;
        }

        let ip_address_id = if range.gateway == management.0 {
            management.1
        } else {
            let address = IPAddress::new(IPAddressBase {
                site_id,
                host_id: Uuid::nil(), // server assigns
                subnet_id: subnet.id,
                ip_address: range.gateway,
                mac_address: None,
                name: None,
                position: 0,
                ip_address_source: UNIFI,
            });
            let id = address.id;
            addresses.push(address);
            id
        };

        interfaces.push(Interface::new(InterfaceBase {
            host_id: Uuid::nil(),
            site_id,
            if_descr: Some(entry.network.name.clone()),
            if_name: Some(entry.network.name.clone()),
            // Virtual, so the L2 view does not draw it as a port and it does not count towards
            // the chassis MAC's uniqueness.
            if_type: Some(if_type::PROP_VIRTUAL),
            ip_address_id: Some(ip_address_id),
            native_vlan_id: Some(vlan_id),
            vlan_membership_source: UNIFI,
            interface_source: UNIFI,
            ..Default::default()
        }));
    }

    (addresses, interfaces)
}

/// Whether the controller's device class is a gateway: the USG (`ugw`), a Dream Machine or Cloud
/// Gateway (`udm`), or a Next-Gen Gateway (`uxg`). unpoller: `USG.Type`, `UDM.Type`, `UXG.Type`.
pub fn is_gateway(device_type: Option<&str>) -> bool {
    matches!(device_type, Some("ugw" | "udm" | "uxg"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::daemon::discovery::integration::unifi::types::UnifiEnvelope;

    /// Captured from a Network Application 10.6.106 container configured with one network of
    /// each purpose; see `tools/unifi/UNIFI-TEST-ENV.md`. Identifiers are the container's own.
    const NETWORKCONF: &str =
        include_str!("../../../../tests/unifi/networkconf_network_app_10_6.json");

    fn networks() -> Vec<SiteNetwork> {
        let envelope: UnifiEnvelope<UnifiNetwork> =
            serde_json::from_str(NETWORKCONF).expect("fixture parses");
        assert!(envelope.meta.is_ok());
        site_networks(&envelope.data)
    }

    fn find<'a>(networks: &'a [SiteNetwork], name: &str) -> &'a SiteNetwork {
        networks
            .iter()
            .find(|n| n.name == name)
            .unwrap_or_else(|| panic!("{name} is a site network"))
    }

    /// The capture holds one network of every purpose plus a disabled one. Only the LAN segments
    /// and the VLAN-only network are VLANs on the site; the WAN, the VPN pool and the disabled
    /// network are not.
    #[test]
    fn only_running_lan_and_vlan_only_networks_are_site_vlans() {
        let names: Vec<String> = networks().into_iter().map(|n| n.name).collect();
        assert_eq!(
            names,
            vec!["Default", "Servers", "IoT", "Guest WiFi", "Cameras"]
        );
    }

    /// The untagged default network rides VLAN 1; the others carry their configured tag.
    #[test]
    fn the_untagged_default_network_is_vlan_1() {
        let networks = networks();
        assert_eq!(find(&networks, "Default").vlan_number, 1);
        assert_eq!(find(&networks, "Servers").vlan_number, 20);
    }

    /// `ip_subnet` holds the gateway's address, so the subnet is its network address at the same
    /// prefix, and the gateway is kept to place the router on it.
    #[test]
    fn a_routed_network_yields_its_range_and_gateway() {
        let networks = networks();
        let iot = find(&networks, "IoT").range.clone().expect("IoT is routed");
        assert_eq!(iot.cidr.to_string(), "10.30.0.0/23");
        assert_eq!(iot.gateway.to_string(), "10.30.0.1");
        assert!(find(&networks, "Cameras").range.is_none());
    }

    #[test]
    fn a_guest_network_becomes_a_guest_subnet() {
        let networks = networks();
        let subnet = find(&networks, "Guest WiFi")
            .to_subnet(Uuid::nil())
            .expect("routed");
        assert_eq!(subnet.base.subnet_type(), SubnetType::Guest);
        assert_eq!(subnet.base.name(), "Guest WiFi");
    }

    fn recorded(name: &str, cidr: &str, gateway: &str) -> RecordedNetwork {
        let range = GatewayRange {
            gateway: gateway.parse().unwrap(),
            cidr: cidr.parse().unwrap(),
        };
        let network = SiteNetwork {
            id: None,
            name: name.to_string(),
            vlan_number: 20,
            range: Some(range),
            subnet_type: SubnetType::Lan,
        };
        let subnet = network.to_subnet(Uuid::nil());
        RecordedNetwork {
            network,
            vlan_id: Some(Uuid::new_v4()),
            subnet,
        }
    }

    /// Each interface is bound to the address it carries, so the server can pair them without
    /// MACs: every one of a gateway's network interfaces shares its chassis MAC.
    #[test]
    fn each_gateway_interface_is_bound_to_its_own_address() {
        let networks = [
            recorded("Servers", "10.20.0.0/24", "10.20.0.1"),
            recorded("IoT", "10.30.0.0/24", "10.30.0.1"),
        ];
        let (addresses, interfaces) = gateway_interfaces(
            &networks,
            Uuid::nil(),
            ("192.168.1.1".parse().unwrap(), Uuid::new_v4()),
        );

        for (entry, interface) in networks.iter().zip(&interfaces) {
            let address = addresses
                .iter()
                .find(|a| Some(a.id) == interface.base.ip_address_id)
                .expect("bound to a submitted address");
            assert_eq!(
                Some(address.base.subnet_id),
                entry.subnet.as_ref().map(|s| s.id)
            );
            assert_eq!(interface.base.native_vlan_id, entry.vlan_id);
        }
    }

    /// A Dream Machine managed at its Default-network address already submits that address; the
    /// Default network's interface binds to it instead of adding a duplicate.
    #[test]
    fn the_management_address_is_not_submitted_twice() {
        let management_id = Uuid::new_v4();
        let (addresses, interfaces) = gateway_interfaces(
            &[recorded("Default", "192.168.1.0/24", "192.168.1.1")],
            Uuid::nil(),
            ("192.168.1.1".parse().unwrap(), management_id),
        );
        assert!(addresses.is_empty());
        assert_eq!(interfaces[0].base.ip_address_id, Some(management_id));
    }
}
