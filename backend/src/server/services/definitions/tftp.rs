use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::tftp::TftpProbe;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{ClientProbe, Pattern};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Tftp;

impl ServiceDefinition for Tftp {
    fn name(&self) -> &'static str {
        "TFTP Server"
    }
    fn description(&self) -> &'static str {
        "Trivial file transfer, used to load device firmware and configuration"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkCore
    }

    /// Matches on the probe's answer, so a container that only declares 69/udp is not claimed as this
    /// service.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::ClientResponse(ClientProbe::Tftp)
    }
    fn app_probes(&self) -> Vec<Box<dyn AppProbe>> {
        vec![Box::new(TftpProbe)]
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Tftp>));
