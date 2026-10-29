use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Unraid;

impl ServiceDefinition for Unraid {
    fn name(&self) -> &'static str {
        "Unraid"
    }
    fn description(&self) -> &'static str {
        "Unraid NAS operating system"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Storage
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Is(DeviceType::Nas)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Lime Technologies", "Unraid")], &[80, 443])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Unraid>));
