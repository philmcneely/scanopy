use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Cleanuparr;

impl ServiceDefinition for Cleanuparr {
    fn name(&self) -> &'static str {
        "Cleanuparr"
    }
    fn description(&self) -> &'static str {
        "Torrent cleanup tool for Sonarr and Radarr"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Media
    }

    /// The served page names the application in its title. `/health` answers a bare `healthy`,
    /// which any HTTP listener can return, so it identifies nothing.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(
            PortType::new_tcp(11011),
            "/",
            "<title>Cleanuparr</title>",
            None,
        )
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Cleanuperr)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Cleanuparr>));
