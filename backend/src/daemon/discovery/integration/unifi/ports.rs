//! Each switch port's VLAN membership, from the controller's port configuration.
//!
//! The controller does not report membership as state. It reports what each port is configured to
//! carry, in three places, most specific first:
//!
//! 1. the device's `port_overrides` entry for the port, inline (`native_networkconf_id`,
//!    `tagged_vlan_mgmt`, …) or by naming a profile (`portconf_id`);
//! 2. the profile the port table says the port runs (`port_table[].portconf_id`);
//! 3. otherwise the built-in "All" profile: the default network untagged and every other VLAN
//!    tagged.
//!
//! A profile's tagged set is `tagged_vlan_mgmt`: `auto` tags every VLAN network, `block_all` none,
//! and `custom` every VLAN network except `excluded_networkconf_ids`. Controllers that predate the
//! field say the same with `forward` (`all`, `native`, `customize` with `tagged_networkconf_ids`,
//! `disabled`).
//!
//! Membership is all-or-nothing per device. A port naming a profile or network the controller did
//! not return is a configuration this mapping cannot read, and claiming the rest of the device as
//! complete would let the server clear VLAN data SNMP recorded for that port.

use std::collections::{HashMap, HashSet};

use uuid::Uuid;

use crate::server::interfaces::r#impl::base::Interface;
use crate::server::services::r#impl::patterns::ClientProbe;
use crate::server::shared::attribution::AttributeSource;

use super::networks::RecordedNetwork;
use super::types::{UnifiDevice, UnifiNetwork, UnifiPortConf, UnifiPortOverride};

/// The controller reporting how a port is configured.
const UNIFI: AttributeSource = AttributeSource::Probe(ClientProbe::UnifiController);

/// One port's VLANs as entity ids: the untagged one, and the tagged ones.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PortVlans {
    pub native: Option<Uuid>,
    pub tagged: Vec<Uuid>,
}

/// What the controller's configuration says about VLANs, indexed for resolving ports.
pub struct VlanConfig<'a> {
    profiles: HashMap<&'a str, &'a UnifiPortConf>,
    /// Every network id `rest/networkconf` returned, VLAN or not. A reference outside this set is
    /// one the controller did not explain.
    known_networks: HashSet<&'a str>,
    /// Networks that are VLANs on the site and were recorded, by networkconf id.
    vlans: HashMap<&'a str, Uuid>,
    /// The untagged network a port carries when nothing says otherwise.
    default_network: Option<&'a str>,
}

impl<'a> VlanConfig<'a> {
    pub fn new(
        networks: &'a [UnifiNetwork],
        recorded: &'a [RecordedNetwork],
        profiles: &'a [UnifiPortConf],
    ) -> Self {
        let vlans: HashMap<&str, Uuid> = recorded
            .iter()
            .filter_map(|r| Some((r.network.id.as_deref()?, r.vlan_id?)))
            .collect();
        let default_network = recorded
            .iter()
            .filter(|r| r.network.vlan_number == 1)
            .find_map(|r| r.network.id.as_deref());
        Self {
            profiles: profiles
                .iter()
                .filter_map(|p| Some((p.id.as_deref()?, p)))
                .collect(),
            known_networks: networks.iter().filter_map(|n| n.id.as_deref()).collect(),
            vlans,
            default_network,
        }
    }

    /// Each port's VLANs, by `port_idx`, or `None` when any port's configuration cannot be read in
    /// full. `None` also for a device with no ports: there is no membership to claim.
    pub fn device_ports(&self, device: &UnifiDevice) -> Option<HashMap<i32, PortVlans>> {
        if device.port_table.is_empty() {
            return None;
        }
        let overrides: HashMap<i32, &UnifiPortOverride> = device
            .port_overrides
            .iter()
            .filter_map(|o| Some((o.port_idx?.as_i32(), o)))
            .collect();

        device
            .port_table
            .iter()
            .map(|port| {
                let port_idx = port.port_idx?.as_i32();
                let setting = match overrides.get(&port_idx) {
                    Some(o)
                        if o.native_networkconf_id.is_some()
                            || o.tagged_vlan_mgmt.is_some()
                            || o.forward.is_some() =>
                    {
                        PortSetting::from_override(o)
                    }
                    Some(o) if o.portconf_id.is_some() => self.profile(o.portconf_id.as_deref())?,
                    _ => match port.portconf_id.as_deref() {
                        Some(id) => self.profile(Some(id))?,
                        None => PortSetting::default(),
                    },
                };
                Some((port_idx, self.resolve(&setting)?))
            })
            .collect()
    }

