use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Observium;

impl ServiceDefinition for Observium {
    fn name(&self) -> &'static str {
        "Observium"
    }
    fn description(&self) -> &'static str {
        "Network monitoring platform"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Observium", "Observium")], &[80, 443])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Observium)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Observium>));
