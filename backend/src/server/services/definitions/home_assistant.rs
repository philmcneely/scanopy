use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct HomeAssistant;

impl ServiceDefinition for HomeAssistant {
    fn name(&self) -> &'static str {
        "Home Assistant"
    }
    fn description(&self) -> &'static str {
        "Open-source home automation platform"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::HomeAutomation
    }

    /// Also binds 1400, where the Sonos integration listens for the speakers' event callbacks: "the
    /// Sonos devices must be able to connect back to the Home Assistant host on TCP port 1400"
    /// (Home Assistant's Sonos integration docs, Network requirements).
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::new_tcp(8123), "/", "home assistant", None)
            .also_binding(Pattern::Port(PortType::new_tcp(1400)))
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::HomeAssistant)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<HomeAssistant>
));
