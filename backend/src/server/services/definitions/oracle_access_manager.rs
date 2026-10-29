use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct OracleAccessManager;

impl ServiceDefinition for OracleAccessManager {
    fn name(&self) -> &'static str {
        "Oracle Access Manager"
    }
    fn description(&self) -> &'static str {
        "Oracle single sign-on and access management"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::IdentityAndAccess
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Oracle", "Access Manager")], &[443])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Oracle)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<OracleAccessManager>
));
