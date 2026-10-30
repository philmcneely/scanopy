use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Fortinet;

impl ServiceDefinition for Fortinet {
    fn name(&self) -> &'static str {
        "Fortinet"
    }
    fn description(&self) -> &'static str {
        "Fortinet security appliance"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkAppliance
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Is(DeviceType::Firewall)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::Http, "/login", "fortinet", None)
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Fortinet)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Fortinet>));
