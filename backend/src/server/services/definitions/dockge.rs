use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Dockge;

impl ServiceDefinition for Dockge {
    fn name(&self) -> &'static str {
        "Dockge"
    }
    fn description(&self) -> &'static str {
        "Docker compose stack management UI"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Orchestrator
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::new_tcp(5001), "/", "Dockge", None)
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Dockge)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Dockge>));
