use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct OpengearConsole;

impl ServiceDefinition for OpengearConsole {
    fn name(&self) -> &'static str {
        "Opengear Console Server"
    }
    fn description(&self) -> &'static str {
        "Opengear console server for out-of-band management"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkCore
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Is(DeviceType::DeviceServer)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Opengear", "Management Console")], &[443])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Opengear)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<OpengearConsole>
));
