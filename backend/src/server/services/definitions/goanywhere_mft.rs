use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct GoAnywhereMft;

impl ServiceDefinition for GoAnywhereMft {
    fn name(&self) -> &'static str {
        "GoAnywhere MFT"
    }
    fn description(&self) -> &'static str {
        "Fortra GoAnywhere managed file transfer"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Integration
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Fortra", "GoAnywhere MFT")], &[8000, 8001])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<GoAnywhereMft>
));
