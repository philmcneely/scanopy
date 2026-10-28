use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct PfSense;

impl ServiceDefinition for PfSense {
    fn name(&self) -> &'static str {
        "pfSense"
    }
    fn description(&self) -> &'static str {
        "Open-source firewall and router platform"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkAppliance
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::Port(PortType::Ssh),
            Pattern::Endpoint(PortType::Http, "/", "pfsense", None),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::PfSense)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<PfSense>));
