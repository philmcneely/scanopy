use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Elasticsearch;

impl ServiceDefinition for Elasticsearch {
    fn name(&self) -> &'static str {
        "Elasticsearch"
    }
    fn description(&self) -> &'static str {
        "Distributed search and analytics engine"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Database
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::Elasticsearch, "/", "lucene", None)
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::Elasticsearch)
    }
    fn is_generic(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<Elasticsearch>
));
