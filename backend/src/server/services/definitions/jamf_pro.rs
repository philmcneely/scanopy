use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct JamfPro;

impl ServiceDefinition for JamfPro {
    fn name(&self) -> &'static str {
        "Jamf Pro"
    }
    fn description(&self) -> &'static str {
        "Jamf Pro Apple device management"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::IdentityAndAccess
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Jamf", "Jamf Pro")], &[443, 8443])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Jamf)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<JamfPro>));
