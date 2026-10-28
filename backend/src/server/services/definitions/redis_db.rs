use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::redis::RedisProbe;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{ClientProbe, Pattern};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct RedisDB;

impl ServiceDefinition for RedisDB {
    fn name(&self) -> &'static str {
        "Redis"
    }
    fn description(&self) -> &'static str {
        "In-memory data store and cache"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Database
    }
    /// Matches on the probe's answer, so a listener on this port that does not speak the protocol is
    /// not claimed as this service.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::ClientResponse(ClientProbe::Redis)
    }
    fn app_probes(&self) -> Vec<Box<dyn AppProbe>> {
        vec![Box::new(RedisProbe)]
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::Redis)
    }
    fn is_generic(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<RedisDB>));
