use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::checkmk::CheckMkAgentProbe;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{ClientProbe, Pattern};
use crate::server::shared::logos::Logo;

/// CheckMK Agent - Monitoring agent for CheckMK server
///
/// Port 6556 (TCP) is the IANA-assigned port for CheckMK agent.
/// The agent uses a text-based protocol where the server connects and
/// receives monitoring data in a custom format with sections like
/// <<<check_mk>>>, <<<cpu>>>, <<<mem>>>, etc.
///
/// Detection: Port-only (Medium confidence for IANA-assigned port).
///
/// Note: This is for the CheckMK Agent (port 6556), not the CheckMK Server
/// which has an HTTP interface.
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct CheckmkAgent;

impl ServiceDefinition for CheckmkAgent {
    fn name(&self) -> &'static str {
        "CheckMK Agent"
    }
    fn description(&self) -> &'static str {
        "Monitoring agent for CheckMK server"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }
    /// Matches on the probe's answer, so a listener on this port that does not speak the protocol is
    /// not claimed as this service.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::ClientResponse(ClientProbe::CheckMkAgent)
    }
    fn app_probes(&self) -> Vec<Box<dyn AppProbe>> {
        vec![Box::new(CheckMkAgentProbe)]
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::Checkmk)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<CheckmkAgent>
));
