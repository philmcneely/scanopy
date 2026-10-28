use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Rapid7InsightVm;

impl ServiceDefinition for Rapid7InsightVm {
    fn name(&self) -> &'static str {
        "Rapid7 InsightVM"
    }
    fn description(&self) -> &'static str {
        "Rapid7 InsightVM and Nexpose vulnerability management"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Rapid7", "InsightVM"), ("Rapid7", "Nexpose")], &[3780])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<Rapid7InsightVm>
));
