use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{Pattern, Vendor};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct WiZ;

impl ServiceDefinition for WiZ {
    fn name(&self) -> &'static str {
        "Wiz"
    }
    fn description(&self) -> &'static str {
        "Wiz IoT Devices"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::IoT
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::MacVendor(Vendor::WIZ)
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Wiz)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<WiZ>));
