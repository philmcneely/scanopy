use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Jellyseerr;

impl ServiceDefinition for Jellyseerr {
    fn name(&self) -> &'static str {
        "Jellyseerr"
    }
    fn description(&self) -> &'static str {
        "Open source software application for managing requests for your media library."
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Media
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::new_tcp(5055), "/", "Jellyseerr", None)
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Jellyseerr)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Jellyseerr>));
