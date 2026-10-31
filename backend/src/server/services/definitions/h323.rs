use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::h323::H323Probe;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{ClientProbe, Pattern};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct H323;

impl ServiceDefinition for H323 {
    fn name(&self) -> &'static str {
        "H.323 Gateway"
    }
    fn description(&self) -> &'static str {
        "Voice and video call signalling over H.323"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Telephony
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    /// Call signalling on 1720, qualified by a Q.931 response to a `SETUP`.
    ///
    /// Defining this port closes a hole rather than only adding a detection: 1720 is one of the
    /// ports a FortiGate session helper answers for every address it fronts, and until now no
    /// definition claimed it — so on a *full* scan of a routed subnet an open 1720 was an open port
    /// no probe covered, which was read as proof that a host existed. See
    /// [`crate::daemon::utils::app_probe::h323`].
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::ClientResponse(ClientProbe::H323)
    }
    fn app_probes(&self) -> Vec<Box<dyn AppProbe>> {
        vec![Box::new(H323Probe)]
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<H323>));
