use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::lpd::LpdProbe;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{ClientProbe, Pattern};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Lpd;

impl ServiceDefinition for Lpd {
    fn name(&self) -> &'static str {
        "LPD"
    }
    fn description(&self) -> &'static str {
        "Line printer daemon accepting print jobs over RFC 1179"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Printer
    }
    /// Matches on the probe's answer, so a listener on 515 that does not return a queue status is
    /// not claimed. No device type: a print server or a Linux host running `cups-lpd` answers the
    /// same way a printer does. Generic, so a vendor printer definition that already named the
    /// device takes the port first.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::ClientResponse(ClientProbe::Lpd)
    }
    fn app_probes(&self) -> Vec<Box<dyn AppProbe>> {
        vec![Box::new(LpdProbe)]
    }
    fn is_generic(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Lpd>));
