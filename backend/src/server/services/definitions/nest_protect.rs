use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::chromecast::{CAST_CA, CAST_PORTS};
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{Pattern, Vendor};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct NestProtect;

impl ServiceDefinition for NestProtect {
    fn name(&self) -> &'static str {
        "Nest Protect"
    }

    fn description(&self) -> &'static str {
        "Google Nest smoke and CO detector"
    }

    fn category(&self) -> ServiceCategory {
        ServiceCategory::IoT
    }
    fn device_type(&self) -> Option<DeviceType> {
        Some(DeviceType::Sensor)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::AnyOf(vec![
                Pattern::MacVendor(Vendor::NEST),
                Pattern::MacVendor(Vendor::GOOGLE),
            ]),
            Pattern::Port(PortType::new_tcp(11095)),
        ])
        // Its Home services on 8012 and 10101 present certificates under Google's Cast CAs.
        .also_binding(Pattern::CertificateIssuer(CAST_CA, CAST_PORTS))
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::GoogleHome)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<NestProtect>));
