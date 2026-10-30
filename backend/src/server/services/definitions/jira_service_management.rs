use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct JiraServiceManagement;

impl ServiceDefinition for JiraServiceManagement {
    fn name(&self) -> &'static str {
        "Jira Service Management"
    }
    fn description(&self) -> &'static str {
        "Atlassian Jira Service Management service desk"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::ProjectManagement
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Atlassian", "Jira Service Management")], &[443, 8080])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Jira)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<JiraServiceManagement>
));