    fn profile(&self, id: Option<&str>) -> Option<PortSetting<'a>> {
        self.profiles
            .get(id?)
            .map(|profile| PortSetting::from_profile(profile))
    }

    fn resolve(&self, setting: &PortSetting<'_>) -> Option<PortVlans> {
        if setting.forward == Some("disabled") {
            return Some(PortVlans::default());
        }

        let native_network = match setting.native {
            // A profile that names no native network, such as one that only sets a voice VLAN,
            // carries none untagged.
            Some("") => None,
            Some(id) => Some(self.known(id)?),
            None => self.default_network,
        };

        let tagged_networks: Vec<&str> = match (setting.tagged_vlan_mgmt, setting.forward) {
            (Some("block_all"), _) | (None, Some("native")) => Vec::new(),
            (Some("auto"), _) | (None, Some("all") | None) => self.all_vlans_but(&[]),
            (Some("custom"), _) => self.all_vlans_but(setting.excluded),
            (None, Some("customize")) => setting
                .tagged
                .iter()
                .map(|id| self.known(id))
                .collect::<Option<_>>()?,
            // A value this mapping does not know the meaning of.
            _ => return None,
        };

        let native = native_network.and_then(|id| self.vlans.get(id).copied());
        let mut tagged: Vec<Uuid> = tagged_networks
            .into_iter()
            .filter(|id| Some(*id) != native_network)
            .filter_map(|id| self.vlans.get(id).copied())
            .collect();
        tagged.sort();
        tagged.dedup();
        Some(PortVlans { native, tagged })
    }

    /// A network id the controller returned. One that is not a VLAN on the site (a WAN, a disabled
    /// network) is known and simply carries no VLAN.
    fn known<'s>(&self, id: &'s str) -> Option<&'s str> {
        self.known_networks.contains(id).then_some(id)
    }

    fn all_vlans_but(&self, excluded: &[String]) -> Vec<&'a str> {
        self.vlans
            .keys()
            .copied()
            .filter(|id| !excluded.iter().any(|e| e == id))
            .collect()
    }
}

/// Write each port's VLANs onto the interface `port_to_interface` built for it (`if_index` is the
/// port's `port_idx`), in the same columns an SNMP walk fills.
pub fn apply(interfaces: &mut [Interface], ports: &HashMap<i32, PortVlans>) {
    for interface in interfaces {
        let Some(vlans) = interface.base.if_index.and_then(|i| ports.get(&i)) else {
            continue;
        };
        interface.base.native_vlan_id = vlans.native;
        // `None` rather than an empty list, as SNMP writes a port with no tagged VLANs.
        interface.base.vlan_ids = (!vlans.tagged.is_empty()).then(|| vlans.tagged.clone());
        interface.base.vlan_membership_source = UNIFI;
    }
}

/// A port's VLAN configuration from wherever it came from.
#[derive(Default)]
struct PortSetting<'a> {
    native: Option<&'a str>,
    tagged_vlan_mgmt: Option<&'a str>,
    excluded: &'a [String],
    tagged: &'a [String],
    forward: Option<&'a str>,
}

impl<'a> PortSetting<'a> {
    fn from_profile(profile: &'a UnifiPortConf) -> Self {
        Self {
            native: profile.native_networkconf_id.as_deref(),
            tagged_vlan_mgmt: profile.tagged_vlan_mgmt.as_deref(),
            excluded: profile
                .excluded_networkconf_ids
                .as_deref()
                .unwrap_or_default(),
            tagged: profile
                .tagged_networkconf_ids
                .as_deref()
                .unwrap_or_default(),
            forward: profile.forward.as_deref(),
        }
    }

