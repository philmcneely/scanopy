use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{DnsSdServiceType, Pattern, Vendor};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct ChromecastDevice;

impl ServiceDefinition for ChromecastDevice {
    fn name(&self) -> &'static str {
        "Chromecast"
    }

    fn description(&self) -> &'static str {
        "Google Chromecast streaming device"
    }

    fn category(&self) -> ServiceCategory {
        ServiceCategory::IoT
    }
    fn device_type(&self) -> Option<DeviceType> {
        Some(DeviceType::MediaPlayer)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        // The OUI is shared with every other Google device, and the port pair with Google Home,
        // so the mDNS arm is what actually separates a Chromecast from a speaker — it is the
        // service type only a Cast receiver advertises. Kept as an alternative rather than a
        // requirement so a Chromecast on a routed subnet, where multicast never reaches, is still
        // found by its ports.
        //
        // A Cast receiver's certificates settle what the ports cannot: the device CA names the
        // product line, so an `Audio Assist` CA (a Nest Mini or Nest Audio) is Google Home's, not
        // this definition's. Every TLS port signed under Google's Cast CAs is the receiver's, on
        // whatever numbers this unit opened.
        Pattern::AllOf(vec![
            Pattern::MacVendor(Vendor::GOOGLE),
            Pattern::Not(Box::new(Pattern::CertificateIssuer(CAST_AUDIO_CA))),
            Pattern::AnyOf(vec![
                Pattern::DnsSd(DnsSdServiceType::GOOGLE_CAST, None),
                Pattern::AllOf(vec![
                    Pattern::Port(PortType::new_tcp(8008)),
                    Pattern::Port(PortType::new_tcp(8009)),
                ]),
                Pattern::CertificateIssuer(CAST_DEVICE_CA),
            ])
            .also_binding(Pattern::CertificateIssuer(CAST_CA)),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::GoogleCast)
    }
}

/// The organizational unit of Google's Cast CAs, which sign every TLS port on a Cast receiver and
/// on Nest devices: `Chromecast ICA …` for the device's own identity, `Usonia …` for the Home
/// services it runs (8012, 10101 and others).
pub(crate) const CAST_CA: &str = "OU=Cast";
/// The CA that signs a Cast receiver's device certificate, e.g. `Chromecast ICA 5 (4K)`.
pub(crate) const CAST_DEVICE_CA: &str = "Chromecast ICA";
/// The device CA of audio-only Cast receivers, e.g. `Chromecast ICA 16 (Audio Assist 4)` on a
/// Nest Mini.
pub(crate) const CAST_AUDIO_CA: &str = "Audio Assist";

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<ChromecastDevice>
));
