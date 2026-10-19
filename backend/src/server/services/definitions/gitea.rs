use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Gitea;

impl ServiceDefinition for Gitea {
    fn name(&self) -> &'static str {
        "Gitea"
    }
    fn description(&self) -> &'static str {
        "DevOps platform"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Development
    }
    /// A set-up instance serves its pages on `/explore/repos`. Until setup is done, every path but
    /// the installer at `/` answers 404, so a fresh install is found by its installer page.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AnyOf(vec![
            Pattern::Endpoint(
                PortType::Http3000,
                "/explore/repos",
                "Powered by Gitea",
                None,
            ),
            Pattern::Endpoint(
                PortType::Http3000,
                "/",
                "Installation - Gitea",
                Some(200..300),
            ),
        ])
    }

    fn logo_url(&self) -> &'static str {
        "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/svg/gitea.svg"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Gitea>));
