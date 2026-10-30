use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct CPanel;

impl ServiceDefinition for CPanel {
    fn name(&self) -> &'static str {
        "cPanel"
    }
    fn description(&self) -> &'static str {
        "Web hosting control panel"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Publishing
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("cPanel", "cPanel"), ("cPanel", "WHM")], &[2083, 2087])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::CPanel)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<CPanel>));
