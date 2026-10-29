use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::docker_ping::DockerPingProbe;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{ClientProbe, Pattern};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Docker;

impl ServiceDefinition for Docker {
    fn name(&self) -> &'static str {
        "Docker"
    }
    fn description(&self) -> &'static str {
        "Docker"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::ContainerRuntime
    }

    /// Either a credential reached the daemon, or an Engine API on plain TCP 2375 answered
    /// `/_ping` without one. The second is what lets a scan prompt for a Docker credential.
    ///
    /// Also binds the daemon's TLS API on 2376 once the daemon answered. Under `--tlsverify` it
    /// accepts no request without a client certificate, so nothing there can be read unauthenticated.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::AnyOf(vec![
                Pattern::ClientResponse(ClientProbe::Docker),
                Pattern::ClientResponse(ClientProbe::DockerPing),
            ]),
            Pattern::Optional(Box::new(Pattern::Port(PortType::DockerTls))),
        ])
    }
    fn app_probes(&self) -> Vec<Box<dyn AppProbe>> {
        vec![Box::new(DockerPingProbe)]
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Docker)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Docker>));
