use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct OracleWebLogic;

impl ServiceDefinition for OracleWebLogic {
    fn name(&self) -> &'static str {
        "Oracle WebLogic"
    }
    fn description(&self) -> &'static str {
        "Oracle WebLogic Java application server"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Development
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("BEA", "WebLogic")], &[7001])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Oracle)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<OracleWebLogic>
));
