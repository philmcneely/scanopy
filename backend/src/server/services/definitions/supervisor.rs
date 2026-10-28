use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Supervisor;

impl ServiceDefinition for Supervisor {
    fn name(&self) -> &'static str {
        "Supervisor"
    }
    fn description(&self) -> &'static str {
        "Supervisor process control web interface"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Supervisord", "Supervisor")], &[9001])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Supervisor>));
