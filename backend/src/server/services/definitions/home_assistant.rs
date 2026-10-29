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

    /// Also binds the ports Home Assistant opens beside its UI:
    /// - 1400, where the Sonos integration listens for the speakers' event callbacks: "the Sonos
    ///   devices must be able to connect back to the Home Assistant host on TCP port 1400" (Sonos
    ///   integration docs, Network requirements).
    /// - 4357, the Supervisor's Observer, a status page titled "Home Assistant observer".
    /// - 18555, the built-in go2rtc's WebRTC port: go2rtc's default 8555, prefixed by 1 (go2rtc
    ///   integration docs).
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::Endpoint(PortType::new_tcp(8123), "/", "home assistant", None),
            Pattern::Optional(Box::new(Pattern::AnyOf(vec![
                Pattern::Port(PortType::new_tcp(1400)),
                Pattern::Endpoint(
                    PortType::new_tcp(4357),
                    "/",
                    "home assistant observer",
                    None,
                ),
                Pattern::Port(PortType::new_tcp(18555)),
            ]))),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::HomeAssistant)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<HomeAssistant>
));
