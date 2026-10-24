use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct VCenter;

impl ServiceDefinition for VCenter {
    fn name(&self) -> &'static str {
        "vCenter"
    }
    fn description(&self) -> &'static str {
        "VMware vCenter Server virtualization management platform"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Hypervisor
    }
    fn device_type(&self) -> Option<DeviceType> {
        Some(DeviceType::Server)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::Https, "/", "VMware vCenter", None)
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::VMware)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<VCenter>));
