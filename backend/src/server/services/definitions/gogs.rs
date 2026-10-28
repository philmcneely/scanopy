use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Gogs;

impl ServiceDefinition for Gogs {
    fn name(&self) -> &'static str {
        "Gogs"
    }
    fn description(&self) -> &'static str {
        "Self-hosted Git service"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Development
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Gogs", "Gogs")], &[3000])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Gogs>));
