use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct VmwareSiteRecoveryManager;

impl ServiceDefinition for VmwareSiteRecoveryManager {
    fn name(&self) -> &'static str {
        "VMware Site Recovery Manager"
    }
    fn description(&self) -> &'static str {
        "VMware disaster recovery orchestration"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Backup
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("VMware", "Site Recovery Manager")], &[443])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::VMware)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<VmwareSiteRecoveryManager>
));
