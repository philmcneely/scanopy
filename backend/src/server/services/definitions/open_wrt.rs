use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct OpenWrt;

impl ServiceDefinition for OpenWrt {
    fn name(&self) -> &'static str {
        "OpenWrt"
    }
    fn description(&self) -> &'static str {
        "Open-source Linux router firmware with the LuCI web interface"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkAppliance
    }
    fn device_type(&self) -> Option<DeviceType> {
        Some(DeviceType::Router)
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(
            PortType::Http,
            "/",
            "LuCI - Lua Configuration Interface",
            None,
        )
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::OpenWrt)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<OpenWrt>));
