use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::udp::DhcpProbe;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{Pattern, probe_pattern};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct DhcpServer;

impl ServiceDefinition for DhcpServer {
    fn name(&self) -> &'static str {
        "DHCP Server"
    }
    fn description(&self) -> &'static str {
        "A generic DHCP server"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkCore
    }

    /// Derived from the probe, so a container that only declares 67/udp is not claimed as this
    /// service.
    fn discovery_pattern(&self) -> Pattern<'_> {
        probe_pattern(&DhcpProbe)
    }

    fn is_generic(&self) -> bool {
        true
    }

    fn app_probes(&self) -> Vec<Box<dyn AppProbe>> {
        vec![Box::new(DhcpProbe)]
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<DhcpServer>));
