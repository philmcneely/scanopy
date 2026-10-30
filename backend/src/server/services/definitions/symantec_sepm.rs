use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct SymantecSepm;

impl ServiceDefinition for SymantecSepm {
    fn name(&self) -> &'static str {
        "Symantec Endpoint Protection Manager"
    }
    fn description(&self) -> &'static str {
        "Symantec endpoint security management"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(
            &[("Symantec", "Endpoint Protection Manager")],
            &[8443, 9090],
        )
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Symantec)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<SymantecSepm>
));
