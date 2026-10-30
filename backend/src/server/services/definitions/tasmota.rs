use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Tasmota;

impl ServiceDefinition for Tasmota {
    fn name(&self) -> &'static str {
        "Tasmota"
    }
    fn description(&self) -> &'static str {
        "ESP device firmware"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::IoT
    }
    // Mostly flashed onto plugs and relays, and also onto bulbs and sensors.
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Suggests(DeviceType::PowerRelay)
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::None
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::Tasmota)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Tasmota>));
