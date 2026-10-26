use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::telnet::TelnetProbe;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{ClientProbe, Pattern};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Telnet;

impl ServiceDefinition for Telnet {
    fn name(&self) -> &'static str {
        "Telnet"
    }
    fn description(&self) -> &'static str {
        "Telnet remote access"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::RemoteAccess
    }
    /// Matches on the probe's answer, so a listener on this port that does not speak the protocol is
    /// not claimed as this service.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::ClientResponse(ClientProbe::Telnet)
    }
    fn app_probes(&self) -> Vec<Box<dyn AppProbe>> {
        vec![Box::new(TelnetProbe)]
    }
    fn is_generic(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Telnet>));
