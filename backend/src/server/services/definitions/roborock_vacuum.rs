use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{Pattern, Vendor};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct RoborockVacuum;

impl ServiceDefinition for RoborockVacuum {
    fn name(&self) -> &'static str {
        "Roborock Vacuum"
    }
    fn description(&self) -> &'static str {
        "Roborock robot vacuum cleaner"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::IoT
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::MacVendor(Vendor::ROBOROCK),
            Pattern::Port(PortType::new_tcp(58867)),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Roborock)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<RoborockVacuum>
));
