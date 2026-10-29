use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Rundeck;

impl ServiceDefinition for Rundeck {
    fn name(&self) -> &'static str {
        "Rundeck"
    }
    fn description(&self) -> &'static str {
        "Runbook automation"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Development
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(
            &[("Rundeck", "Rundeck"), ("Rundeck", "Rundeck Enterprise")],
            &[4440],
        )
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Rundeck>));