    fn from_override(port: &'a UnifiPortOverride) -> Self {
        Self {
            native: port.native_networkconf_id.as_deref(),
            tagged_vlan_mgmt: port.tagged_vlan_mgmt.as_deref(),
            excluded: port.excluded_networkconf_ids.as_deref().unwrap_or_default(),
            tagged: &[],
            forward: port.forward.as_deref(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::daemon::discovery::integration::unifi::networks::site_networks;
    use crate::daemon::discovery::integration::unifi::types::UnifiEnvelope;

    /// Captured from the same Network Application 10.6.106 container as the networks: an IoT
    /// access profile (`block_all`), an AP trunk excluding the guest network (`custom`), and a
    /// camera uplink tagging everything (`auto`).
    const PORTCONF: &str = include_str!("../../../../tests/unifi/portconf_network_app_10_6.json");
    const NETWORKCONF: &str =
        include_str!("../../../../tests/unifi/networkconf_network_app_10_6.json");
    /// Hand-authored: a switch whose ports reference the captured profiles and networks. The
    /// container adopts no hardware, so `stat/device` from it is empty; field names come from
    /// go-unifi `DevicePortOverrides` and aiounifi `TypedDevicePortTable`.
    const SWITCH: &str = include_str!("../../../../tests/unifi/stat_device_port_vlans.json");

    struct Fixture {
        networks: Vec<UnifiNetwork>,
        recorded: Vec<RecordedNetwork>,
        profiles: Vec<UnifiPortConf>,
        devices: Vec<UnifiDevice>,
    }

    impl Fixture {
        fn load() -> Self {
            let networks: UnifiEnvelope<UnifiNetwork> = serde_json::from_str(NETWORKCONF).unwrap();
            let profiles: UnifiEnvelope<UnifiPortConf> = serde_json::from_str(PORTCONF).unwrap();
            let devices: UnifiEnvelope<UnifiDevice> = serde_json::from_str(SWITCH).unwrap();
            let recorded = site_networks(&networks.data)
                .into_iter()
                .map(|network| RecordedNetwork {
                    network,
                    vlan_id: Some(Uuid::new_v4()),
                    subnet: None,
                })
                .collect();
            Self {
                networks: networks.data,
                recorded,
                profiles: profiles.data,
                devices: devices.data,
            }
        }

        fn vlan(&self, name: &str) -> Uuid {
            self.recorded
                .iter()
                .find(|r| r.network.name == name)
                .and_then(|r| r.vlan_id)
                .unwrap()
        }

        fn vlans(&self, names: &[&str]) -> Vec<Uuid> {
            let mut ids: Vec<Uuid> = names.iter().map(|n| self.vlan(n)).collect();
            ids.sort();
            ids
        }

        fn ports(&self, device: usize) -> Option<HashMap<i32, PortVlans>> {
            VlanConfig::new(&self.networks, &self.recorded, &self.profiles)
                .device_ports(&self.devices[device])
        }
    }

    #[test]
    fn a_port_with_no_configuration_runs_the_all_profile() {
        let f = Fixture::load();
        let ports = f.ports(0).expect("complete");
        assert_eq!(
            ports[&1],
            PortVlans {
                native: Some(f.vlan("Default")),
                tagged: f.vlans(&["Servers", "IoT", "Guest WiFi", "Cameras"]),
            }
        );
    }

    #[test]
    fn a_block_all_profile_is_an_access_port() {
        let f = Fixture::load();
        let ports = f.ports(0).expect("complete");
        assert_eq!(
            ports[&2],
            PortVlans {
                native: Some(f.vlan("IoT")),
                tagged: vec![],
            }
        );
    }

    #[test]
    fn a_custom_profile_tags_every_vlan_but_the_excluded_ones() {
        let f = Fixture::load();
        let ports = f.ports(0).expect("complete");
        assert_eq!(
            ports[&3],
            PortVlans {
                native: Some(f.vlan("Default")),
                tagged: f.vlans(&["Servers", "IoT", "Cameras"]),
            }
        );
    }

    /// Inline override fields win over the profile the port table names.
    #[test]
    fn an_inline_override_beats_the_port_tables_profile() {
        let f = Fixture::load();
        let ports = f.ports(0).expect("complete");
        assert_eq!(
            ports[&4],
            PortVlans {
                native: Some(f.vlan("Servers")),
                tagged: vec![],
            }
        );
    }

    /// The port table's own `portconf_id` applies where no override names the port.
    #[test]
    fn the_port_tables_profile_applies_without_an_override() {
        let f = Fixture::load();
        let ports = f.ports(0).expect("complete");
        assert_eq!(ports[&5].native, Some(f.vlan("Cameras")));
        assert_eq!(ports[&5].tagged.len(), 4, "auto tags every other VLAN");
    }

    #[test]
    fn a_disabled_port_carries_nothing() {
        let f = Fixture::load();
        assert_eq!(f.ports(0).expect("complete")[&6], PortVlans::default());
    }

    /// The second switch has a port on a profile the controller did not return. Its other ports
    /// resolve, but the device is not claimed complete.
    #[test]
    fn an_unknown_profile_leaves_the_whole_device_unclaimed() {
        let f = Fixture::load();
        assert!(f.ports(1).is_none());
    }

    /// An access point has no port table, so there is no membership to claim.
    #[test]
    fn a_portless_device_claims_nothing() {
        let f = Fixture::load();
        assert!(f.ports(2).is_none());
    }
}
