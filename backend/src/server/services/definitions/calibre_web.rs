use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct CalibreWeb;

impl ServiceDefinition for CalibreWeb {
    fn name(&self) -> &'static str {
        "Calibre-Web"
    }
    fn description(&self) -> &'static str {
        "A web app for browsing, reading and downloading books from a calibre library."
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Media
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    /// The login page is unauthenticated and titled `<instance> | <page>`, where the instance name
    /// defaults to "Calibre-Web".
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::new_tcp(8083), "/login", "Calibre-Web |", None)
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::CalibreWeb)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<CalibreWeb>));
