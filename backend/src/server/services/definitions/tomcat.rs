use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

// Tomcat
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Tomcat;

impl ServiceDefinition for Tomcat {
    fn name(&self) -> &'static str {
        "Tomcat"
    }
    fn description(&self) -> &'static str {
        "Java servlet container"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Development
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::Http8080, "/", "apache tomcat", None)
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::Tomcat)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Tomcat>));
