use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Caddy;

impl ServiceDefinition for Caddy {
    fn name(&self) -> &'static str {
        "Caddy"
    }
    fn description(&self) -> &'static str {
        "Lightweight & versatile reverse proxy, web & file server"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::ReverseProxy
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(
            PortType::new_tcp(2019),
            "/reverse_proxy/upstreams",
            "num_requests",
            None,
        )
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Caddy)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Caddy>));
