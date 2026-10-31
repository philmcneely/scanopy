use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct R1SoftBackup;

impl ServiceDefinition for R1SoftBackup {
    fn name(&self) -> &'static str {
        "R1Soft Server Backup Manager"
    }
    fn description(&self) -> &'static str {
        "ConnectWise R1Soft server backup"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Backup
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(
            &[("ConnectWise", "R1Soft Server Backup Manager")],
            &[80, 443],
        )
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<R1SoftBackup>
));
