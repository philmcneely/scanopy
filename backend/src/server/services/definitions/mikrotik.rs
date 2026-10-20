use crate::server::hosts::r#impl::device_role::DeviceRole;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Mikrotik;

impl ServiceDefinition for Mikrotik {
    fn name(&self) -> &'static str {
        "MikroTik"
    }
    fn description(&self) -> &'static str {
        "MikroTik RouterOS network device"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkAppliance
    }
    fn device_role(&self) -> Option<DeviceRole> {
        Some(DeviceRole::Router)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::Http, "/", "MikroTik RouterOS", None)
    }

    fn logo_url(&self) -> &'static str {
        "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/svg/mikrotik.svg"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Mikrotik>));
