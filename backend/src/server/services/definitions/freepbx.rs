use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct FreePBX;

impl ServiceDefinition for FreePBX {
    fn name(&self) -> &'static str {
        "FreePBX"
    }
    fn description(&self) -> &'static str {
        "Web interface for managing an Asterisk phone system"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Telephony
    }
    fn device_type(&self) -> Option<DeviceType> {
        Some(DeviceType::Server)
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::Endpoint(PortType::Http, "/", "freepbx", None),
            Pattern::Port(PortType::Sip),
        ])
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::FreePbx)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<FreePBX>));
