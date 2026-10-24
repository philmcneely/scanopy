use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{InstantOnDeviceType, Pattern};
use crate::server::shared::logos::Logo;

/// An Instant On gateway/router, as reported by the site inventory.
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct InstantOnGateway;

impl ServiceDefinition for InstantOnGateway {
    fn name(&self) -> &'static str {
        "Instant On Gateway"
    }
    fn description(&self) -> &'static str {
        "HPE Networking (formerly Aruba) Instant On gateway"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkAccess
    }
    fn device_type(&self) -> Option<DeviceType> {
        Some(DeviceType::Router)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::ManagedDeviceType(InstantOnDeviceType::GATEWAY)
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Hpe)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<InstantOnGateway>
));
