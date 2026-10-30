use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct AtlassianFisheye;

impl ServiceDefinition for AtlassianFisheye {
    fn name(&self) -> &'static str {
        "Atlassian Fisheye"
    }
    fn description(&self) -> &'static str {
        "Atlassian Fisheye repository browser and Crucible code review"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Development
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Atlassian", "Fisheye")], &[8060])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Atlassian)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<AtlassianFisheye>
));
