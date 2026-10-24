use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{Pattern, Vendor};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct RingDoorbell;

impl ServiceDefinition for RingDoorbell {
    fn name(&self) -> &'static str {
        "Ring Doorbell"
    }

    fn description(&self) -> &'static str {
        "Ring video doorbell or security camera"
    }

    fn category(&self) -> ServiceCategory {
        ServiceCategory::IoT
    }
    fn device_type(&self) -> Option<DeviceType> {
        Some(DeviceType::Camera)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::MacVendor(Vendor::AMAZON),
            Pattern::AnyOf(vec![
                Pattern::Port(PortType::new_tcp(8557)),
                Pattern::Port(PortType::new_tcp(9998)),
                Pattern::Port(PortType::new_tcp(19302)),
                Pattern::Port(PortType::new_tcp(9999)),
            ]),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Ring)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<RingDoorbell>
));
