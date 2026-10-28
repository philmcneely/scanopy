use crate::server::hosts::r#impl::base::Host;
use crate::server::ip_addresses::r#impl::base::{IPAddress, mac_of};
use crate::server::services::r#impl::base::Service;
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::{
    ServiceDefinition, ServiceDefinitionExt, VirtualizationRole,
};
use crate::server::services::r#impl::patterns::Vendor;
use crate::server::shared::entities::EntityDiscriminants;
use crate::server::shared::oui;
use crate::server::shared::types::metadata::{EntityMetadataProvider, HasId, TypeMetadataProvider};
use crate::server::shared::types::{Color, Icon};
use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, IntoStaticStr};
use utoipa::ToSchema;

/// What a host is, as a device: the class its icon stands for on the map.
///
/// Declaration order is precedence, highest first. A host whose services claim several device types takes
/// the first one here, so a firewall that also serves files reads as a firewall, and `Server`,
/// which any application implies, never hides a more specific device type.
///
/// `Unidentified` means no evidence yet, not a device class outside this list: every class the
/// evidence names gets its own variant.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    ToSchema,
    EnumIter,
    IntoStaticStr,
)]
pub enum DeviceType {
    // A refinement sits directly above the class it refines, so a host whose services claim both
    // reads as the more specific one: a Synology box that also answers Modbus is a NAS.
    Ips,
    SecurityAppliance,
    Vpn,
    Firewall,
    SdWanAppliance,
    BroadbandRouter,
    CableModem,
    DslModem,
    VoipGateway,
    SipGateway,
    Multiplexer,
    Telecom,
    Router,
    Switch,
    NetworkManagementDevice,
    LightsOutManagement,
    Kvm,
    DeviceServer,
    WirelessController,
    WlanRepeater,
    Powerline,
    AccessPoint,
    Hypervisor,
    Nas,
    StorageAppliance,
    TapeLibrary,
    Storage,
    MultifunctionDevice,
    PrintServer,
    Scanner,
    HandheldScanner,
    CheckScanner,
    Printer,
    NetworkVideoRecorder,
    Dvr,
    WebCam,
    Camera,
    VideoConferencing,
    VideoEncoder,
    VideoDecoder,
    DisplayController,
    WirelessPresenter,
    Whiteboard,
    VoipServer,
    VoiceAppliance,
    Phone,
    Laptop,
    Desktop,
    Workstation,
    Tablet,
    Mobile,
    DataTerminal,
    SmartTv,
    Iptv,
    MediaReceiver,
    AvReceiver,
    MediaServer,
    MediaPlayer,
    NetworkAudio,
    AudioEncoder,
    Speaker,
    Plc,
    HmiController,
    Industrial,
    BuildingAutomation,
    AccessControl,
    AlarmPanel,
    EnvironmentControl,
    Ups,
    PowerDevice,
    PowerMeter,
    PowerRelay,
    RelayController,
    DeviceHub,
    Sensor,
    LightBulb,
    IoT,
    Medical,
    TestInstrument,
    JtagAdapter,
    EthernetAdapter,
    SupportAppliance,
    Monitoring,
    Server,
    /// Also what a device type this build doesn't know reads as, so a newer server cannot break an older
    /// reader.
    #[default]
    #[serde(other)]
    Unidentified,
}

impl std::fmt::Display for DeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.id())
    }
}

impl HasId for DeviceType {
    fn id(&self) -> &'static str {
        self.into()
    }
}

impl EntityMetadataProvider for DeviceType {
    /// Every device type wears the Host entity's colour: a device is a host first, and the glyph alone
    /// names its device type. Software takes its category's colour instead, so the two never compete.
    fn color(&self) -> Color {
        EntityDiscriminants::Host.color()
    }

