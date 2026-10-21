//! Raw UniFi controller JSON shapes.
//!
//! **This module is deliberately free of Scanopy entity types.** Everything here mirrors what
//! the controller actually puts on the wire; the translation into hosts/interfaces lives in
//! [`super::mapping`]. When a field shape turns out to be wrong against real hardware, this is
//! the only file that needs correcting.
//!
//! # Provenance
//!
//! Ubiquiti does not officially document the `stat/device` sub-tables (`port_table`,
//! `lldp_table`, `mac_table`, `uplink`, `downlink_table`). The de-facto reference is the
//! **unpoller/unifi** Go library, whose structs are cited per field below. Where unpoller
//! defines one struct per device class (`USW`, `UAP`, `USG`, `UDM`), Scanopy uses a single
//! permissive struct instead — every field is optional, so a device class that omits one
//! simply leaves it `None`.
//!
//! No `deny_unknown_fields` anywhere: newer firmware adds keys routinely, and rejecting them
//! would turn a cosmetic firmware change into total topology loss.

use serde::Deserialize;

/// Standard UniFi response envelope: `{"meta": {"rc": "ok"}, "data": [...]}`.
#[derive(Debug, Deserialize)]
pub struct UnifiEnvelope<T> {
    #[serde(default)]
    pub meta: UnifiMeta,
    #[serde(default = "Vec::new")]
    pub data: Vec<T>,
}

#[derive(Debug, Default, Deserialize)]
pub struct UnifiMeta {
    /// `"ok"` on success; `"error"` with `msg` populated otherwise.
    #[serde(default)]
    pub rc: String,
    #[serde(default)]
    pub msg: Option<String>,
}

impl UnifiMeta {
    pub fn is_ok(&self) -> bool {
        // Older controllers omit `meta` entirely on success, so an empty `rc` is not a failure.
        self.rc.is_empty() || self.rc == "ok"
    }
}

/// One device from `GET /stat/device`.
///
/// unpoller: `unifi.USW` / `unifi.UAP` / `unifi.USG` / `unifi.UDM`.
#[derive(Debug, Default, Clone, Deserialize)]
pub struct UnifiDevice {
    /// unpoller: `USW.Mac`. The device's chassis MAC — its stable identity.
    pub mac: Option<String>,
    /// unpoller: `USW.IP`. Management IP. Absent for a device that has not checked in.
    pub ip: Option<String>,
    /// unpoller: `USW.Name`. Admin-assigned name; absent until the device is named.
    pub name: Option<String>,
    /// unpoller: `USW.Model` (e.g. `"USL24P"`).
    pub model: Option<String>,
    /// unpoller: `USW.Serial`.
    pub serial: Option<String>,
    /// unpoller: `USW.Version`. Running firmware version.
    pub version: Option<String>,
    /// unpoller: `USW.Type` — the device class discriminator (`"usw"`, `"uap"`, `"ugw"`,
    /// `"udm"`). Feeds `Pattern::ManagedDeviceType` for service matching.
    #[serde(rename = "type")]
    pub device_type: Option<String>,
    /// unpoller: `USW.PortTable []Port`. Empty on devices with no switch ports (e.g. an AP).
    #[serde(default)]
    pub port_table: Vec<UnifiPort>,
    /// unpoller: `USW.LLDPTable []LLDPTable`. Neighbors the firmware learned via LLDP —
    /// the payload SNMP cannot reach on these switches.
    #[serde(default)]
    pub lldp_table: Vec<UnifiLldpEntry>,
    /// unpoller: `USW.DownlinkTable []DownlinkTable`. Devices adopted *below* this one.
    #[serde(default)]
    pub downlink_table: Vec<UnifiDownlink>,
    /// unpoller: `USW.Uplink Uplink`. How this device reaches its parent.
    pub uplink: Option<UnifiUplink>,
    /// Per-port configuration that departs from the default profile. go-unifi:
    /// `Device.PortOverrides []DevicePortOverrides` (`json:"port_overrides"`); aiounifi:
    /// `TypedDevice.port_overrides`. Absent on devices with no switch ports, and empty on a switch
    /// whose every port runs the default profile.
    #[serde(default)]
    pub port_overrides: Vec<UnifiPortOverride>,
}

/// One port's configuration on a device. go-unifi: `DevicePortOverrides`; aiounifi:
/// `TypedDevicePortOverrides`.
///
/// Newer controllers set the VLAN fields inline; older ones only point at a port profile through
/// `portconf_id`. Inline fields win, as they do in the controller's own port editor.
#[derive(Debug, Default, Clone, Deserialize)]
pub struct UnifiPortOverride {
    /// go-unifi: `PortIDX` (`json:"port_idx"`). Matches `UnifiPort::port_idx`.
    pub port_idx: Option<FlexInt>,
    /// go-unifi: `PortProfileID` (`json:"portconf_id"`) — the `_id` of a `rest/portconf` profile.
    pub portconf_id: Option<String>,
    /// go-unifi: `NATiveNetworkID` (`json:"native_networkconf_id"`) — the `_id` of the network
    /// carried untagged.
    pub native_networkconf_id: Option<String>,
    /// go-unifi: `TaggedVLANMgmt` (`json:"tagged_vlan_mgmt"`), one of `auto|block_all|custom`.
    pub tagged_vlan_mgmt: Option<String>,
    /// go-unifi: `ExcludedNetworkIDs` (`json:"excluded_networkconf_ids"`).
    #[serde(default)]
    pub excluded_networkconf_ids: Option<Vec<String>>,
    /// go-unifi: `Forward` (`json:"forward"`), one of `all|native|customize|disabled`. The
    /// pre-`tagged_vlan_mgmt` way of saying the same thing.
    pub forward: Option<String>,
}

