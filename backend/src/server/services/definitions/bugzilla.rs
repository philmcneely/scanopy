use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Bugzilla;

impl ServiceDefinition for Bugzilla {
    fn name(&self) -> &'static str {
        "Bugzilla"
    }
    fn description(&self) -> &'static str {
        "Bug tracking system"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::ProjectManagement
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Mozilla", "Bugzilla")], &[80, 443])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Bugzilla>));
