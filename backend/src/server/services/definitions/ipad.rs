use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{DnsSdServiceType, Pattern};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct IPad;

impl ServiceDefinition for IPad {
    fn name(&self) -> &'static str {
        "iPad"
    }
    fn description(&self) -> &'static str {
        "Apple iPad"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Mobile
    }
    fn device_type(&self) -> Option<DeviceType> {
        Some(DeviceType::Tablet)
    }

    /// The `_device-info` TXT `model` is the hardware identifier (`iPad16,1`), which names the
    /// product line by prefix. Recog's database lists exact identifiers and stops several
    /// generations back, so a prefix here identifies the models it has not caught up with.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::DnsSd(DnsSdServiceType::DEVICE_INFO, Some(("model", "iPad")))
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Apple)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<IPad>));
