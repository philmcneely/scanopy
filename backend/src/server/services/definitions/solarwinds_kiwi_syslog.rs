use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct SolarWindsKiwiSyslog;

impl ServiceDefinition for SolarWindsKiwiSyslog {
    fn name(&self) -> &'static str {
        "Kiwi Syslog Server"
    }
    fn description(&self) -> &'static str {
        "SolarWinds Kiwi Syslog server"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("SolarWinds", "Kiwi Syslog")], &[8088])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<SolarWindsKiwiSyslog>
));
