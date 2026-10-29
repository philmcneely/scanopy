use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Wso2;

impl ServiceDefinition for Wso2 {
    fn name(&self) -> &'static str {
        "WSO2"
    }
    fn description(&self) -> &'static str {
        "WSO2 API manager, integrator and identity server"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Integration
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(
            &[
                ("WSO2", "API Manager"),
                ("WSO2", "Carbon"),
                ("WSO2", "Enterprise Integrator"),
                ("WSO2", "Identity Server"),
            ],
            &[9443],
        )
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Wso2>));
