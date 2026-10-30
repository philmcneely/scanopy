use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct OracleSecureGlobalDesktop;

impl ServiceDefinition for OracleSecureGlobalDesktop {
    fn name(&self) -> &'static str {
        "Oracle Secure Global Desktop"
    }
    fn description(&self) -> &'static str {
        "Oracle remote application access gateway"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::RemoteAccess
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Nothing
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Oracle", "Secure Global Desktop")], &[443])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Oracle)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<OracleSecureGlobalDesktop>
));
