use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::ipp::IppProbe;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{Pattern, probe_pattern};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct IppPrinter;

impl ServiceDefinition for IppPrinter {
    fn name(&self) -> &'static str {
        "IPP Printer"
    }
    fn description(&self) -> &'static str {
        "Printer accepting jobs over the Internet Printing Protocol"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Printer
    }

    /// Derived from the probe, so a listener on 631 that does not answer in IPP is not claimed.
    /// Generic, so CUPS and the vendor printer definitions, which match the same port with more
    /// specific evidence, claim it first: a CUPS server answers IPP too, and is a server.
    fn discovery_pattern(&self) -> Pattern<'_> {
        probe_pattern(&IppProbe)
    }
    fn app_probes(&self) -> Vec<Box<dyn AppProbe>> {
        vec![Box::new(IppProbe)]
    }

    fn is_generic(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<IppPrinter>));
