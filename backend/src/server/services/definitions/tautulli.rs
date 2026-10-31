use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Tautulli;

impl ServiceDefinition for Tautulli {
    fn name(&self) -> &'static str {
        "Tautulli"
    }
    fn description(&self) -> &'static str {
        "Monitor, view analytics, and receive notifications about your Plex Media Server."
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Media
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::new_tcp(8181), "/", "Tautulli", None)
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Tautulli)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Tautulli>));
