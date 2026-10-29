use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct KubernetesDashboard;

impl ServiceDefinition for KubernetesDashboard {
    fn name(&self) -> &'static str {
        "Kubernetes Dashboard"
    }
    fn description(&self) -> &'static str {
        "Kubernetes web UI"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Orchestrator
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Kubernetes", "Dashboard")], &[443, 8443])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Kubernetes)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<KubernetesDashboard>
));
