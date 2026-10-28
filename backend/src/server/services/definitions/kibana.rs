use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Kibana;

impl ServiceDefinition for Kibana {
    fn name(&self) -> &'static str {
        "Kibana"
    }
    fn description(&self) -> &'static str {
        "Elastic data visualization dashboard"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Dashboard
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Elastic", "Kibana")], &[5601])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Elastic)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Kibana>));
