use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct APC;

impl ServiceDefinition for APC {
    fn name(&self) -> &'static str {
        "APC"
    }
    fn description(&self) -> &'static str {
        "APC Network-Connected UPS"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Is(DeviceType::Ups)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::Http, "/", "Schneider Electric", None)
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Apc)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<APC>));
