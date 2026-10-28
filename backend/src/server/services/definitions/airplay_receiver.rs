use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct AirPlayReceiver;

impl ServiceDefinition for AirPlayReceiver {
    fn name(&self) -> &'static str {
        "AirPlay Receiver"
    }
    fn description(&self) -> &'static str {
        "Receives AirPlay audio and screen mirroring"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Media
    }
    // Macs, Apple TVs, HomePods and third-party speakers all run it.
    fn is_generic(&self) -> bool {
        true
    }

    // An AirPlay receiver answers a plain GET with an error status and `Server: AirTunes/<version>`:
    // on 7000 for AirPlay 2 (Macs, Sonos and Bose speakers), and on 5000 too on macOS.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AnyOf(
            [PortType::Http5000, PortType::new_tcp(7000)]
                .into_iter()
                .map(|port| Pattern::Header(Some(port), "server", "AirTunes", Some(200..500)))
                .collect(),
        )
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::AirPlay)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<AirPlayReceiver>
));
