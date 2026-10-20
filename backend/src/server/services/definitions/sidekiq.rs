use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Sidekiq;

impl ServiceDefinition for Sidekiq {
    fn name(&self) -> &'static str {
        "Sidekiq"
    }
    fn description(&self) -> &'static str {
        "Ruby background job processor, seen through its web dashboard"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::MessageQueue
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        // Sidekiq workers listen on no port of their own. Its web dashboard is mounted at
        // /sidekiq in the Rails app, which serves on 3000 or behind a proxy on 80.
        Pattern::AnyOf(vec![
            Pattern::Endpoint(PortType::Http3000, "/sidekiq", "sidekiq", Some(200..300)),
            Pattern::Endpoint(PortType::Http, "/sidekiq", "sidekiq", Some(200..300)),
        ])
    }
    fn logo_url(&self) -> &'static str {
        "https://simpleicons.org/icons/sidekiq.svg"
    }
    fn logo_needs_white_background(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Sidekiq>));
