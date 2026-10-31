use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::epics_ca::EpicsCaProbe;
use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{ClientProbe, Pattern};
use crate::server::shared::logos::Logo;

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
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Is(DeviceType::Industrial)
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::ClientResponse(ClientProbe::EpicsCa)
    }
    fn app_probes(&self) -> Vec<Box<dyn AppProbe>> {
        vec![Box::new(EpicsCaProbe)]
    }
    fn is_generic(&self) -> bool {
        true
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Epics)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Epics>));
