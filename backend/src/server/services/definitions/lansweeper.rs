use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Lansweeper;

impl ServiceDefinition for Lansweeper {
    fn name(&self) -> &'static str {
        "Lansweeper"
    }
    fn description(&self) -> &'static str {
        "IT asset discovery and inventory"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Lansweeper", "Lansweeper")], &[81, 443])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Lansweeper>));
