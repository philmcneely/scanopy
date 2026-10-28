use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Metabase;

impl ServiceDefinition for Metabase {
    fn name(&self) -> &'static str {
        "Metabase"
    }
    fn description(&self) -> &'static str {
        "Business intelligence and analytics dashboards"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Dashboard
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Metabase", "Metabase")], &[3000])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Metabase>));
