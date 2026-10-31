use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct JiraServer;

impl ServiceDefinition for JiraServer {
    fn name(&self) -> &'static str {
        "Jira"
    }
    fn description(&self) -> &'static str {
        "Project management platform"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::ProjectManagement
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(
            PortType::Http8080,
            "/rest/api/2/serverInfo",
            "jira",
            Some(200..300),
        )
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::Jira)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<JiraServer>));
