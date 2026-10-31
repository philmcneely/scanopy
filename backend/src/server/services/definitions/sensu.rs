use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Sensu;

impl ServiceDefinition for Sensu {
    fn name(&self) -> &'static str {
        "Sensu"
    }
    fn description(&self) -> &'static str {
        "Monitoring framework"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::new_tcp(4567), "/health", "sensu", None)
    }
    fn is_generic(&self) -> bool {
        true
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::Sensu)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Sensu>));
