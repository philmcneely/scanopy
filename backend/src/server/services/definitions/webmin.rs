use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Webmin;

impl ServiceDefinition for Webmin {
    fn name(&self) -> &'static str {
        "Webmin"
    }
    fn description(&self) -> &'static str {
        "Web-based server administration"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::RemoteAccess
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Nothing
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Webmin", "Webmin"), ("Webmin", "Usermin")], &[10000])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Webmin)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Webmin>));
