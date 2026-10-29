use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::oracle::OracleProbe;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{ClientProbe, Pattern};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct OracleDB;

impl ServiceDefinition for OracleDB {
    fn name(&self) -> &'static str {
        "Oracle Database"
    }
    fn description(&self) -> &'static str {
        "Enterprise relational database"
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
        Pattern::ClientResponse(ClientProbe::OracleTns)
    }
    fn app_probes(&self) -> Vec<Box<dyn AppProbe>> {
        vec![Box::new(OracleProbe)]
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::Oracle)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<OracleDB>));
