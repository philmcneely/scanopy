use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct CloudPanel;

impl ServiceDefinition for CloudPanel {
    fn name(&self) -> &'static str {
        "CloudPanel"
    }
    fn description(&self) -> &'static str {
        "Server control panel"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Publishing
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("MGT-COMMERCE GmbH", "CloudPanel")], &[8443])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<CloudPanel>));
