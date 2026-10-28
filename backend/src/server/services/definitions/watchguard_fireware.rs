use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct WatchGuardFireware;

impl ServiceDefinition for WatchGuardFireware {
    fn name(&self) -> &'static str {
        "WatchGuard Fireware"
    }
    fn description(&self) -> &'static str {
        "WatchGuard Firebox firewall management"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkAppliance
    }
    fn device_type(&self) -> Option<DeviceType> {
        Some(DeviceType::Firewall)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("WatchGuard", "Fireware XTM")], &[443, 8080])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<WatchGuardFireware>
));
