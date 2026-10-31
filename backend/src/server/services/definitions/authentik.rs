use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Authentik;

impl ServiceDefinition for Authentik {
    fn name(&self) -> &'static str {
        "Authentik"
    }
    fn description(&self) -> &'static str {
        "A self-hosted, open source identity provider"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::IdentityAndAccess
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AnyOf(vec![
            Pattern::Endpoint(PortType::Http9000, "/", "window.authentik", None),
            Pattern::Endpoint(PortType::Https9443, "/", "window.authentik", None),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Authentik)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Authentik>));
