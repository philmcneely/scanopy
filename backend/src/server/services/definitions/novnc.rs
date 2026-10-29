use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct NoVnc;

impl ServiceDefinition for NoVnc {
    fn name(&self) -> &'static str {
        "noVNC"
    }
    fn description(&self) -> &'static str {
        "Browser-based VNC client"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::RemoteAccess
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Nothing
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("noVNC", "noVNC")], &[6080])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<NoVnc>));
