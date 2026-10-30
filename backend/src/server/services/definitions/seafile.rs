use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Seafile;

impl ServiceDefinition for Seafile {
    fn name(&self) -> &'static str {
        "Seafile"
    }
    fn description(&self) -> &'static str {
        "File hosting platform"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Office
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::new_tcp(8000), "/api2/ping", "seafile", None)
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::Seafile)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Seafile>));
