use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::smux::SmuxProbe;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::{DeviceTypeImplication, ServiceDefinition};
use crate::server::services::r#impl::patterns::{ClientProbe, Pattern};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Smux;

impl ServiceDefinition for Smux {
    fn name(&self) -> &'static str {
        "SMUX"
    }
    fn description(&self) -> &'static str {
        "SNMP multiplexing port where subagents register MIB subtrees with the agent (RFC 1227)"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkCore
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Nothing
    }
    /// A separate protocol from SNMP on the same agent: subagents connect to it, managers never do.
    /// net-snmp opens it on every address by default, so it can answer where the agent's own UDP
    /// 161 does not.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::ClientResponse(ClientProbe::Smux)
    }
    fn app_probes(&self) -> Vec<Box<dyn AppProbe>> {
        vec![Box::new(SmuxProbe)]
    }
    fn is_generic(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Smux>));
