use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct ApacheSolr;

impl ServiceDefinition for ApacheSolr {
    fn name(&self) -> &'static str {
        "Apache Solr"
    }
    fn description(&self) -> &'static str {
        "Search platform"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Database
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Apache", "Solr")], &[8983])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<ApacheSolr>));
