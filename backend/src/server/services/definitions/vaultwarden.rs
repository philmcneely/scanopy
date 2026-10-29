use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Vaultwarden;

impl ServiceDefinition for Vaultwarden {
    fn name(&self) -> &'static str {
        "Vaultwarden"
    }
    fn description(&self) -> &'static str {
        "Self-hosted Bitwarden-compatible server, written in Rust"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::IdentityAndAccess
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(
            PortType::new_tcp(8000),
            "/manifest.json",
            "Vaultwarden Web",
            None,
        )
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Vaultwarden)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Vaultwarden>));
