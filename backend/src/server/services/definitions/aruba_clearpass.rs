use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct ArubaClearPass;

impl ServiceDefinition for ArubaClearPass {
    fn name(&self) -> &'static str {
        "Aruba ClearPass"
    }
    fn description(&self) -> &'static str {
        "Aruba ClearPass network access control"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::IdentityAndAccess
    }
    fn device_type(&self) -> Option<DeviceType> {
        Some(DeviceType::SecurityAppliance)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Aruba Networks", "ClearPass Policy Manager")], &[443])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Aruba)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<ArubaClearPass>
));
