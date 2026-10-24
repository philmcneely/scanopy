use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{DnsSdServiceType, Pattern, Vendor};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct SonosSpeaker;

impl ServiceDefinition for SonosSpeaker {
    fn name(&self) -> &'static str {
        "Sonos Speaker"
    }

    fn description(&self) -> &'static str {
        "Sonos wireless speaker system"
    }

    fn category(&self) -> ServiceCategory {
        ServiceCategory::IoT
    }
    fn device_type(&self) -> Option<DeviceType> {
        Some(DeviceType::Speaker)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        // Sonos speakers have very distinctive port signature:
        // TCP 1400 (HTTP API), 1443 (HTTPS API), 4444 (control)
        Pattern::AllOf(vec![
            Pattern::MacVendor(Vendor::SONOS),
            Pattern::AnyOf(vec![
                Pattern::DnsSd(DnsSdServiceType::SONOS, None),
                Pattern::Port(PortType::Samba),
                Pattern::Port(PortType::new_tcp(3445)),
                Pattern::Port(PortType::new_tcp(1400)),
                Pattern::Port(PortType::new_tcp(1410)),
                Pattern::Port(PortType::new_tcp(1843)),
                Pattern::Port(PortType::new_tcp(3400)),
                Pattern::Port(PortType::new_tcp(3401)),
                Pattern::Port(PortType::new_tcp(3500)),
                // The HTTPS API on 1443 presents a certificate under `Sonos Device Authentication
                // Root CA`.
                Pattern::CertificateIssuer("O=Sonos", &[1443]),
            ]),
        ])
    }

    fn logo_url(&self) -> &'static str {
        "https://simpleicons.org/icons/sonos.svg"
    }

    fn logo_needs_white_background(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<SonosSpeaker>
));
