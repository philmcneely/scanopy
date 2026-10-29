use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{Pattern, Vendor};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct EeroRepeater;

impl ServiceDefinition for EeroRepeater {
    fn name(&self) -> &'static str {
        "Eero Repeater"
    }
    fn description(&self) -> &'static str {
        "Eero device providing mesh network services"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkAccess
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Is(DeviceType::WlanRepeater)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::MacVendor(Vendor::EERO),
            Pattern::Not(Box::new(Pattern::IsGateway)),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Eero)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<EeroRepeater>
));