    fn icon(&self) -> Icon {
        // Lucide has no modem, tape or HMI glyph; those take the nearest shape no other device type
        // uses (`no_two_device_types_share_an_icon`).
        match self {
            DeviceType::Ips => Icon::ShieldAlert,
            DeviceType::SecurityAppliance => Icon::ShieldHalf,
            DeviceType::Vpn => Icon::LockKeyhole,
            DeviceType::Firewall => Icon::BrickWallFire,
            DeviceType::SdWanAppliance => Icon::Earth,
            DeviceType::BroadbandRouter => Icon::HouseWifi,
            DeviceType::CableModem => Icon::Cable,
            DeviceType::DslModem => Icon::Signal,
            DeviceType::VoipGateway => Icon::PhoneForwarded,
            DeviceType::SipGateway => Icon::ArrowRightLeft,
            DeviceType::Multiplexer => Icon::Merge,
            DeviceType::Telecom => Icon::RadioTower,
            DeviceType::Router => Icon::Router,
            DeviceType::Switch => Icon::Network,
            DeviceType::NetworkManagementDevice => Icon::Settings2,
            DeviceType::LightsOutManagement => Icon::ServerCog,
            DeviceType::Kvm => Icon::Keyboard,
            DeviceType::DeviceServer => Icon::SquareTerminal,
            DeviceType::WirelessController => Icon::WifiCog,
            DeviceType::WlanRepeater => Icon::Repeat2,
            DeviceType::Powerline => Icon::PlugZap,
            DeviceType::AccessPoint => Icon::Wifi,
            DeviceType::Hypervisor => ServiceCategory::Hypervisor.icon(),
            DeviceType::Nas => Icon::Database,
            DeviceType::StorageAppliance => Icon::Layers2,
            DeviceType::TapeLibrary => Icon::Archive,
            DeviceType::Storage => Icon::HardDrive,
            DeviceType::MultifunctionDevice => Icon::PrinterCheck,
            DeviceType::PrintServer => Icon::Receipt,
            DeviceType::Scanner => Icon::Scan,
            DeviceType::HandheldScanner => Icon::ScanBarcode,
            DeviceType::CheckScanner => Icon::ScanText,
            DeviceType::Printer => Icon::Printer,
            DeviceType::NetworkVideoRecorder => Icon::Film,
            DeviceType::Dvr => Icon::Videotape,
            DeviceType::WebCam => Icon::Webcam,
            DeviceType::Camera => Icon::Cctv,
            DeviceType::VideoConferencing => Icon::Video,
            DeviceType::VideoEncoder => Icon::Clapperboard,
            DeviceType::VideoDecoder => Icon::MonitorPlay,
            DeviceType::DisplayController => Icon::Monitor,
            DeviceType::WirelessPresenter => Icon::Cast,
            DeviceType::Whiteboard => Icon::Presentation,
            DeviceType::VoipServer => Icon::PhoneIncoming,
            DeviceType::VoiceAppliance => Icon::Voicemail,
            DeviceType::Phone => Icon::Phone,
            DeviceType::Laptop => Icon::Laptop,
            DeviceType::Desktop => Icon::PcCase,
            DeviceType::Workstation => Icon::Computer,
            DeviceType::Tablet => Icon::Tablet,
            DeviceType::Mobile => Icon::Smartphone,
            DeviceType::DataTerminal => Icon::Terminal,
            DeviceType::SmartTv => Icon::TvMinimalPlay,
            DeviceType::Iptv => Icon::SatelliteDish,
            DeviceType::MediaReceiver => Icon::RadioReceiver,
            DeviceType::AvReceiver => Icon::MonitorSpeaker,
            DeviceType::MediaServer => Icon::Disc,
            DeviceType::MediaPlayer => Icon::Tv,
            DeviceType::NetworkAudio => Icon::AudioLines,
            DeviceType::AudioEncoder => Icon::AudioWaveform,
            DeviceType::Speaker => Icon::Speaker,
            DeviceType::Plc => Icon::Microchip,
            DeviceType::HmiController => Icon::SlidersVertical,
            DeviceType::Industrial => Icon::Factory,
            DeviceType::BuildingAutomation => Icon::Building2,
            DeviceType::AccessControl => Icon::DoorClosedLocked,
            DeviceType::AlarmPanel => Icon::Siren,
            DeviceType::EnvironmentControl => Icon::Thermometer,
            DeviceType::Ups => Icon::BatteryCharging,
            DeviceType::PowerDevice => Icon::Zap,
            DeviceType::PowerMeter => Icon::Gauge,
            DeviceType::PowerRelay => Icon::ToggleRight,
            DeviceType::RelayController => Icon::SlidersHorizontal,
            DeviceType::DeviceHub => Icon::Waypoints,
            DeviceType::Sensor => Icon::Radar,
            DeviceType::LightBulb => Icon::Lightbulb,
            DeviceType::IoT => Icon::CircuitBoard,
            DeviceType::Medical => Icon::Stethoscope,
            DeviceType::TestInstrument => Icon::Microscope,
            DeviceType::JtagAdapter => Icon::Usb,
            DeviceType::EthernetAdapter => Icon::EthernetPort,
            DeviceType::SupportAppliance => Icon::LifeBuoy,
            DeviceType::Monitoring => Icon::ChartLine,
            DeviceType::Server => Icon::Server,
            // The glyph every other unknown value draws, so a host nothing identifies never reads as
            // a device type. The Host icon is `Server`'s.
            DeviceType::Unidentified => Icon::CircleQuestionMark,
        }
    }
}

