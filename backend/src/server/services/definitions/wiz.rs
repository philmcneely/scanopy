use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{Pattern, Vendor};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct WiZ;

impl ServiceDefinition for WiZ {
    fn name(&self) -> &'static str {
        "Wiz"
    }
    fn description(&self) -> &'static str {
        "Wiz IoT Devices"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::IoT
    }
    // WiZ sells mostly bulbs, and the MAC vendor is the only evidence.
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Is(DeviceType::LightBulb)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::MacVendor(Vendor::WIZ)
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Wiz)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<WiZ>));
