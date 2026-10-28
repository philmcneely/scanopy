use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Plesk;

impl ServiceDefinition for Plesk {
    fn name(&self) -> &'static str {
        "Plesk"
    }
    fn description(&self) -> &'static str {
        "Web hosting control panel"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Publishing
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Parallels", "Plesk")], &[8443, 8880])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Plesk>));
