use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::{ConnectOnly, ServiceDefinition};
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Denodo;

impl ServiceDefinition for Denodo {
    fn name(&self) -> &'static str {
        "Denodo"
    }
    fn description(&self) -> &'static str {
        "Data virtualization"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Database
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    /// Denodo is commercial, with no pullable image and no published source, so a match string for
    /// its web consoles would be a guess.
    fn connect_only_rationale(&self) -> Option<ConnectOnly> {
        Some(ConnectOnly::NoVerifiableImplementation)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::Port(PortType::new_tcp(9990)),
            Pattern::Port(PortType::new_tcp(9996)),
            Pattern::Port(PortType::new_tcp(9090)),
            Pattern::Port(PortType::new_tcp(9099)),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Denodo)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Denodo>));
