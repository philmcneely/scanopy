use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Ollama;

impl ServiceDefinition for Ollama {
    fn name(&self) -> &'static str {
        "Ollama"
    }
    fn description(&self) -> &'static str {
        "An easy way to get up and running with LLMs."
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Development
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::new_tcp(11434), "/", "Ollama is running", None)
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Ollama)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Ollama>));
