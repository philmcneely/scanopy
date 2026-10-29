use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct CouchDB;

impl ServiceDefinition for CouchDB {
    fn name(&self) -> &'static str {
        "CouchDB"
    }
    fn description(&self) -> &'static str {
        "NoSQL document database"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Database
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::CouchDb, "/", "couchdb", None)
    }
    fn is_generic(&self) -> bool {
        true
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::CouchDb)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<CouchDB>));
