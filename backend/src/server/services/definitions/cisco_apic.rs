use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct CiscoApic;

impl ServiceDefinition for CiscoApic {
    fn name(&self) -> &'static str {
        "Cisco APIC"
    }
    fn description(&self) -> &'static str {
        "Cisco ACI Application Policy Infrastructure Controller"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkCore
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Is(DeviceType::NetworkManagementDevice)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Cisco", "APIC")], &[443])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Cisco)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<CiscoApic>));
