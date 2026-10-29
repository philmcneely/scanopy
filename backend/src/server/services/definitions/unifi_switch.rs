use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{Pattern, UnifiDeviceType, Vendor};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct UnifiSwitch;

impl ServiceDefinition for UnifiSwitch {
    fn name(&self) -> &'static str {
        "UniFi Switch"
    }
    fn description(&self) -> &'static str {
        "Ubiquiti UniFi managed switch"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkCore
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Is(DeviceType::Switch)
    }

    /// UniFi switches expose no distinguishing network banner — that is the whole reason this
    /// integration exists — so the controller's own inventory is the only evidence. Narrower
    /// than the generic `Switch` definition, and it carries the UniFi logo.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::MacVendor(Vendor::UBIQUITI),
            Pattern::ManagedDeviceType(UnifiDeviceType::SWITCH),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::UniFi)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<UnifiSwitch>));
