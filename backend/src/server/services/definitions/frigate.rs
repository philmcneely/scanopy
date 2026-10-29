use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Frigate;

impl ServiceDefinition for Frigate {
    fn name(&self) -> &'static str {
        "Frigate"
    }
    fn description(&self) -> &'static str {
        "NVR with realtime object detection for IP Cameras"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::IoT
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AnyOf(vec![
            Pattern::Endpoint(PortType::Https, "/", "<title>Frigate</title>", None),
            Pattern::Endpoint(PortType::Http5000, "/", "<title>Frigate</title>", None),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Frigate)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Frigate>));
