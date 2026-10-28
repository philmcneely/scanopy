use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{Pattern, Vendor};
use crate::server::shared::logos::Logo;

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
    fn device_type(&self) -> Option<DeviceType> {
        Some(DeviceType::Router)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![Pattern::MacVendor(Vendor::EERO), Pattern::IsGateway])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Eero)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<EeroGateway>));
