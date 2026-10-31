use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Zigbee2MQTT;

impl ServiceDefinition for Zigbee2MQTT {
    fn name(&self) -> &'static str {
        "Zigbee2MQTT"
    }
    fn description(&self) -> &'static str {
        "Zigbee to MQTT bridge"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::HomeAutomation
    }
    // Often a dedicated Pi with the coordinator, and also Docker on a server.
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Suggests(DeviceType::DeviceHub)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::Http8080, "/", "Zigbee2MQTT WindFront", None)
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Zigbee2Mqtt)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Zigbee2MQTT>));
