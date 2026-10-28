use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Moodle;

impl ServiceDefinition for Moodle {
    fn name(&self) -> &'static str {
        "Moodle"
    }
    fn description(&self) -> &'static str {
        "Learning management system"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Publishing
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Moodle", "Moodle")], &[80, 443])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Moodle>));
