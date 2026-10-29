use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct SonicWallUma;

impl ServiceDefinition for SonicWallUma {
    fn name(&self) -> &'static str {
        "SonicWall UMA"
    }
    fn description(&self) -> &'static str {
        "SonicWall Universal Management Appliance"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Is(DeviceType::NetworkManagementDevice)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("SonicWall", "Universal Management Appliance")], &[443])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::SonicWall)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<SonicWallUma>
));
