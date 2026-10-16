use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Consul;

impl ServiceDefinition for Consul {
    fn name(&self) -> &'static str {
        "Consul"
    }
    fn description(&self) -> &'static str {
        "HashiCorp service discovery and service mesh"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Orchestrator
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::new_tcp(8500), "/ui/", "Consul by HashiCorp", None)
    }
    fn logo_url(&self) -> &'static str {
        "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/svg/consul.svg"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Consul>));
