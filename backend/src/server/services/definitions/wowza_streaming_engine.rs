use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct WowzaStreamingEngine;

impl ServiceDefinition for WowzaStreamingEngine {
    fn name(&self) -> &'static str {
        "Wowza Streaming Engine"
    }
    fn description(&self) -> &'static str {
        "Live video streaming server"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Media
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Wowza", "Streaming Engine")], &[8087, 8088])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<WowzaStreamingEngine>
));
