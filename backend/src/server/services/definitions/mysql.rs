use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::mysql::MySqlProbe;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{ClientProbe, Pattern};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct MySQL;

impl ServiceDefinition for MySQL {
    fn name(&self) -> &'static str {
        "MySQL"
    }
    fn description(&self) -> &'static str {
        "Open-source relational database"
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
        Pattern::ClientResponse(ClientProbe::MySql)
    }
    fn app_probes(&self) -> Vec<Box<dyn AppProbe>> {
        vec![Box::new(MySqlProbe)]
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::Mysql)
    }
    fn is_generic(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<MySQL>));