/// One port profile from `GET /rest/portconf`. go-unifi: `PortProfile`.
///
/// Field names confirmed against Network Application 10.6.106 (`portconf_network_app_10_6.json`).
/// That capture also shows the controller rewriting `forward` to `"native"` when a profile is saved
/// with `tagged_vlan_mgmt: "block_all"`, so the two fields agree on profiles it has touched.
#[derive(Debug, Default, Clone, Deserialize)]
pub struct UnifiPortConf {
    /// go-unifi: `ID` (`json:"_id"`).
    #[serde(rename = "_id")]
    pub id: Option<String>,
    /// go-unifi: `Name`.
    pub name: Option<String>,
    /// go-unifi: `NATiveNetworkID` (`json:"native_networkconf_id"`).
    pub native_networkconf_id: Option<String>,
    /// go-unifi: `TaggedVLANMgmt` (`json:"tagged_vlan_mgmt"`), `auto|block_all|custom`.
    pub tagged_vlan_mgmt: Option<String>,
    /// go-unifi: `ExcludedNetworkIDs` (`json:"excluded_networkconf_ids"`). Under `custom`, every
    /// VLAN network except these is tagged.
    #[serde(default)]
    pub excluded_networkconf_ids: Option<Vec<String>>,
    /// Pre-`tagged_vlan_mgmt` controllers list the tagged networks instead of the excluded ones
    /// (`forward: "customize"` with `tagged_networkconf_ids`). Not in go-unifi's current
    /// definitions, which track the newer field.
    #[serde(default)]
    pub tagged_networkconf_ids: Option<Vec<String>>,
    /// go-unifi: `Forward` (`json:"forward"`), `all|native|customize|disabled`.
    pub forward: Option<String>,
}

/// One network from `GET /rest/networkconf`. go-unifi: `Network`.
///
/// Field names confirmed against Network Application 10.6.106
/// (`networkconf_network_app_10_6.json`). The Art-of-WiFi UniFi-API-client reads the same endpoint
/// as `list_networkconf()`. The v2 `lan/enriched-configuration` endpoint wraps the same
/// `configuration` object and omits `vlan-only` networks, so this one is read instead.
#[derive(Debug, Default, Clone, Deserialize)]
pub struct UnifiNetwork {
    /// go-unifi: `ID` (`json:"_id"`). What port profiles and overrides reference.
    #[serde(rename = "_id")]
    pub id: Option<String>,
    /// go-unifi: `Name`. The name an administrator gave the network.
    pub name: Option<String>,
    /// go-unifi: `Purpose`, one of
    /// `corporate|guest|remote-user-vpn|site-vpn|vlan-only|vpn-client|wan`.
    pub purpose: Option<String>,
    /// go-unifi: `VLAN` (`json:"vlan"`), 2-4018 per its validation. Absent on the untagged
    /// default network, which the captured `Default` network confirms.
    pub vlan: Option<FlexInt>,
    /// go-unifi: `VLANEnabled` (`json:"vlan_enabled"`). `false` on the untagged default network.
    pub vlan_enabled: Option<FlexBool>,
    /// go-unifi: `IPSubnet` (`json:"ip_subnet"`) — the gateway's address with the prefix
    /// (`"10.20.0.1/24"`), not the network address.
    pub ip_subnet: Option<String>,
    /// go-unifi: `Enabled`. Absent on the captured default network; absent reads as enabled.
    pub enabled: Option<FlexBool>,
}

/// One connected client from `GET /stat/sta`.
///
/// unpoller: `unifi.Client`. These are the devices the controller sees on the network but has
/// not adopted — laptops, phones, servers — and the controller is often the only place their
/// user-assigned name exists.
#[derive(Debug, Default, Clone, Deserialize)]
pub struct UnifiStation {
    /// unpoller: `Client.Mac`.
    pub mac: Option<String>,
    /// unpoller: `Client.IP`. Absent for a client with no current lease.
    pub ip: Option<String>,
    /// unpoller: `Client.Name`. The alias an administrator typed in the controller.
    pub name: Option<String>,
    /// unpoller: `Client.Hostname`. What the client called itself over DHCP — a fallback for a
    /// client nobody has named.
    pub hostname: Option<String>,
    /// unpoller: `Client.Oui`. The MAC vendor the controller resolved.
    pub oui: Option<String>,
}

