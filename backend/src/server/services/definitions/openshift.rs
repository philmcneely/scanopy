use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct OpenShift;

impl ServiceDefinition for OpenShift {
    fn name(&self) -> &'static str {
        "OpenShift"
    }
    fn description(&self) -> &'static str {
        "Enterprise Kubernetes"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Orchestrator
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::Kubernetes, "/healthz", "openshift", None)
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::OpenShift)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<OpenShift>));
