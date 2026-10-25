use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{DnsSdServiceType, Pattern, Vendor};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct PhilipsHueBridge;

impl ServiceDefinition for PhilipsHueBridge {
    fn name(&self) -> &'static str {
        "Philips Hue Bridge"
    }
    fn description(&self) -> &'static str {
        "Philips Hue Bridge for lighting control"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::IoT
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::MacVendor(Vendor::PHILIPS),
            Pattern::AnyOf(vec![
                Pattern::Endpoint(PortType::Http, "/", "hue", None),
                Pattern::DnsSd(DnsSdServiceType::HUE, None),
            ]),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::PhilipsHue)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<PhilipsHueBridge>
));
