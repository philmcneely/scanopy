use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Ntfy;

impl ServiceDefinition for Ntfy {
    fn name(&self) -> &'static str {
        "Ntfy"
    }
    fn description(&self) -> &'static str {
        "Simple HTTP-based pub-sub notification service"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::MessageQueue
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AnyOf(vec![
            Pattern::Endpoint(PortType::Http, "/", "ntfy web", None),
            Pattern::Endpoint(PortType::new_tcp(2856), "/", "ntfy web", None),
        ])
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::Ntfy)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Ntfy>));