impl TypeMetadataProvider for DeviceType {
    fn name(&self) -> &'static str {
        match self {
            DeviceType::Ips => "Intrusion Prevention System",
            DeviceType::SecurityAppliance => "Security Appliance",
            DeviceType::Vpn => "VPN Gateway",
            DeviceType::Firewall => "Firewall",
            DeviceType::SdWanAppliance => "SD-WAN Appliance",
            DeviceType::BroadbandRouter => "Broadband Router",
            DeviceType::CableModem => "Cable Modem",
            DeviceType::DslModem => "DSL Modem",
            DeviceType::VoipGateway => "VoIP Gateway",
            DeviceType::SipGateway => "SIP Gateway",
            DeviceType::Multiplexer => "Multiplexer",
            DeviceType::Telecom => "Telecom Equipment",
            DeviceType::Router => "Router",
            DeviceType::Switch => "Switch",
            DeviceType::NetworkManagementDevice => "Network Management Device",
            DeviceType::LightsOutManagement => "Lights-Out Management",
            DeviceType::Kvm => "KVM Switch",
            DeviceType::DeviceServer => "Device Server",
            DeviceType::WirelessController => "Wireless Controller",
            DeviceType::WlanRepeater => "Wireless Repeater",
            DeviceType::Powerline => "Powerline Adapter",
            DeviceType::AccessPoint => "Access Point",
            DeviceType::Hypervisor => "Hypervisor",
            DeviceType::Nas => "NAS",
            DeviceType::StorageAppliance => "Storage Appliance",
            DeviceType::TapeLibrary => "Tape Library",
            DeviceType::Storage => "Storage",
            DeviceType::MultifunctionDevice => "Multifunction Printer",
            DeviceType::PrintServer => "Print Server",
            DeviceType::Scanner => "Scanner",
            DeviceType::HandheldScanner => "Handheld Scanner",
            DeviceType::CheckScanner => "Check Scanner",
            DeviceType::Printer => "Printer",
            DeviceType::NetworkVideoRecorder => "Network Video Recorder",
            DeviceType::Dvr => "DVR",
            DeviceType::WebCam => "Webcam",
            DeviceType::Camera => "Camera",
            DeviceType::VideoConferencing => "Video Conferencing",
            DeviceType::VideoEncoder => "Video Encoder",
            DeviceType::VideoDecoder => "Video Decoder",
            DeviceType::DisplayController => "Display Controller",
            DeviceType::WirelessPresenter => "Wireless Presenter",
            DeviceType::Whiteboard => "Interactive Whiteboard",
            DeviceType::VoipServer => "VoIP Server",
            DeviceType::VoiceAppliance => "Voice Appliance",
            DeviceType::Phone => "Phone",
            DeviceType::Laptop => "Laptop",
            DeviceType::Desktop => "Desktop",
            DeviceType::Workstation => "Workstation",
            DeviceType::Tablet => "Tablet",
            DeviceType::Mobile => "Mobile Device",
            DeviceType::DataTerminal => "Data Terminal",
            DeviceType::SmartTv => "Smart TV",
            DeviceType::Iptv => "IPTV Device",
            DeviceType::MediaReceiver => "Media Receiver",
            DeviceType::AvReceiver => "AV Receiver",
            DeviceType::MediaServer => "Media Server",
            DeviceType::MediaPlayer => "Media Player",
            DeviceType::NetworkAudio => "Network Audio",
            DeviceType::AudioEncoder => "Audio Encoder",
            DeviceType::Speaker => "Speaker",
            DeviceType::Plc => "PLC",
            DeviceType::HmiController => "HMI Controller",
            DeviceType::Industrial => "Industrial Controller",
            DeviceType::BuildingAutomation => "Building Automation",
            DeviceType::AccessControl => "Access Control",
            DeviceType::AlarmPanel => "Alarm Panel",
            DeviceType::EnvironmentControl => "Environment Control",
            DeviceType::Ups => "UPS",
            DeviceType::PowerDevice => "Power Device",
            DeviceType::PowerMeter => "Power Meter",
            DeviceType::PowerRelay => "Smart Plug",
            DeviceType::RelayController => "Relay Controller",
            DeviceType::DeviceHub => "Smart Home Hub",
            DeviceType::Sensor => "Sensor",
            DeviceType::LightBulb => "Smart Light",
            DeviceType::IoT => "IoT Device",
            DeviceType::Medical => "Medical Device",
            DeviceType::TestInstrument => "Test Instrument",
            DeviceType::JtagAdapter => "JTAG Adapter",
            DeviceType::EthernetAdapter => "Ethernet Adapter",
            DeviceType::SupportAppliance => "Support Appliance",
            DeviceType::Monitoring => "Monitoring Appliance",
            DeviceType::Server => "Server",
            DeviceType::Unidentified => "Unidentified",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            DeviceType::Ips => "Inspects traffic and blocks attacks",
            DeviceType::SecurityAppliance => "Filters, inspects or proxies traffic for security",
            DeviceType::Vpn => "Terminates VPN tunnels",
            DeviceType::Firewall => "Filters traffic between networks",
            DeviceType::SdWanAppliance => "Routes branch traffic over several WAN links",
            DeviceType::BroadbandRouter => "Connects a site to its internet provider",
            DeviceType::CableModem => "Connects to a cable internet line",
            DeviceType::DslModem => "Connects to a DSL internet line",
            DeviceType::VoipGateway => "Connects phone lines to a VoIP network",
            DeviceType::SipGateway => "Bridges SIP calls between networks",
            DeviceType::Multiplexer => "Combines several signals onto one link",
            DeviceType::Telecom => "Carrier or telephony network equipment",
            DeviceType::Router => "Routes traffic between networks",
            DeviceType::Switch => "Forwards frames between ports on one network",
            DeviceType::NetworkManagementDevice => "Manages or monitors network equipment",
            DeviceType::LightsOutManagement => {
                "Manages a server out of band, independent of its OS"
            }
            DeviceType::Kvm => "Shares keyboard, video and mouse across machines",
            DeviceType::DeviceServer => "Puts serial devices on the network",
            DeviceType::WirelessController => "Manages a fleet of access points",
            DeviceType::WlanRepeater => "Extends a wireless network",
            DeviceType::Powerline => "Carries the network over power wiring",
            DeviceType::AccessPoint => "Bridges wireless clients onto the network",
            DeviceType::Hypervisor => "Runs virtual machines",
            DeviceType::Nas => "Shares files from its own disks",
            DeviceType::StorageAppliance => "Provides storage to servers",
            DeviceType::TapeLibrary => "Backs data up to tape",
            DeviceType::Storage => "Holds files or blocks for other hosts",
            DeviceType::MultifunctionDevice => "Prints, scans and copies",
            DeviceType::PrintServer => "Shares printers on the network",
            DeviceType::Scanner => "Scans documents",
            DeviceType::HandheldScanner => "Reads barcodes by hand",
            DeviceType::CheckScanner => "Scans checks",
            DeviceType::Printer => "Prints or scans",
            DeviceType::NetworkVideoRecorder => "Records network camera streams",
            DeviceType::Dvr => "Records camera video",
            DeviceType::WebCam => "A camera attached to a computer",
            DeviceType::Camera => "Streams video",
            DeviceType::VideoConferencing => "Runs video calls in a meeting room",
            DeviceType::VideoEncoder => "Encodes video for streaming",
            DeviceType::VideoDecoder => "Decodes a video stream to a display",
            DeviceType::DisplayController => "Drives a wall or array of displays",
            DeviceType::WirelessPresenter => "Shares a laptop screen to a room display",
            DeviceType::Whiteboard => "A touch display for meetings",
            DeviceType::VoipServer => "Runs calls for VoIP phones",
            DeviceType::VoiceAppliance => "Records or routes voice calls",
            DeviceType::Phone => "Carries voice calls",
            DeviceType::Laptop => "A portable computer",
            DeviceType::Desktop => "A desktop computer",
            DeviceType::Workstation => "A computer a person works at",
            DeviceType::Tablet => "A tablet",
            DeviceType::Mobile => "A phone or other handheld device",
            DeviceType::DataTerminal => "A terminal for data entry or payment",
            DeviceType::SmartTv => "A television with apps",
            DeviceType::Iptv => "Receives television over the network",
            DeviceType::MediaReceiver => "Receives broadcast or streamed media",
            DeviceType::AvReceiver => "Switches and amplifies home audio and video",
            DeviceType::MediaServer => "Serves media to players",
            DeviceType::MediaPlayer => "Plays video to a screen",
            DeviceType::NetworkAudio => "Streams audio over the network",
            DeviceType::AudioEncoder => "Encodes audio for streaming",
            DeviceType::Speaker => "Plays audio",
            DeviceType::Plc => "Runs control logic for machinery",
            DeviceType::HmiController => "The operator panel for industrial equipment",
            DeviceType::Industrial => "Controls building or industrial equipment",
            DeviceType::BuildingAutomation => "Controls a building's HVAC, lighting or systems",
            DeviceType::AccessControl => "Controls doors or locks",
            DeviceType::AlarmPanel => "Runs an alarm system",
            DeviceType::EnvironmentControl => "Controls temperature or climate",
            DeviceType::Ups => "Keeps equipment powered through outages",
            DeviceType::PowerDevice => "Distributes or manages power",
            DeviceType::PowerMeter => "Measures power use",
            DeviceType::PowerRelay => "Switches an outlet on and off",
            DeviceType::RelayController => "Switches circuits remotely",
            DeviceType::DeviceHub => "Bridges smart-home devices onto the network",
            DeviceType::Sensor => "Reports what it measures",
            DeviceType::LightBulb => "A networked light",
            DeviceType::IoT => "A smart-home or embedded device",
            DeviceType::Medical => "Medical or clinical equipment",
            DeviceType::TestInstrument => "Measures or tests electronics",
            DeviceType::JtagAdapter => "Debugs hardware over JTAG",
            DeviceType::EthernetAdapter => "Puts a device on Ethernet",
            DeviceType::SupportAppliance => "A vendor's remote support appliance",
            DeviceType::Monitoring => "Monitors networks or systems",
            DeviceType::Server => "Runs applications for other hosts",
            DeviceType::Unidentified => "Nothing found on this host names what it is",
        }
    }
}

