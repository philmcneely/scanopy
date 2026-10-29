use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct CrushFtp;

impl ServiceDefinition for CrushFtp {
    fn name(&self) -> &'static str {
        "CrushFTP"
    }
    fn description(&self) -> &'static str {
        "CrushFTP file transfer server"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Storage
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("CrushFTP", "CrushFTP Web Interface")], &[443, 8080])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<CrushFtp>));
