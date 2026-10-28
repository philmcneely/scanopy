use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Cacti;

impl ServiceDefinition for Cacti {
    fn name(&self) -> &'static str {
        "Cacti"
    }
    fn description(&self) -> &'static str {
        "Network graphing and monitoring"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Cacti", "Cacti")], &[80, 443])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Cacti>));
