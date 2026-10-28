use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct ManageEngineAdAuditPlus;

impl ServiceDefinition for ManageEngineAdAuditPlus {
    fn name(&self) -> &'static str {
        "ManageEngine ADAudit Plus"
    }
    fn description(&self) -> &'static str {
        "ManageEngine Active Directory change auditing"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::IdentityAndAccess
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("ManageEngine", "ADAudit Plus")], &[8081])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<ManageEngineAdAuditPlus>
));
