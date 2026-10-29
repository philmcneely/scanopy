use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct CitrixNetScaler;

impl ServiceDefinition for CitrixNetScaler {
    fn name(&self) -> &'static str {
        "Citrix NetScaler"
    }
    fn description(&self) -> &'static str {
        "Citrix NetScaler application delivery controller and gateway"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkAppliance
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Is(DeviceType::SecurityAppliance)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(
            &[
                ("Citrix", "Netscaler"),
                ("Citrix", "NetScaler"),
                ("Citrix", "NetScaler Gateway"),
                ("Citrix", "NetScaler SDX Gateway"),
            ],
            &[80, 443],
        )
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<CitrixNetScaler>
));
