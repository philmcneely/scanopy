use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Jellyfin;

impl ServiceDefinition for Jellyfin {
    fn name(&self) -> &'static str {
        "Jellyfin"
    }
    fn description(&self) -> &'static str {
        "Free media server for personal streaming"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Media
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::Http, "/System/Info/Public", "Jellyfin", None)
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Jellyfin)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Jellyfin>));
