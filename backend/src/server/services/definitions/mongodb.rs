use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::mongodb::MongoDbProbe;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{Pattern, probe_pattern};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct MongoDB;

impl ServiceDefinition for MongoDB {
    fn name(&self) -> &'static str {
        "MongoDB"
    }
    fn description(&self) -> &'static str {
        "NoSQL document database"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Database
    }
    /// Derived from the probe, so a listener on this port that does not speak the protocol is
    /// not claimed as this service.
    fn discovery_pattern(&self) -> Pattern<'_> {
        probe_pattern(&MongoDbProbe)
    }
    fn app_probes(&self) -> Vec<Box<dyn AppProbe>> {
        vec![Box::new(MongoDbProbe)]
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::MongoDb)
    }
    fn is_generic(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<MongoDB>));