impl Host {
    /// The device type this host's icon shows.
    ///
    /// The ladder, first answer wins:
    /// 1. The highest-precedence device type the host's own services declare, other than `Server`.
    ///    A matched definition names the product the host runs, so it outranks the OS.
    /// 2. The device type a fingerprint of something the host emitted names (`matched_device_type`):
    ///    a certificate subject or page title that says "IP Camera" is more specific than an OS
    ///    family, and less than the product a matched definition names.
    /// 3. The device type its OS family implies.
    /// 4. The device type its hostname or name implies: Apple names a device after its product
    ///    ("Maya's iPhone", "Mayas-MacBook-Pro"), and that name reaches us through PTR, mDNS and
    ///    controller DHCP leases.
    /// 5. The device type its MAC vendor implies, for vendors whose every product is one class of
    ///    device: Espressif sells only embedded Wi-Fi chips.
    /// 6. `Server`, when one of its services implies it.
    /// 7. `Server`, when the host runs containers: a container runtime, or software inside one.
    /// 8. `Unidentified`.
    ///
    /// `Server` is what any application implies, so it yields to an OS or a name that says what
    /// the machine is: a Mac sharing files over SMB, or running Postgres, is still a workstation.
    /// A name sits below the OS because anyone can type one. Containerized software says what the
    /// container does, not what the machine is, so it ranks with the weakest evidence. A Linux host
    /// gives no OS device type, so its applications or containers still make it a server.
    pub fn device_type<'a, 'b>(
        &self,
        services: impl IntoIterator<Item = &'a Service>,
        ip_addresses: impl IntoIterator<Item = &'b IPAddress>,
    ) -> DeviceType {
        let (containerized, own): (Vec<&Service>, Vec<&Service>) = services
            .into_iter()
            .partition(|s| s.base.virtualization_metadata.is_some());
        let runs_containers = !containerized.is_empty()
            || own.iter().any(|s| {
                matches!(
                    s.base.service_definition.virtualization_role(),
                    Some(VirtualizationRole::ContainerRuntime { .. })
                )
            });

        let declared: Vec<DeviceType> = own
            .iter()
            .filter_map(|s| s.base.service_definition.device_type())
            .collect();
        let implies_server = declared.contains(&DeviceType::Server);

        declared
            .into_iter()
            .filter(|device_type| *device_type != DeviceType::Server)
            .min()
            .or_else(|| {
                self.base
                    .matched_device_type
                    .as_ref()
                    .map(|matched| matched.value().0)
                    .filter(|matched| *matched != DeviceType::Unidentified)
            })
            .or_else(|| {
                self.base
                    .os
                    .as_ref()
                    .and_then(|os| os.value().0.device_type())
            })
            .or_else(|| self.named_device_type())
            .or_else(|| {
                ip_addresses
                    .into_iter()
                    .filter_map(|ip| mac_of(&ip.base.mac_address))
                    .find_map(|mac| device_type_from_mac(&mac.to_string()))
            })
            .or((implies_server || runs_containers).then_some(DeviceType::Server))
            .unwrap_or_default()
    }

