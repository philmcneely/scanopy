use crate::server::hosts::r#impl::base::Host;
use crate::server::ip_addresses::r#impl::base::{IPAddress, mac_of};
use crate::server::services::r#impl::base::Service;
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::{
    DeviceTypeImplication, ServiceDefinition, ServiceDefinitionExt, VirtualizationRole,
};
use crate::server::services::r#impl::patterns::Vendor;
use crate::server::shared::attribution::AttributeSource;
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
    /// 1. The highest-precedence device type the host's own services say it is
    ///    ([`DeviceTypeImplication::Is`]). A matched definition names the product the host runs, so
    ///    it outranks the OS.
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
    /// 8. The highest-precedence device type its services suggest
    ///    ([`DeviceTypeImplication::Suggests`]): Spotify Connect and AirPlay run on receivers and
    ///    speakers, and on desktops and servers too, so they decide only a host nothing else names.
    /// 9. `Unidentified`.
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
        self.device_type_with_source(services, ip_addresses).0
    }

    /// [`Host::device_type`], with the rung that decided it.
    pub fn device_type_with_source<'a, 'b>(
        &self,
        services: impl IntoIterator<Item = &'a Service>,
        ip_addresses: impl IntoIterator<Item = &'b IPAddress>,
    ) -> (DeviceType, HostDeviceTypeSource) {
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

        let implications: Vec<(DeviceTypeImplication, &Service)> = own
            .iter()
            .map(|s| (s.base.service_definition.device_type(), *s))
            .collect();
        let highest = |strength: fn(DeviceTypeImplication) -> Option<DeviceType>| {
            implications
                .iter()
                .filter_map(|(implication, service)| Some((strength(*implication)?, *service)))
                .min_by_key(|(device_type, _)| *device_type)
        };
        let from_service = |service: &Service| HostDeviceTypeSource::Service {
            service_id: service.id,
            service_name: service.base.name.clone(),
        };

        if let Some((device_type, service)) = highest(|implication| match implication {
            DeviceTypeImplication::Is(device_type) => Some(device_type),
            _ => None,
        }) {
            return (device_type, from_service(service));
        }
        if let Some(matched) = self
            .base
            .matched_device_type
            .as_ref()
            .filter(|matched| matched.value().0 != DeviceType::Unidentified)
        {
            return (
                matched.value().0,
                HostDeviceTypeSource::Matched {
                    source: matched.source(),
                },
            );
        }
        if let Some((device_type, os)) = self
            .base
            .os
            .as_ref()
            .and_then(|os| Some((os.value().0.device_type()?, os)))
        {
            return (
                device_type,
                HostDeviceTypeSource::OperatingSystem {
                    source: os.source(),
                },
            );
        }
        if let Some(named) = self.named_device_type() {
            return named;
        }
        if let Some(device_type) = ip_addresses
            .into_iter()
            .filter_map(|ip| mac_of(&ip.base.mac_address))
            .find_map(|mac| device_type_from_mac(&mac.to_string()))
        {
            return (device_type, HostDeviceTypeSource::MacVendor);
        }
        if let Some((_, service)) = implications
            .iter()
            .find(|(implication, _)| *implication == DeviceTypeImplication::Server)
        {
            return (DeviceType::Server, from_service(service));
        }
        if runs_containers {
            return (DeviceType::Server, HostDeviceTypeSource::Containers);
        }
        if let Some((device_type, service)) = highest(|implication| match implication {
            DeviceTypeImplication::Suggests(device_type) => Some(device_type),
            _ => None,
        }) {
            return (device_type, from_service(service));
        }
        (DeviceType::Unidentified, HostDeviceTypeSource::NoEvidence)
    }

    /// The device type the hostname or name names by product, for products that name themselves.
    fn named_device_type(&self) -> Option<(DeviceType, HostDeviceTypeSource)> {
        let hostname = self.base.hostname.as_ref().and_then(|h| {
            Some((
                device_type_from_label(&h.value().0)?,
                HostDeviceTypeSource::Hostname,
            ))
        });
        hostname.or_else(|| {
            Some((
                device_type_from_label(&self.base.name.value().as_str())?,
                HostDeviceTypeSource::Name,
            ))
        })
    }
}

/// Which rung of [`Host::device_type`] decided a host's device type, and the evidence on it.
///
/// Only `Matched` is a stored value with a source of its own. Every other rung is derived from
/// other fields each time the host is read, so the UI says which ones instead of naming a source.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type")]
pub enum HostDeviceTypeSource {
    /// A service on the host declares the device type, or implies `Server`.
    #[schema(title = "Service")]
    Service {
        /// The service whose definition declares or implies the device type.
        service_id: uuid::Uuid,
        /// That service's name, for display.
        service_name: String,
    },
    /// A fingerprint of something the host emitted, stored in `matched_device_type`.
    #[schema(title = "Matched")]
    Matched { source: AttributeSource },
    /// The host's OS implies it. `source` is where the OS came from.
    #[schema(title = "OperatingSystem")]
    OperatingSystem { source: AttributeSource },
    /// The host's hostname names a product.
    #[schema(title = "Hostname")]
    Hostname,
    /// The host's name names a product.
    #[schema(title = "Name")]
    Name,
    /// The maker of one of the host's network adapters makes only this type of device.
    #[schema(title = "MacVendor")]
    MacVendor,
    /// The host runs containers.
    #[schema(title = "Containers")]
    Containers,
    /// Nothing on the host names what it is. Also what a reader makes of a rung it does not know.
    #[default]
    #[serde(other)]
    #[schema(title = "NoEvidence")]
    NoEvidence,
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
mod tests;
