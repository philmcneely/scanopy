use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Gatus;

impl ServiceDefinition for Gatus {
    fn name(&self) -> &'static str {
        "Gatus"
    }
    fn description(&self) -> &'static str {
        "Automated developer-oriented status page"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::Http8080, "/manifest.json", "Gatus", None)
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Gatus)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Gatus>));
