use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

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

    // macOS's AirPlay Receiver listens on 5000 and answers a plain GET with 403 and
    // `Server: AirTunes/<version>`.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Header(
            Some(PortType::Http5000),
            "server",
            "AirTunes",
            Some(200..500),
        )
    }

    fn logo_url(&self) -> &'static str {
        "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/svg/apple.svg"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<AirPlayReceiver>
));
