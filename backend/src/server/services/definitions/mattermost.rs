use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Mattermost;

impl ServiceDefinition for Mattermost {
    fn name(&self) -> &'static str {
        "Mattermost"
    }
    fn description(&self) -> &'static str {
        "Team messaging platform"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Messaging
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        // The ping reply is JSON whose `status` reads `OK` (`api4/system.go`).
        Pattern::Endpoint(
            PortType::new_tcp(8065),
            "/api/v4/system/ping",
            "\"status\":\"OK\"",
            None,
        )
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::Mattermost)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Mattermost>));
