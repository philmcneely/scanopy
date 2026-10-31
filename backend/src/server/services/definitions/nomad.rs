use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Nomad;

impl ServiceDefinition for Nomad {
    fn name(&self) -> &'static str {
        "Nomad"
    }
    fn description(&self) -> &'static str {
        "Workload orchestration"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Orchestrator
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        // The leader endpoint returns the leader's RPC address, on 4647 unless reconfigured.
        Pattern::Endpoint(PortType::new_tcp(4646), "/v1/status/leader", ":4647", None)
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::Nomad)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Nomad>));
