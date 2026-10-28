use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::docker_swarm::DockerSwarmProbe;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{ClientProbe, Pattern};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct DockerSwarm;

impl ServiceDefinition for DockerSwarm {
    fn name(&self) -> &'static str {
        "Docker Swarm"
    }
    fn description(&self) -> &'static str {
        "Docker native clustering and orchestration"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Orchestrator
    }

    /// Matches on the probe's answer. The raft port speaks mutual TLS, and the certificate the server
    /// presents before demanding ours carries the organizational unit Docker issues to swarm nodes
    /// — so this identifies a swarm, not merely a TLS listener on 2377.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::ClientResponse(ClientProbe::DockerSwarm)
    }
    fn app_probes(&self) -> Vec<Box<dyn AppProbe>> {
        vec![Box::new(DockerSwarmProbe)]
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Docker)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<DockerSwarm>));
