use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Adminer;

impl ServiceDefinition for Adminer {
    fn name(&self) -> &'static str {
        "Adminer"
    }
    fn description(&self) -> &'static str {
        "Database web administration"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Database
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Adminer", "Adminer")], &[80, 8080])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Adminer>));
