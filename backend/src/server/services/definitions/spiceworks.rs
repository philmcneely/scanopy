use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Spiceworks;

impl ServiceDefinition for Spiceworks {
    fn name(&self) -> &'static str {
        "Spiceworks"
    }
    fn description(&self) -> &'static str {
        "Spiceworks help desk and network monitor"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(
            &[
                ("Spiceworks", "Help Desk Server"),
                ("Spiceworks", "Network Monitor"),
            ],
            &[9675, 9676],
        )
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Spiceworks)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Spiceworks>));
