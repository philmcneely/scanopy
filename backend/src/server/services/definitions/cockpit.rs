use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Cockpit;

impl ServiceDefinition for Cockpit {
    fn name(&self) -> &'static str {
        "Cockpit"
    }
    fn description(&self) -> &'static str {
        "Web-based Linux server administration console"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::RemoteAccess
    }
    // An application serving other hosts, so its host is a server like any application's.
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    // cockpit-ws clears its session cookie on the login page (`cockpit=deleted`). 9090 is also
    // Prometheus's default port, so the port alone identifies nothing.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Header(
            Some(PortType::new_tcp(9090)),
            "set-cookie",
            "cockpit=",
            None,
        )
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Cockpit)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Cockpit>));
