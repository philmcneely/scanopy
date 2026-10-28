use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct ManageEngineAccessManagerPlus;

impl ServiceDefinition for ManageEngineAccessManagerPlus {
    fn name(&self) -> &'static str {
        "ManageEngine Access Manager Plus"
    }
    fn description(&self) -> &'static str {
        "ManageEngine privileged remote access gateway"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::RemoteAccess
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("ManageEngine", "Access Manager Plus")], &[9443])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<ManageEngineAccessManagerPlus>
));
