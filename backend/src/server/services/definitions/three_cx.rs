use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct ThreeCx;

impl ServiceDefinition for ThreeCx {
    fn name(&self) -> &'static str {
        "3CX"
    }
    fn description(&self) -> &'static str {
        "Phone system from 3CX"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Telephony
    }
    fn device_type(&self) -> Option<DeviceType> {
        Some(DeviceType::Server)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("3CX", "3CX Web Server")], &[5000, 5001])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<ThreeCx>));
