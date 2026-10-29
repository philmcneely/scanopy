use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Serviio;

impl ServiceDefinition for Serviio {
    fn name(&self) -> &'static str {
        "Serviio"
    }
    fn description(&self) -> &'static str {
        "DLNA media streaming server"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Media
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(
            PortType::new_tcp(23423),
            "/rest/application",
            "<serviioId>",
            None,
        )
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Serviio)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Serviio>));
