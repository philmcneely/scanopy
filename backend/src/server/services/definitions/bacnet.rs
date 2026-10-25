use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::udp::BacnetProbe;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{Pattern, probe_pattern};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct BACnet;

impl ServiceDefinition for BACnet {
    fn name(&self) -> &'static str {
        "BACnet"
    }
    fn description(&self) -> &'static str {
        "Building automation and control network protocol"
    }
    /// Building automation is operational technology, not a smart home. Filed under `IoT` until
    /// `Industrial` existed, which got more wrong with every industrial protocol added beside it.
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Industrial
    }
    /// Derived from the probe, so a container that only declares 47808/udp is not claimed as this
    /// service.
    fn discovery_pattern(&self) -> Pattern<'_> {
        probe_pattern(&BacnetProbe)
    }
    fn is_generic(&self) -> bool {
        true
    }

    fn app_probes(&self) -> Vec<Box<dyn AppProbe>> {
        vec![Box::new(BacnetProbe)]
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<BACnet>));
