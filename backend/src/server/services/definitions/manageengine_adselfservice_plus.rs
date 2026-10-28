use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct ManageEngineAdSelfServicePlus;

impl ServiceDefinition for ManageEngineAdSelfServicePlus {
    fn name(&self) -> &'static str {
        "ManageEngine ADSelfService Plus"
    }
    fn description(&self) -> &'static str {
        "ManageEngine self-service password reset"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::IdentityAndAccess
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("ManageEngine", "ADSelfService Plus")], &[8888, 9251])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<ManageEngineAdSelfServicePlus>
));
