use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct ManageEnginePam360;

impl ServiceDefinition for ManageEnginePam360 {
    fn name(&self) -> &'static str {
        "ManageEngine PAM360"
    }
    fn description(&self) -> &'static str {
        "ManageEngine privileged access and password management"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::IdentityAndAccess
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(
            &[
                ("ManageEngine", "PAM360"),
                ("ManageEngine", "ManageEngine Password Manager Pro"),
            ],
            &[7272, 8282],
        )
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<ManageEnginePam360>
));
