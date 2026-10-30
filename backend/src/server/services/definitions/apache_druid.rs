use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct ApacheDruid;

impl ServiceDefinition for ApacheDruid {
    fn name(&self) -> &'static str {
        "Apache Druid"
    }
    fn description(&self) -> &'static str {
        "Real-time analytics database"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Database
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Apache", "Druid")], &[8888])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::ApacheDruid)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<ApacheDruid>));
