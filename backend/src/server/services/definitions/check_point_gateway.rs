use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct CheckPointGateway;

impl ServiceDefinition for CheckPointGateway {
    fn name(&self) -> &'static str {
        "Check Point Gateway"
    }
    fn description(&self) -> &'static str {
        "Check Point security gateway"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkAppliance
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Is(DeviceType::Firewall)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Check Point", "Firewall-1")], &[443])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::CheckPoint)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<CheckPointGateway>
));
