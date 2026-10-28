use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct RedisCommander;

impl ServiceDefinition for RedisCommander {
    fn name(&self) -> &'static str {
        "Redis Commander"
    }
    fn description(&self) -> &'static str {
        "Redis web administration"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Database
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Ferner", "Redis Commander")], &[8081])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Redis)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<RedisCommander>
));
