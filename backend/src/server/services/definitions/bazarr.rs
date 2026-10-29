use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Bazarr;

impl ServiceDefinition for Bazarr {
    fn name(&self) -> &'static str {
        "Bazarr"
    }
    fn description(&self) -> &'static str {
        "A companion application to Sonarr and Radarr that manages and downloads subtitles"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Media
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    /// The login page is served unauthenticated and carries the application name in its title.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::new_tcp(6767), "/", "<title>Bazarr</title>", None)
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Bazarr)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Bazarr>));
