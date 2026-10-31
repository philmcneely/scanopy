use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Icinga;

impl ServiceDefinition for Icinga {
    fn name(&self) -> &'static str {
        "Icinga"
    }
    fn description(&self) -> &'static str {
        "Infrastructure monitoring"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::new_tcp(5665), "/v1", "icinga", None)
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::Icinga)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Icinga>));
