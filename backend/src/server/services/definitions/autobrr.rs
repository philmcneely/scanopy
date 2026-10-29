use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Autobrr;

impl ServiceDefinition for Autobrr {
    fn name(&self) -> &'static str {
        "Autobrr"
    }
    fn description(&self) -> &'static str {
        "The modern autodl-irssi replacement."
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Media
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    /// The served page names the application. `/api/healthz/liveness` is unauthenticated but
    /// answers a bare `OK`, which any HTTP listener can return, so it identifies nothing.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::new_tcp(7474), "/", "autobrr", None)
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Autobrr)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Autobrr>));
