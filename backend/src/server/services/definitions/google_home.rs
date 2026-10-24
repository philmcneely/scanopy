use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::chromecast::{CAST_AUDIO_CA, CAST_CA, CAST_DEVICE_CA};
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{DnsSdServiceType, Pattern, Vendor};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct GoogleHome;

impl ServiceDefinition for GoogleHome {
    fn name(&self) -> &'static str {
        "Google Home"
    }

    fn description(&self) -> &'static str {
        "Google Home smart speaker or display"
    }

    fn category(&self) -> ServiceCategory {
        ServiceCategory::IoT
    }
    fn device_type(&self) -> Option<DeviceType> {
        Some(DeviceType::Speaker)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::AnyOf(vec![
                Pattern::MacVendor(Vendor::NEST),
                Pattern::MacVendor(Vendor::GOOGLE),
            ]),
            // Google Home shares its OUI and its Cast port pair with Chromecast, so on their own
            // these two definitions are indistinguishable. The mDNS TXT `md=` model string is what
            // tells them apart in practice; until a pattern can read TXT values, the port pair
            // stays the primary signal and the service type only widens reach.
            //
            // The Cast device certificate does tell them apart: an `Audio Assist` CA is a speaker,
            // and a device CA without it is a video receiver, which is Chromecast's.
            Pattern::Not(Box::new(Pattern::AllOf(vec![
                Pattern::CertificateIssuer(CAST_DEVICE_CA),
                Pattern::Not(Box::new(Pattern::CertificateIssuer(CAST_AUDIO_CA))),
            ]))),
            Pattern::AnyOf(vec![
                Pattern::AllOf(vec![
                    Pattern::Port(PortType::new_tcp(8008)),
                    Pattern::Port(PortType::new_tcp(8009)),
                ]),
                Pattern::DnsSd(DnsSdServiceType::GOOGLE_CAST, None),
                Pattern::CertificateIssuer(CAST_AUDIO_CA),
            ])
            .also_binding(Pattern::CertificateIssuer(CAST_CA)),
        ])
    }

    fn logo_url(&self) -> &'static str {
        "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/svg/google-home.svg"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<GoogleHome>));
