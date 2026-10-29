use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct ElasticAPM;

impl ServiceDefinition for ElasticAPM {
    fn name(&self) -> &'static str {
        "Elastic APM"
    }
    fn description(&self) -> &'static str {
        "Application performance monitoring"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::new_tcp(8200), "/", "apm", None)
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::Elastic)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<ElasticAPM>));
