use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct ManageEngineOpManager;

impl ServiceDefinition for ManageEngineOpManager {
    fn name(&self) -> &'static str {
        "ManageEngine OpManager"
    }
    fn description(&self) -> &'static str {
        "ManageEngine network monitoring"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("ManageEngine", "OpManager")], &[443, 8060])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<ManageEngineOpManager>
));
