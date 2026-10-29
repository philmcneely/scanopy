use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::ssh::SshProbe;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{ClientProbe, Pattern};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Ssh;

impl ServiceDefinition for Ssh {
    fn name(&self) -> &'static str {
        "SSH"
    }
    fn description(&self) -> &'static str {
        "Secure Shell remote access"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::RemoteAccess
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Nothing
    }
    /// Either an SSH credential authenticated, on whatever port it is configured for, or the
    /// banner probe answered on 22. Both require the protocol to answer, so a listener that does
    /// not speak SSH is not claimed as this service.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::ClientResponse(ClientProbe::Ssh)
    }
    fn app_probes(&self) -> Vec<Box<dyn AppProbe>> {
        vec![Box::new(SshProbe)]
    }
    fn is_generic(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Ssh>));
