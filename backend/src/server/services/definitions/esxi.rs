use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Esxi;

impl ServiceDefinition for Esxi {
    fn name(&self) -> &'static str {
        "ESXi"
    }
    fn description(&self) -> &'static str {
        "VMware ESXi bare-metal hypervisor for server virtualization"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Hypervisor
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::Https, "/", "VMware ESXi", None)
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Esxi)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Esxi>));
