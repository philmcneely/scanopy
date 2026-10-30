use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Radicale;

impl ServiceDefinition for Radicale {
    fn name(&self) -> &'static str {
        "Radicale"
    }
    fn description(&self) -> &'static str {
        "Free and Open-Source CalDAV and CardDAV Server"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Office
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(
            PortType::new_tcp(5232),
            "/.web/",
            "Radicale Web Interface",
            None,
        )
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::Radicale)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Radicale>));
