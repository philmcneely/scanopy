use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Greenbone;

impl ServiceDefinition for Greenbone {
    fn name(&self) -> &'static str {
        "Greenbone Security Assistant"
    }
    fn description(&self) -> &'static str {
        "Greenbone OpenVAS vulnerability scanner"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Greenbone", "Security Scanner")], &[443, 9392])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Greenbone>));
