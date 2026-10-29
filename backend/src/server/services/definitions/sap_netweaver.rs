use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct SapNetWeaver;

impl ServiceDefinition for SapNetWeaver {
    fn name(&self) -> &'static str {
        "SAP NetWeaver"
    }
    fn description(&self) -> &'static str {
        "SAP NetWeaver application server"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Integration
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("SAP", "NetWeaver Web AS")], &[8000, 50000])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<SapNetWeaver>
));
