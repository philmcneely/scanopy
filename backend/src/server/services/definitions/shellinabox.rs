use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct ShellInABox;

impl ServiceDefinition for ShellInABox {
    fn name(&self) -> &'static str {
        "Shell In A Box"
    }
    fn description(&self) -> &'static str {
        "Browser-based terminal"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::RemoteAccess
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("ShellInABox", "ShellInABox")], &[4200])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<ShellInABox>));
