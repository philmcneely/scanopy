use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::mgcp::MgcpProbe;
use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{ClientProbe, Pattern};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Mgcp;

impl ServiceDefinition for Mgcp {
    fn name(&self) -> &'static str {
        "MGCP Endpoint"
    }
    fn description(&self) -> &'static str {
        "Media Gateway Control Protocol, tying a voice gateway to its call agent"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Telephony
    }
    fn device_type(&self) -> Option<DeviceType> {
        Some(DeviceType::VoipGateway)
    }

    /// Either end of the protocol. A gateway listens on 2427 and a call agent on 2727; finding
    /// either identifies the service, and the two are not usually on one host.
    ///
    /// Each arm needs its probe's `AUEP` answered, so a container that only declares 2427/udp or
    /// 2727/udp is not claimed as this service.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::ClientResponse(ClientProbe::Mgcp)
    }
    fn app_probes(&self) -> Vec<Box<dyn AppProbe>> {
        vec![Box::new(MgcpProbe)]
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Mgcp>));
