use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct MoveItTransfer;

impl ServiceDefinition for MoveItTransfer {
    fn name(&self) -> &'static str {
        "MOVEit Transfer"
    }
    fn description(&self) -> &'static str {
        "Progress MOVEit managed file transfer"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Integration
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Progress", "MOVEit Transfer")], &[443])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Progress)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<MoveItTransfer>
));
