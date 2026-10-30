use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Docmost;

impl ServiceDefinition for Docmost {
    fn name(&self) -> &'static str {
        "Docmost"
    }
    fn description(&self) -> &'static str {
        "Open-source collaborative wiki and documentation platform"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::ProjectManagement
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        // Single-page app: the title is served at every path, on the default port
        // and behind a reverse proxy alike.
        Pattern::AnyOf(vec![
            Pattern::Endpoint(PortType::Http3000, "/", "<title>Docmost</title>", None),
            Pattern::Endpoint(PortType::Http, "/", "<title>Docmost</title>", None),
            Pattern::Endpoint(PortType::Https, "/", "<title>Docmost</title>", None),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Docmost)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Docmost>));
