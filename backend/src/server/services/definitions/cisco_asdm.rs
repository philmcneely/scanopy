use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct CiscoAsdm;

impl ServiceDefinition for CiscoAsdm {
    fn name(&self) -> &'static str {
        "Cisco ASDM"
    }
    fn description(&self) -> &'static str {
        "Cisco Adaptive Security Device Manager on an ASA firewall"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkAppliance
    }
    fn device_type(&self) -> Option<DeviceType> {
        Some(DeviceType::Firewall)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Cisco", "ASDM")], &[443])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Cisco)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<CiscoAsdm>));
