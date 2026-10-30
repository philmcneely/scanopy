use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct NATS;

impl ServiceDefinition for NATS {
    fn name(&self) -> &'static str {
        "NATS"
    }
    fn description(&self) -> &'static str {
        "Cloud-native messaging system"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::MessageQueue
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        // `/varz` is the server's JSON stats, keyed by its `server_id` (`server/monitor.go`).
        Pattern::Endpoint(PortType::new_tcp(8222), "/varz", "\"server_id\"", None)
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::Nats)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<NATS>));
