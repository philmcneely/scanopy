use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct BlueCoatProxy;

impl ServiceDefinition for BlueCoatProxy {
    fn name(&self) -> &'static str {
        "Blue Coat ProxySG"
    }
    fn description(&self) -> &'static str {
        "Blue Coat ProxySG web proxy"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkAppliance
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Is(DeviceType::SecurityAppliance)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Blue Coat", "Proxy")], &[443, 8082])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<BlueCoatProxy>
));
