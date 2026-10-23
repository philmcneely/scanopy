use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::epics_ca::EpicsCaProbe;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{Pattern, probe_pattern};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Epics;

impl ServiceDefinition for Epics {
    fn name(&self) -> &'static str {
        "EPICS IOC"
    }
    fn description(&self) -> &'static str {
        "EPICS Channel Access server for accelerator and lab control systems"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Industrial
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        probe_pattern(&EpicsCaProbe)
    }
    fn app_probes(&self) -> Vec<Box<dyn AppProbe>> {
        vec![Box::new(EpicsCaProbe)]
    }
    fn is_generic(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Epics>));