    /// The device type the hostname or name names by product, for products that name themselves.
    fn named_device_type(&self) -> Option<DeviceType> {
        let hostname = self.base.hostname.as_ref().map(|h| h.value().0.clone());
        let name = self.base.name.value().as_str().into_owned();
        [hostname, Some(name)]
            .into_iter()
            .flatten()
            .find_map(|label| device_type_from_label(&label))
    }
}

/// The device type a MAC's vendor implies. Only vendors that make one class of device are listed:
/// Google's OUI covers phones, laptops and speakers alike, so it implies nothing. A randomized
/// (locally administered) MAC has no vendor and implies nothing either.
fn device_type_from_mac(mac: &str) -> Option<DeviceType> {
    match oui::lookup_by_mac(mac)?.company_name.as_str() {
        Vendor::ESPRESSIF | Vendor::NEST => Some(DeviceType::IoT),
        _ => None,
    }
}

/// `"Mayas-iPhone"`, `"Maya's iPad"`, `"iphone15"` and `"FB-Macbook"` name a product;
/// `"iphoneclone"` does not. A word matches when it is the product name, optionally followed by a
/// model number. A Mac name split in two (`"mac-mini"`, `"Mac Studio"`) matches as the pair; a bare
/// `"mac"` names nothing. A brand's tablet (`"xiaomi-pad-mini"`) matches as the pair; a bare `"pad"`
/// names nothing.
fn device_type_from_label(label: &str) -> Option<DeviceType> {
    let words: Vec<String> = label
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_ascii_lowercase)
        .collect();
    words.iter().enumerate().find_map(|(i, word)| {
        let product = word.trim_end_matches(|c: char| c.is_ascii_digit());
        match product {
            "iphone" => Some(DeviceType::Mobile),
            // Android's default device name is "<Owner>'s Phone". A desk phone calls itself an IP
            // or SIP phone, so those pairs stay unmatched.
            "phone"
                if !matches!(
                    i.checked_sub(1)
                        .and_then(|p| words.get(p))
                        .map(String::as_str),
                    Some("ip" | "voip" | "sip")
                ) =>
            {
                Some(DeviceType::Mobile)
            }
            "ipad" => Some(DeviceType::Tablet),
            "pad"
                if matches!(
                    i.checked_sub(1)
                        .and_then(|p| words.get(p))
                        .map(String::as_str),
                    Some("xiaomi" | "redmi")
                ) =>
            {
                Some(DeviceType::Tablet)
            }
            "doorbell" => Some(DeviceType::Camera),
            // A Schlage Wi-Fi lock's default DHCP name. It sleeps between cloud check-ins and opens
            // no ports, so the name is the only evidence a scan gets.
            "schlage" => Some(DeviceType::AccessControl),
            "macbook" | "macbookpro" | "macbookair" => Some(DeviceType::Laptop),
            "imac" | "imacpro" | "macmini" | "macpro" | "macstudio" => Some(DeviceType::Desktop),
            "mac"
                if matches!(
                    words.get(i + 1).map(String::as_str),
                    Some("mini" | "pro" | "studio")
                ) =>
            {
                Some(DeviceType::Desktop)
            }
            _ => None,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::hosts::r#impl::attributes::{HostHostnameValue, HostOsValue};
    use crate::server::hosts::r#impl::os::{HostOs, HostOsFamily};
    use crate::server::ip_addresses::r#impl::base::{MacEvidence, MacEvidenceValue};
    use crate::server::services::definitions::{
        docker_daemon::Docker, kerberos::Kerberos, pf_sense::PfSense, postgresql::PostgreSQL,
        proxmox::Proxmox, samba::Samba, snmp::Snmp, true_nas::TrueNAS,
    };
    use crate::server::services::r#impl::base::ServiceBase;
    use crate::server::services::r#impl::virtualization::{
        DockerVirtualization, ServiceVirtualization,
    };
    use crate::server::shared::attribution::{AttributeSource, Attributed};
    use crate::tests::ip_address;
    use uuid::Uuid;

    fn host_with_os(os: Option<HostOs>) -> Host {
        let mut host = crate::server::shared::types::examples::host();
        host.base.os =
            os.map(|os| Attributed::new(HostOsValue(os), AttributeSource::SshBannerMatch));
        host
    }

    fn service(definition: Box<dyn ServiceDefinition>) -> Service {
        Service {
            base: ServiceBase {
                service_definition: definition,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn containerized(definition: Box<dyn ServiceDefinition>) -> Service {
        let mut service = service(definition);
        service.base.virtualization_metadata =
            Some(ServiceVirtualization::Docker(DockerVirtualization {
                state: Default::default(),
                container_name: None,
                container_id: None,
                compose_project: None,
            }));
        service
    }

    /// Software in a container says what the container does, not what the machine is: it ranks
    /// below the OS, and makes a host a server only when nothing else names it.
    #[test]
    fn containers_rank_below_the_os_and_above_nothing() {
        let os = |family| Some(HostOs::family_only(family));
        let db_in_docker = [
            service(Box::new(Docker)),
            containerized(Box::new(PostgreSQL)),
        ];

        // A Mac running Postgres in Docker is a workstation.
        assert_eq!(
            host_with_os(os(HostOsFamily::MacOs)).device_type(&db_in_docker, []),
            DeviceType::Workstation
        );
        // A Linux host gives no OS device type, so the same containers make it a server, and so does the
        // runtime alone.
        assert_eq!(
            host_with_os(os(HostOsFamily::Linux)).device_type(&db_in_docker, []),
            DeviceType::Server
        );
        assert_eq!(
            host_with_os(os(HostOsFamily::Linux)).device_type(&db_in_docker[..1], []),
            DeviceType::Server
        );
        // Software installed on the host itself implies only `Server`, which yields to the OS too.
        assert_eq!(
            host_with_os(os(HostOsFamily::MacOs)).device_type(&[service(Box::new(PostgreSQL))], []),
            DeviceType::Workstation
        );
        // And a host's own device type outranks its containers.
        let nas_with_containers = [
            service(Box::new(TrueNAS)),
            containerized(Box::new(PostgreSQL)),
        ];
        assert_eq!(
            host_with_os(os(HostOsFamily::Linux)).device_type(&nas_with_containers, []),
            DeviceType::Nas
        );
    }

    /// The ladder from most specific evidence to none: a service's specific device type beats every
    /// lower-ranked one and the OS, and the `Server` a service implies yields to the OS.
    #[test]
    fn services_outrank_each_other_by_precedence_then_the_os() {
        let macos = Some(HostOs::family_only(HostOsFamily::MacOs));

        let firewall_and_nas = [service(Box::new(TrueNAS)), service(Box::new(PfSense))];
        assert_eq!(
            host_with_os(None).device_type(&firewall_and_nas, []),
            DeviceType::Firewall
        );

        let hypervisor_with_db = [service(Box::new(PostgreSQL)), service(Box::new(Proxmox))];
        assert_eq!(
            host_with_os(None).device_type(&hypervisor_with_db, []),
            DeviceType::Hypervisor
        );

        // A service's specific device type wins over the OS device type.
        let nas = [service(Box::new(TrueNAS))];
        assert_eq!(
            host_with_os(macos.clone()).device_type(&nas, []),
            DeviceType::Nas
        );

        // `Server` from a service yields to the OS: a Mac sharing files over SMB is a workstation.
        let plumbing = [service(Box::new(Snmp)), service(Box::new(Samba))];
        assert_eq!(
            host_with_os(macos.clone()).device_type(&plumbing[..1], []),
            DeviceType::Workstation
        );
        assert_eq!(
            host_with_os(macos).device_type(&plumbing, []),
            DeviceType::Workstation
        );
        // With no OS or name to yield to, the same service still makes it a server.
        assert_eq!(
            host_with_os(None).device_type(&plumbing, []),
            DeviceType::Server
        );

        // The SMB probe's Windows reading names no edition, so it implies nothing, and a Windows
        // server's applications still make it a server.
        let windows_by_version = Some(HostOs {
            version: Some("10.0.17763".to_string()),
            ..HostOs::family_only(HostOsFamily::Windows)
        });
        assert_eq!(
            host_with_os(windows_by_version).device_type(&[service(Box::new(PostgreSQL))], []),
            DeviceType::Server
        );
    }

    #[test]
    fn a_host_with_no_evidence_is_unidentified() {
        let linux = Some(HostOs::family_only(HostOsFamily::Linux));
        let snmp = [service(Box::new(Snmp))];
        assert_eq!(
            host_with_os(linux).device_type(&snmp, []),
            DeviceType::Unidentified
        );
        assert_eq!(
            host_with_os(None).device_type(&[], []),
            DeviceType::Unidentified
        );
    }

    /// A product name in the hostname identifies a host nothing else does, and yields to every
    /// stronger rung.
    #[test]
    fn a_product_name_identifies_a_host_below_services_and_the_os() {
        let named = |hostname: &str| {
            let mut host = host_with_os(None);
            host.base.hostname = Some(Attributed::new(
                HostHostnameValue(hostname.to_string()),
                AttributeSource::ReverseDns,
            ));
            host
        };
        assert_eq!(
            named("Mayas-iPhone.local").device_type(&[], []),
            DeviceType::Mobile
        );
        assert_eq!(named("ipad13.lan").device_type(&[], []), DeviceType::Tablet);
        assert_eq!(
            named("iphoneclone.lan").device_type(&[], []),
            DeviceType::Unidentified
        );
        assert_eq!(
            named("mac.lan").device_type(&[], []),
            DeviceType::Unidentified
        );
        // Hostnames from a live scan with randomized MACs, where the name is the only evidence.
        assert_eq!(
            named("josep-s-phone.lan").device_type(&[], []),
            DeviceType::Mobile
        );
        assert_eq!(
            named("xiaomi-pad-mini.lan").device_type(&[], []),
            DeviceType::Tablet
        );
        assert_eq!(
            named("nest-doorbell-battery.lan").device_type(&[], []),
            DeviceType::Camera
        );
        assert_eq!(
            named("schlage.lan").device_type(&[], []),
            DeviceType::AccessControl
        );
        // A desk phone and a bare "pad" name no handheld.
        assert_eq!(
            named("lobby-ip-phone").device_type(&[], []),
            DeviceType::Unidentified
        );
        assert_eq!(
            named("pad.lan").device_type(&[], []),
            DeviceType::Unidentified
        );

        // The two Macs from a live scan, each sharing files and running macOS's local Kerberos KDC,
        // with no OS reading.
        let mac_services = [service(Box::new(Samba)), service(Box::new(Kerberos))];
        for (hostname, expected) in [
            ("mac-mini.lan", DeviceType::Desktop),
            ("FB-Macbook.local", DeviceType::Laptop),
            ("Mayas-MacBook-Pro.local", DeviceType::Laptop),
        ] {
            assert_eq!(
                named(hostname).device_type(&mac_services, []),
                expected,
                "{hostname}"
            );
        }

        // The OS and services both outrank the name.
        let mut mac = named("Mayas-iPhone");
        mac.base.os = Some(Attributed::new(
            HostOsValue(HostOs::family_only(HostOsFamily::MacOs)),
            AttributeSource::SshBannerMatch,
        ));
        assert_eq!(mac.device_type(&[], []), DeviceType::Workstation);
        assert_eq!(
            named("ipad").device_type(&[service(Box::new(TrueNAS))], []),
            DeviceType::Nas
        );
    }

    /// A single-purpose vendor's MAC types a host nothing else names, below the OS and the name and
    /// above the `Server` an application implies.
    #[test]
    fn a_single_purpose_mac_vendor_ranks_below_the_name_and_above_server() {
        let with_mac = |mac: &str| {
            let mut ip = ip_address(&Uuid::new_v4(), &Uuid::new_v4());
            ip.base.mac_address = Some(MacEvidence::new(
                MacEvidenceValue(mac.parse().expect("valid MAC")),
                AttributeSource::ArpReply,
            ));
            ip
        };
        // Espressif and Nest Labs OUIs from a live scan.
        let espressif = [with_mac("0C:B8:15:01:44:10")];
        let nest = [with_mac("18:B4:30:CF:84:53")];
        assert_eq!(
            host_with_os(None).device_type(&[], &espressif),
            DeviceType::IoT
        );
        assert_eq!(host_with_os(None).device_type(&[], &nest), DeviceType::IoT);

        // Above `Server`: an application that implies a server does not outrank the chip vendor.
        assert_eq!(
            host_with_os(None).device_type(&[service(Box::new(PostgreSQL))], &espressif),
            DeviceType::IoT
        );
        // Below the OS and services.
        assert_eq!(
            host_with_os(Some(HostOs::family_only(HostOsFamily::MacOs)))
                .device_type(&[], &espressif),
            DeviceType::Workstation
        );
        assert_eq!(
            host_with_os(None).device_type(&[service(Box::new(TrueNAS))], &espressif),
            DeviceType::Nas
        );

        // A multi-product vendor (Google) and a randomized MAC imply nothing.
        for mac in ["B4:23:A2:59:5B:2C", "AA:26:68:D2:1D:1C"] {
            assert_eq!(
                host_with_os(None).device_type(&[], &[with_mac(mac)]),
                DeviceType::Unidentified,
                "{mac}"
            );
        }
    }

    #[test]
    fn windows_is_a_server_only_when_its_edition_or_name_says_so() {
        let windows = |name: Option<&str>, edition: Option<&str>| HostOs {
            name: name.map(str::to_string),
            edition: edition.map(str::to_string),
            ..HostOs::family_only(HostOsFamily::Windows)
        };
        assert_eq!(
            windows(Some("Windows Server 2022"), None).device_type(),
            Some(DeviceType::Server)
        );
        assert_eq!(
            windows(Some("Windows 11"), Some("Pro")).device_type(),
            Some(DeviceType::Workstation)
        );
        assert_eq!(windows(None, None).device_type(), None);
    }

    /// Every word of a device type's name is capitalized, matching service definition names, so
    /// an L3 group reads "Access Point" beside "Google Nest Router". A word with a capital anywhere
    /// in it (IoT) is spelled as written.
    #[test]
    fn every_word_of_a_device_type_name_is_capitalized() {
        use strum::IntoEnumIterator;

        let offenders: Vec<&'static str> = DeviceType::iter()
            .map(|device_type| device_type.name())
            .filter(|name| {
                name.split_whitespace().any(|word| {
                    let Some(first) = word.chars().next() else {
                        return false;
                    };
                    !(first.is_uppercase() || word.chars().any(char::is_uppercase))
                })
            })
            .collect();

        assert!(
            offenders.is_empty(),
            "device type names with an uncapitalized word: {offenders:?}"
        );
    }

    /// A fingerprinted device class sits between what the host's services declare and what its OS
    /// implies.
    #[test]
    fn a_matched_device_type_ranks_below_services_and_above_the_os() {
        use crate::server::hosts::r#impl::attributes::HostMatchedDeviceTypeValue;
        let matched = |os: Option<HostOs>| {
            let mut host = host_with_os(os);
            host.base.matched_device_type = Some(Attributed::new(
                HostMatchedDeviceTypeValue(DeviceType::Camera),
                AttributeSource::X509SubjectMatch,
            ));
            host
        };
        assert_eq!(
            matched(Some(HostOs::family_only(HostOsFamily::MacOs))).device_type(&[], []),
            DeviceType::Camera
        );
        assert_eq!(
            matched(None).device_type(&[service(Box::new(TrueNAS))], []),
            DeviceType::Nas
        );
        // And above `Server`, which any application implies.
        assert_eq!(
            matched(None).device_type(&[service(Box::new(PostgreSQL))], []),
            DeviceType::Camera
        );
    }

    #[test]
    fn no_two_device_types_share_an_icon() {
        use strum::IntoEnumIterator;
        let mut seen = std::collections::HashMap::new();
        let mut shared = Vec::new();
        for device_type in DeviceType::iter() {
            if let Some(other) = seen.insert(format!("{:?}", device_type.icon()), device_type) {
                shared.push((other, device_type));
            }
        }
        assert!(
            shared.is_empty(),
            "device types sharing an icon: {shared:?}"
        );
    }
}
