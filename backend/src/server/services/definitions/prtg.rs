use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Prtg;

impl ServiceDefinition for Prtg {
    fn name(&self) -> &'static str {
        "PRTG Network Monitor"
    }
    fn description(&self) -> &'static str {
        "Paessler PRTG network monitoring"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("PRTG", "Network Monitor")], &[80, 443])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Prtg>));