/// One switch port. unpoller: `unifi.Port`.
#[derive(Debug, Default, Clone, Deserialize)]
pub struct UnifiPort {
    /// unpoller: `Port.PortIDX`. 1-based, matches the physical port label.
    pub port_idx: Option<FlexInt>,
    /// unpoller: `Port.Name` (e.g. `"Port 1"`, or an admin-set label).
    pub name: Option<String>,
    /// unpoller: `Port.Up` — link state.
    pub up: Option<FlexBool>,
    /// unpoller: `Port.Enable` — admin state.
    pub enable: Option<FlexBool>,
    /// unpoller: `Port.Speed`, in Mbit/s (0 when down).
    pub speed: Option<FlexInt>,
    /// unpoller: `Port.Mac`. Frequently the device's chassis MAC repeated on every port
    /// rather than a per-port address — see `mapping::port_to_interface` for why that
    /// distinction matters.
    pub mac: Option<String>,
    /// unpoller: `Port.IsUplink`.
    pub is_uplink: Option<FlexBool>,
    /// aiounifi: `TypedDevicePortTable.portconf_id` — the profile the port is running. Present
    /// where the device has no `port_overrides` entry for the port.
    pub portconf_id: Option<String>,
    /// unpoller: `Port.MacTable []MacTable`. Learned MACs on this port (the bridge FDB).
    #[serde(default)]
    pub mac_table: Vec<UnifiMacTableEntry>,
}

/// One learned MAC on a port. unpoller: `unifi.MacTable`.
#[derive(Debug, Default, Clone, Deserialize)]
pub struct UnifiMacTableEntry {
    /// unpoller: `MacTable.Mac`.
    pub mac: Option<String>,
    /// unpoller: `MacTable.Vlan`.
    pub vlan: Option<FlexInt>,
    /// unpoller: `MacTable.Age`.
    pub age: Option<FlexInt>,
    /// unpoller: `MacTable.Static`.
    #[serde(rename = "static")]
    pub is_static: Option<FlexBool>,
}

/// One LLDP neighbor. unpoller: `unifi.LLDPTable`.
///
/// Note the controller reports already-decoded strings and **no IEEE subtype byte**, which is
/// why mapping uses `LldpChassisId::from_identifier_str` rather than `from_snmp`.
#[derive(Debug, Default, Clone, Deserialize)]
pub struct UnifiLldpEntry {
    /// unpoller: `LLDPTable.ChassisID`. Usually the neighbor's MAC; sometimes a hostname.
    pub chassis_id: Option<String>,
    /// unpoller: `LLDPTable.PortID`. The neighbor's port identifier, vendor-formatted.
    pub port_id: Option<String>,
    /// unpoller: `LLDPTable.LocalPortIdx`. Which of *our* ports saw this neighbor.
    pub local_port_idx: Option<FlexInt>,
    /// unpoller: `LLDPTable.LocalPortName`.
    pub local_port_name: Option<String>,
    /// unpoller: `LLDPTable.IsWired`.
    pub is_wired: Option<FlexBool>,
}

/// A device adopted below this one. unpoller: `unifi.DownlinkTable`.
#[derive(Debug, Default, Clone, Deserialize)]
pub struct UnifiDownlink {
    /// unpoller: `DownlinkTable.Mac` — the downstream device's chassis MAC.
    pub mac: Option<String>,
    /// unpoller: `DownlinkTable.PortIdx` — our port it is attached to.
    pub port_idx: Option<FlexInt>,
    /// unpoller: `DownlinkTable.SpeedMbps`.
    pub speed: Option<FlexInt>,
}

/// How a device reaches its parent. unpoller: `unifi.Uplink`.
#[derive(Debug, Default, Clone, Deserialize)]
pub struct UnifiUplink {
    /// unpoller: `Uplink.Name` — our local uplink interface name (e.g. `"eth0"`).
    pub name: Option<String>,
    /// unpoller: `Uplink.Mac` — *our* uplink interface MAC. On an AP this is a genuinely
    /// distinct per-interface address, unlike the repeated port MACs on a switch.
    pub mac: Option<String>,
    /// unpoller: `Uplink.PortIdx` — our local port index, when we have a port table.
    pub port_idx: Option<FlexInt>,
    /// unpoller: `Uplink.UplinkMac` — the *parent's* chassis MAC.
    pub uplink_mac: Option<String>,
    /// unpoller: `Uplink.UplinkRemotePort` — the parent's port index we plug into.
    pub uplink_remote_port: Option<FlexInt>,
    /// unpoller: `Uplink.UplinkDeviceName` — the parent's name, when reported.
    pub uplink_device_name: Option<String>,
}

// `FlexInt`/`FlexBool` moved to `super::super::flex` when Instant On needed the same tolerance
// against its own loosely-typed vendor JSON. Re-exported here so the wire structs above (and
// anything importing them from this module) are unaffected.
pub use crate::daemon::discovery::integration::flex::{FlexBool, FlexInt};
