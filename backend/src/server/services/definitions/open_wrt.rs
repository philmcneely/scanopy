use crate::server::hosts::r#impl::device_role::DeviceRole;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

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
    fn device_role(&self) -> Option<DeviceRole> {
        Some(DeviceRole::Router)
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(
            PortType::Http,
            "/",
            "LuCI - Lua Configuration Interface",
            None,
        )
    }
    fn logo_url(&self) -> &'static str {
        "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/svg/openwrt.svg"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<OpenWrt>));
