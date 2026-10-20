use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct SpringBoot;

impl ServiceDefinition for SpringBoot {
    fn name(&self) -> &'static str {
        "Spring Boot"
    }
    fn description(&self) -> &'static str {
        "Java application framework"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Development
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        // Actuator exposes its health endpoint by default, on the app's port or on a separate
        // management port, conventionally 8081.
        Pattern::AnyOf(vec![
            Pattern::Endpoint(
                PortType::Http8080,
                "/actuator/health",
                "\"status\":\"up\"",
                Some(200..300),
            ),
            Pattern::Endpoint(
                PortType::new_tcp(8081),
                "/actuator/health",
                "\"status\":\"up\"",
                Some(200..300),
            ),
        ])
    }
    fn logo_url(&self) -> &'static str {
        "https://www.vectorlogo.zone/logos/springio/springio-icon.svg"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<SpringBoot>));
