use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::vnc::VncProbe;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{Pattern, probe_pattern};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Vnc;

impl ServiceDefinition for Vnc {
    fn name(&self) -> &'static str {
        "VNC"
    }
    fn description(&self) -> &'static str {
        "VNC remote desktop, including macOS Screen Sharing"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::RemoteAccess
    }
    /// Derived from the probe, so a listener on 5900 that does not greet with an RFB version is
    /// not claimed as this service.
    fn discovery_pattern(&self) -> Pattern<'_> {
        probe_pattern(&VncProbe)
    }
    fn app_probes(&self) -> Vec<Box<dyn AppProbe>> {
        vec![Box::new(VncProbe)]
    }
    fn is_generic(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Vnc>));
