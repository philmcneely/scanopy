use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Nessus;

impl ServiceDefinition for Nessus {
    fn name(&self) -> &'static str {
        "Tenable Nessus"
    }
    fn description(&self) -> &'static str {
        "Tenable Nessus vulnerability scanner"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Tenable", "Nessus")], &[8834])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Nessus>));
