use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct FreeIPA;

impl ServiceDefinition for FreeIPA {
    fn name(&self) -> &'static str {
        "FreeIPA"
    }
    fn description(&self) -> &'static str {
        "Identity management system"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::IdentityAndAccess
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        // The web UI's page title (`install/ui/index.html`).
        Pattern::Endpoint(
            PortType::Http,
            "/ipa/ui",
            "Identity Management",
            Some(200..300),
        )
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::FreeIpa)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<FreeIPA>));
