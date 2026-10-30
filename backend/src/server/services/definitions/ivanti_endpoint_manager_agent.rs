use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct IvantiEndpointManagerAgent;

impl ServiceDefinition for IvantiEndpointManagerAgent {
    fn name(&self) -> &'static str {
        "Ivanti Endpoint Manager Agent"
    }
    fn description(&self) -> &'static str {
        "Ivanti (LANDesk) endpoint management agent"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("LANDesk", "Management Agent")], &[9595])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Ivanti)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<IvantiEndpointManagerAgent>
));
