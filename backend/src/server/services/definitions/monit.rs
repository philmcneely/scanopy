use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Monit;

impl ServiceDefinition for Monit {
    fn name(&self) -> &'static str {
        "Monit"
    }
    fn description(&self) -> &'static str {
        "Process and service monitoring"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Tildeslash", "Monit")], &[2812])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Monit>));
