use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Ntopng;

impl ServiceDefinition for Ntopng {
    fn name(&self) -> &'static str {
        "ntopng"
    }
    fn description(&self) -> &'static str {
        "Network traffic monitoring"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("ntop", "ntopng")], &[3000])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Ntopng)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Ntopng>));
