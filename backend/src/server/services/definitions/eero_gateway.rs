use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{Pattern, Vendor};
use crate::server::shared::logos::Logo;

/// The ports eero units open beyond DNS, NTP and DHCP, seen on a gateway and a mesh point both. eero
/// documents neither. 3001 speaks TLS under a certificate the unit issues itself, `O=eero`. 10101
/// speaks nothing a probe can read, and IANA's registration of it (`ezmeeting-2`) is unrelated.
/// Optional, so they are bound beside the vendor's MAC and never stand as evidence alone.
pub(crate) fn eero_ports() -> Pattern<'static> {
    Pattern::Optional(Box::new(Pattern::AnyOf(vec![
        Pattern::CertificateIssuer("O=eero", &[3001]),
        Pattern::Port(PortType::new_tcp(10101)),
    ])))
}

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct EeroGateway;

impl ServiceDefinition for EeroGateway {
    fn name(&self) -> &'static str {
        "Eero Gateway"
    }
    fn description(&self) -> &'static str {
        "Eero device providing routing and gateway services"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkAccess
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Is(DeviceType::Router)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::MacVendor(Vendor::EERO),
            Pattern::IsGateway,
            eero_ports(),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Eero)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<EeroGateway>));
