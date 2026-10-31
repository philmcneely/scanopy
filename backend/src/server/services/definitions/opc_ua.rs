use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::opcua::OpcUaProbe;
use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{ClientProbe, Pattern};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct OpcUa;

impl ServiceDefinition for OpcUa {
    fn name(&self) -> &'static str {
        "OPC UA"
    }
    fn description(&self) -> &'static str {
        "Vendor-neutral industrial interoperability protocol for plant and process data"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Industrial
    }
    // Kepware and Ignition serve it from Windows PCs, so the OS outranks it.
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Suggests(DeviceType::Industrial)
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::ClientResponse(ClientProbe::OpcUa)
    }
    fn app_probes(&self) -> Vec<Box<dyn AppProbe>> {
        vec![Box::new(OpcUaProbe)]
    }
    fn is_generic(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<OpcUa>));
