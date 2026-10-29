use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct ControlWebPanel;

impl ServiceDefinition for ControlWebPanel {
    fn name(&self) -> &'static str {
        "Control Web Panel"
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
        Pattern::Recog(&[("Control Web Panel", "Control Web Panel")], &[2030, 2031])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<ControlWebPanel>
));
