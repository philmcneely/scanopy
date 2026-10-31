use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct SangomaSwitchvox;

impl ServiceDefinition for SangomaSwitchvox {
    fn name(&self) -> &'static str {
        "Switchvox"
    }
    fn description(&self) -> &'static str {
        "Sangoma Switchvox phone system"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Telephony
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Is(DeviceType::VoipServer)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Sangoma", "Switchvox")], &[443])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Sangoma)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<SangomaSwitchvox>
));
