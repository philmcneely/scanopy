use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Tasmota;

impl ServiceDefinition for Tasmota {
    fn name(&self) -> &'static str {
        "Tasmota"
    }
    fn description(&self) -> &'static str {
        "ESP device firmware"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::IoT
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::None
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::Tasmota)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Tasmota>));
