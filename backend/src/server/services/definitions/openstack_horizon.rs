use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct OpenStackHorizon;

impl ServiceDefinition for OpenStackHorizon {
    fn name(&self) -> &'static str {
        "OpenStack Horizon"
    }
    fn description(&self) -> &'static str {
        "OpenStack dashboard"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Orchestrator
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(
            &[("OpenStack", "Horizon"), ("OpenStack", "Dashboard")],
            &[80, 443],
        )
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::OpenStack)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<OpenStackHorizon>
));
