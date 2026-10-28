use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct DellOpenManage;

impl ServiceDefinition for DellOpenManage {
    fn name(&self) -> &'static str {
        "Dell OpenManage"
    }
    fn description(&self) -> &'static str {
        "Dell OpenManage server management"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Dell", "OpenManage")], &[1311, 443])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<DellOpenManage>
));
