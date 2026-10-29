use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct IgnitionGateway;

impl ServiceDefinition for IgnitionGateway {
    fn name(&self) -> &'static str {
        "Ignition Gateway"
    }
    fn description(&self) -> &'static str {
        "Inductive Automation Ignition SCADA platform"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Industrial
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(
            &[("Inductive Automation", "Ignition Gateway")],
            &[8043, 8088],
        )
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<IgnitionGateway>
));
