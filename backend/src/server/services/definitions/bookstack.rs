use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct BookStack;

impl ServiceDefinition for BookStack {
    fn name(&self) -> &'static str {
        "BookStack"
    }
    fn description(&self) -> &'static str {
        "Self-hosted wiki for documentation and knowledge bases"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::ProjectManagement
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Header(None, "set-cookie", "bookstack_session", None)
    }
    fn logo_url(&self) -> &'static str {
        "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/svg/bookstack.svg"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<BookStack>));
