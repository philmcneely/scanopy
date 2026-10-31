use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::rpcbind::RpcbindProbe;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::{DeviceTypeImplication, ServiceDefinition};
use crate::server::services::r#impl::patterns::{ClientProbe, Pattern};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Rpcbind;

impl ServiceDefinition for Rpcbind {
    fn name(&self) -> &'static str {
        "RPC Portmapper"
    }
    fn description(&self) -> &'static str {
        "ONC RPC portmapper (rpcbind), which maps RPC programs to their ports"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkCore
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Nothing
    }
    /// Matches on the probe's answer, so a listener on 111 that does not reply to the portmapper's
    /// NULL call is not claimed.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::ClientResponse(ClientProbe::Rpcbind)
    }
    fn app_probes(&self) -> Vec<Box<dyn AppProbe>> {
        vec![Box::new(RpcbindProbe)]
    }
    fn is_generic(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Rpcbind>));
