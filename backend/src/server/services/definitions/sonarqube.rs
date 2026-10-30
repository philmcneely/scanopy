use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct SonarQube;

impl ServiceDefinition for SonarQube {
    fn name(&self) -> &'static str {
        "SonarQube"
    }
    fn description(&self) -> &'static str {
        "Code quality and security analysis"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Development
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("SonarQube", "SonarQube")], &[9000])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::SonarQube)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<SonarQube>));
