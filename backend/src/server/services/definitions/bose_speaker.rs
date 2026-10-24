use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{Pattern, Vendor};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct BoseSpeaker;

impl ServiceDefinition for BoseSpeaker {
    fn name(&self) -> &'static str {
        "Bose Speaker"
    }
    fn description(&self) -> &'static str {
        "Bose wireless speaker, soundbar or amplifier"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::IoT
    }
    fn device_type(&self) -> Option<DeviceType> {
        Some(DeviceType::Speaker)
    }

    // The ports a Bose Music Amplifier listens on: 80, Cast built-in on 8008/8009/8443 (the 8443
    // certificate is issued by "Bose CAST CA"), and 8085 and 8091 from Bose's own platform. Spotify Connect's port is left to that definition, which binds
    // it from the mDNS advertisement.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::MacVendor(Vendor::BOSE),
            Pattern::AnyOf(vec![
                Pattern::Port(PortType::Http),
                Pattern::Port(PortType::new_tcp(8008)),
                Pattern::Port(PortType::new_tcp(8009)),
                Pattern::Port(PortType::new_tcp(8085)),
                Pattern::Port(PortType::new_tcp(8091)),
                Pattern::Port(PortType::Https8443),
                // Every TLS port is signed by a Bose CA (`Bose CEPD Device CA` on the Wi-Fi setup
                // pages at 8082/8083 and on 8087, `Bose CAST CA` on 8443 and 10003).
                Pattern::CertificateIssuer("O=Bose Corporation", &[8082, 8083, 8087, 8443, 10003]),
            ]),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Bose)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<BoseSpeaker>));
