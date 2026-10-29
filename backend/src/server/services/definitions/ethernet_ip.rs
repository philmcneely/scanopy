use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::ethernet_ip::EtherNetIpProbe;
use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{ClientProbe, Pattern};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct EtherNetIp;

impl ServiceDefinition for EtherNetIp {
    fn name(&self) -> &'static str {
        "EtherNet/IP"
    }
    fn description(&self) -> &'static str {
        "CIP-based industrial protocol used by Rockwell and Allen-Bradley automation equipment"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Industrial
    }
    // Adapters sit on PLCs, drives and I/O modules alike.
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Suggests(DeviceType::Plc)
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::ClientResponse(ClientProbe::EtherNetIp)
    }
    fn app_probes(&self) -> Vec<Box<dyn AppProbe>> {
        vec![Box::new(EtherNetIpProbe)]
    }
    fn is_generic(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<EtherNetIp>));
