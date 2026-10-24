use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{DnsSdServiceType, Pattern};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct SpotifyConnect;

impl ServiceDefinition for SpotifyConnect {
    fn name(&self) -> &'static str {
        "Spotify Connect"
    }
    fn description(&self) -> &'static str {
        "Spotify Connect playback receiver"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Media
    }
    // A protocol speakers, receivers and TVs of every brand implement, not a vendor's product.
    fn is_generic(&self) -> bool {
        true
    }

    // The port differs by vendor (Bose 8200, Sonos 1400) and only the advertisement names it, so
    // the advertisement is the pattern; `DnsSd` binds the port its SRV record names. The network
    // scan's `spotify_connect` probe reads the device's brand and model from the same port.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::DnsSd(DnsSdServiceType::SPOTIFY_CONNECT, None)
    }

    fn logo_url(&self) -> &'static str {
        "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/svg/spotify.svg"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<SpotifyConnect>
));
