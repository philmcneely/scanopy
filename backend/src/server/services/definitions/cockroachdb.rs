use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct CockroachDb;

impl ServiceDefinition for CockroachDb {
    fn name(&self) -> &'static str {
        "CockroachDB"
    }
    fn description(&self) -> &'static str {
        "Distributed SQL database"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Database
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Cockroach Labs", "CockroachDB")], &[8080])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<CockroachDb>));
