use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct AvtechDvr;

impl ServiceDefinition for AvtechDvr {
    fn name(&self) -> &'static str {
        "AVTECH DVR"
    }
    fn description(&self) -> &'static str {
        "AVTECH video recorder web interface"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::IoT
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Is(DeviceType::Dvr)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("AVTECH", "Device Manager")], &[80, 88])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<AvtechDvr>));
