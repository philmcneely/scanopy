use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::rtsp::RtspProbe;
use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{ClientProbe, Pattern};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct WebService;

impl ServiceDefinition for WebService {
    fn name(&self) -> &'static str {
        "RTSP Camera"
    }
    fn description(&self) -> &'static str {
        "Camera with RTSP Streaming"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::IoT
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Is(DeviceType::Camera)
    }

    /// Matches on the probe's answer, so a listener on this port that does not speak the protocol is
    /// not claimed as this service.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::ClientResponse(ClientProbe::Rtsp)
    }
    fn app_probes(&self) -> Vec<Box<dyn AppProbe>> {
        vec![Box::new(RtspProbe)]
    }

    fn is_generic(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<WebService>));
