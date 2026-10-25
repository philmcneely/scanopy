use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct RustDesk;

impl ServiceDefinition for RustDesk {
    fn name(&self) -> &'static str {
        "RustDesk"
    }
    fn description(&self) -> &'static str {
        "Self-hosted remote desktop server"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::RemoteAccess
    }
    // An application serving other hosts, so its host is a server like any application's.
    fn device_type(&self) -> Option<DeviceType> {
        Some(DeviceType::Server)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::new_tcp(21114), "/", "RustDesk API Admin", None)
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Rustdesk)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<RustDesk>));
