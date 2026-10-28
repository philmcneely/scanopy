//! Which device type a Recog `hw.device` value names.

use crate::server::hosts::r#impl::device_type::DeviceType;

/// How a Recog `hw.device` value reads as a device type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RecogDevice {
    /// The value names a class of device.
    Type(DeviceType),
    /// The value names no class ("Device", "Appliance"). A match still names the vendor and model.
    Generic,
    /// The value is in neither list. The vendored data must contain none
    /// (`every_vendored_device_is_classified`).
    Unlisted,
}

/// Classify a Recog `hw.device`. One device type per value Recog uses, so a device keeps the
/// granularity Recog gives it; only spellings of one class share a variant.
pub(super) fn classify_device(device: &str) -> RecogDevice {
    use DeviceType as D;
    let device_type = match device {
        "Router" => D::Router,
        "Switch" => D::Switch,
        "Firewall" => D::Firewall,
        "Printer" => D::Printer,
        "Tablet" => D::Tablet,
        "Hypervisor" => D::Hypervisor,
        "Storage" => D::Storage,
        "Media Player" => D::MediaPlayer,
        "WAP" => D::AccessPoint,
        "IP Camera" => D::Camera,
        "Mobile Phone" => D::Mobile,
        // Recog files desk phones under VoIP.
        "VoIP" => D::Phone,
        "Industrial Control" => D::Industrial,
        "Laptop" => D::Laptop,
        "Desktop" => D::Desktop,
        "Broadband Router" => D::BroadbandRouter,
        "Cable Modem" | "DOCSIS Cable Modem" => D::CableModem,
        "DSL Modem" | "ADSL Modem" => D::DslModem,
        "Wireless Controller" => D::WirelessController,
        "WLAN Repeater" => D::WlanRepeater,
        "Powerline" => D::Powerline,
        "SD-WAN Appliance" => D::SdWanAppliance,
        "VPN" => D::Vpn,
        "Security Appliance" => D::SecurityAppliance,
        "IPS" => D::Ips,
        "Network Management Device" => D::NetworkManagementDevice,
        "Multiplexer" => D::Multiplexer,
        "Telecom" => D::Telecom,
        "Ethernet Adapter" => D::EthernetAdapter,
        "Device Server" => D::DeviceServer,
        "KVM" => D::Kvm,
        "Device Hub" => D::DeviceHub,
        "VoIP Gateway" => D::VoipGateway,
        "SIP Gateway" => D::SipGateway,
        "Lights Out Management" => D::LightsOutManagement,
        "NAS" => D::Nas,
        "Storage Appliance" => D::StorageAppliance,
        "Tape Library" => D::TapeLibrary,
        "Support Appliance" => D::SupportAppliance,
        "Monitoring" => D::Monitoring,
        "VoIP Server" => D::VoipServer,
        "Voice Appliance" => D::VoiceAppliance,
        "Media Server" => D::MediaServer,
        "Multifunction Device" => D::MultifunctionDevice,
        "Print Server" => D::PrintServer,
        "Scanner" => D::Scanner,
        "Handheld Scanner" => D::HandheldScanner,
        "Check Scanner" => D::CheckScanner,
        "DVR" => D::Dvr,
        "Cloud Network Video Recorder" => D::NetworkVideoRecorder,
        "Web Cam" => D::WebCam,
        "Video Encoder" => D::VideoEncoder,
        "Video Decoder" => D::VideoDecoder,
        "Video Conferencing" => D::VideoConferencing,
        "Display Controller" => D::DisplayController,
        "Wireless Presenter" => D::WirelessPresenter,
        "Whiteboard" => D::Whiteboard,
        "Smart TV" => D::SmartTv,
        "IPTV" => D::Iptv,
        "Media Receiver" => D::MediaReceiver,
        "AV Receiver" => D::AvReceiver,
        "Network Audio" => D::NetworkAudio,
        "Audio Encoder" => D::AudioEncoder,
        "Power Device" => D::PowerDevice,
        "UPS" => D::Ups,
        "Power Relay" => D::PowerRelay,
        "Power Meter" => D::PowerMeter,
        "Relay Controller" => D::RelayController,
        "Building Automation" => D::BuildingAutomation,
        "Environment Control" => D::EnvironmentControl,
        "Access Control" => D::AccessControl,
        "Alarm Panel" => D::AlarmPanel,
        "Sensor" => D::Sensor,
        "Light Bulb" => D::LightBulb,
        "PLC" => D::Plc,
        "HMI Controller" => D::HmiController,
        "Test Instrument" => D::TestInstrument,
        "JTAG Adapter" => D::JtagAdapter,
        "Data Terminal" => D::DataTerminal,
        "Medical" => D::Medical,
        "Device" | "Appliance" | "Networking" | "Network Appliance" => {
            return RecogDevice::Generic;
        }
        _ => return RecogDevice::Unlisted,
    };
    RecogDevice::Type(device_type)
}
