use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::cassandra::CassandraProbe;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{ClientProbe, Pattern};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Cassandra;

impl ServiceDefinition for Cassandra {
    fn name(&self) -> &'static str {
        "Cassandra"
    }
    fn description(&self) -> &'static str {
        "Distributed NoSQL database"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Database
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }
    /// Matches on the probe's answer, so a listener on this port that does not speak the protocol is
    /// not claimed as this service.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::ClientResponse(ClientProbe::Cassandra)
    }
    fn app_probes(&self) -> Vec<Box<dyn AppProbe>> {
        vec![Box::new(CassandraProbe)]
    }
    fn is_generic(&self) -> bool {
        true
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::Cassandra)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Cassandra>));
