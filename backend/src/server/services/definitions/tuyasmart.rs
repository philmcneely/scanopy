use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{Pattern, Vendor};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct TuyaSmart;

impl ServiceDefinition for TuyaSmart {
    fn name(&self) -> &'static str {
        "Tuya Smart"
    }
    fn description(&self) -> &'static str {
        "Tuya Smart IoT Devices"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::IoT
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::MacVendor(Vendor::TUYASMART),
            Pattern::Port(PortType::new_tcp(6668)),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Tuya)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<TuyaSmart>));
