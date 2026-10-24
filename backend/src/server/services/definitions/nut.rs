use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::nut::NutProbe;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{Pattern, probe_pattern};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct NUT;

impl ServiceDefinition for NUT {
    fn name(&self) -> &'static str {
        "NUT"
    }
    fn description(&self) -> &'static str {
        "Network UPS Tools"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }

    /// Derived from the probe, so a listener on this port that does not speak the protocol is
    /// not claimed as this service.
    fn discovery_pattern(&self) -> Pattern<'_> {
        probe_pattern(&NutProbe)
    }
    fn app_probes(&self) -> Vec<Box<dyn AppProbe>> {
        vec![Box::new(NutProbe)]
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Nut)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<NUT>));
