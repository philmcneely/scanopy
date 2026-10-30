use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::llmnr::LlmnrProbe;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::{DeviceTypeImplication, ServiceDefinition};
use crate::server::services::r#impl::patterns::{ClientProbe, Pattern};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Llmnr;

impl ServiceDefinition for Llmnr {
    fn name(&self) -> &'static str {
        "LLMNR"
    }
    fn description(&self) -> &'static str {
        "Link-Local Multicast Name Resolution responder (RFC 4795)"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::DNS
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Nothing
    }
    /// Matches on the probe's answer, so a 5355 listener on a link where the responder has LLMNR
    /// off is not claimed.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::ClientResponse(ClientProbe::Llmnr)
    }
    fn app_probes(&self) -> Vec<Box<dyn AppProbe>> {
        vec![Box::new(LlmnrProbe)]
    }
    fn is_generic(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Llmnr>));
