use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct LibreNms;

impl ServiceDefinition for LibreNms {
    fn name(&self) -> &'static str {
        "LibreNMS"
    }
    fn description(&self) -> &'static str {
        "Network monitoring system"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("LibreNMS", "LibreNMS")], &[80, 443])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<LibreNms>));
