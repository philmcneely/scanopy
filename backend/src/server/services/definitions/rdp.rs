use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::rdp::RdpProbe;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{ClientProbe, Pattern};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct RemoteDesktop;

impl ServiceDefinition for RemoteDesktop {
    fn name(&self) -> &'static str {
        "Remote Desktop"
    }
    fn description(&self) -> &'static str {
        "Remote Desktop Protocol (RDP)"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::RemoteAccess
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Nothing
    }
    /// Matches on the probe's answer, so a listener on this port that does not speak the protocol is
    /// not claimed as this service.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::ClientResponse(ClientProbe::Rdp)
    }
    fn app_probes(&self) -> Vec<Box<dyn AppProbe>> {
        vec![Box::new(RdpProbe)]
    }
    fn is_generic(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<RemoteDesktop>
));
