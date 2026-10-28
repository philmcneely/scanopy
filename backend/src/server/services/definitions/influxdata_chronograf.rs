use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Chronograf;

impl ServiceDefinition for Chronograf {
    fn name(&self) -> &'static str {
        "Chronograf"
    }
    fn description(&self) -> &'static str {
        "InfluxData time series dashboard"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Dashboard
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("InfluxData", "Chronograf")], &[8888])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::InfluxDb)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Chronograf>));
