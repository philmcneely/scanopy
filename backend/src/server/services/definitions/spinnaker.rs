use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Spinnaker;

impl ServiceDefinition for Spinnaker {
    fn name(&self) -> &'static str {
        "Spinnaker"
    }
    fn description(&self) -> &'static str {
        "Multi-cloud CD platform"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Development
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::None
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::Spinnaker)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Spinnaker>));
