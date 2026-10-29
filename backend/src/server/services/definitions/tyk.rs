use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Tyk;

impl ServiceDefinition for Tyk {
    fn name(&self) -> &'static str {
        "Tyk"
    }
    fn description(&self) -> &'static str {
        "API gateway"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::ReverseProxy
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::Http8080, "/hello", "tyk", Some(200..300))
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::Tyk)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Tyk>));
