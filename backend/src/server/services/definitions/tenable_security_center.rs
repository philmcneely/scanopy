use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct TenableSecurityCenter;

impl ServiceDefinition for TenableSecurityCenter {
    fn name(&self) -> &'static str {
        "Tenable Security Center"
    }
    fn description(&self) -> &'static str {
        "Tenable vulnerability management console"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Tenable", "Security Center")], &[443])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Tenable)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<TenableSecurityCenter>
));
