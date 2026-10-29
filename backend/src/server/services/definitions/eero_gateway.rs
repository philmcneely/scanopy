use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{Pattern, Vendor};
use crate::server::shared::logos::Logo;

/// A TCP port eero units open, seen on a gateway and a mesh point both. eero does not document it,
/// and IANA's registration of 10101 (`ezmeeting-2`) is unrelated, so it is bound alongside the
/// vendor's MAC and never stands as evidence alone.
pub(crate) const EERO_PORT: u16 = 10101;

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
        Pattern::AllOf(vec![Pattern::MacVendor(Vendor::EERO), Pattern::IsGateway])
            .also_binding(Pattern::Port(PortType::new_tcp(EERO_PORT)))
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Eero)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<EeroGateway>));
