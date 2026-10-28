use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct ApacheFlink;

impl ServiceDefinition for ApacheFlink {
    fn name(&self) -> &'static str {
        "Apache Flink"
    }
    fn description(&self) -> &'static str {
        "Stream processing framework"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Development
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Apache", "Flink")], &[8081])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<ApacheFlink>));
