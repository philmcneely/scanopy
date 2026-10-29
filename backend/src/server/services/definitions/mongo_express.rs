use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct MongoExpress;

impl ServiceDefinition for MongoExpress {
    fn name(&self) -> &'static str {
        "mongo-express"
    }
    fn description(&self) -> &'static str {
        "MongoDB web administration"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Database
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("mongo-express Project", "mongo-express")], &[8081])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<MongoExpress>
));
