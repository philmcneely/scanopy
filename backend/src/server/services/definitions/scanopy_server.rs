use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct ScanopyServer;

impl ServiceDefinition for ScanopyServer {
    fn name(&self) -> &'static str {
        "Scanopy Server"
    }
    fn description(&self) -> &'static str {
        "Automatically discover and visually document network infrastructure"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Scanopy
    }
    // An application serving other hosts, so its host is a server like any application's.
    fn device_type(&self) -> Option<DeviceType> {
        Some(DeviceType::Server)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::new_tcp(60072), "/api/health", "scanopy", None)
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Scanopy)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<ScanopyServer>
));
