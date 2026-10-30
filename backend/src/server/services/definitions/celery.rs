use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Celery;

impl ServiceDefinition for Celery {
    fn name(&self) -> &'static str {
        "Celery"
    }
    fn description(&self) -> &'static str {
        "Python task queue, seen through its Flower dashboard"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::MessageQueue
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        // Celery workers listen on no port of their own. Flower, the dashboard that monitors
        // them, serves on 5555.
        Pattern::Endpoint(PortType::new_tcp(5555), "/", "<title>flower", None)
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::Celery)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Celery>));
