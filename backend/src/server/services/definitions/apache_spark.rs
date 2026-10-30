use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct ApacheSpark;

impl ServiceDefinition for ApacheSpark {
    fn name(&self) -> &'static str {
        "Apache Spark"
    }
    fn description(&self) -> &'static str {
        "Cluster computing engine"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Development
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Apache", "Spark")], &[4040, 8080])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::ApacheSpark)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<ApacheSpark>));
