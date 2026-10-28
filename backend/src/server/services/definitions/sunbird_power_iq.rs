use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct PowerIq;

impl ServiceDefinition for PowerIq {
    fn name(&self) -> &'static str {
        "Power IQ"
    }
    fn description(&self) -> &'static str {
        "Sunbird Power IQ data center power management"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Genscape", "Power IQ")], &[443])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<PowerIq>));
