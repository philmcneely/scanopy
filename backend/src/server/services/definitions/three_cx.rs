use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

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
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("3CX", "3CX Web Server")], &[5000, 5001])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::ThreeCx)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<ThreeCx>));
