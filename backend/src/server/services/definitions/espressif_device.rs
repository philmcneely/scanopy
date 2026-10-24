use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{Pattern, Vendor};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct EspressifDevice;

impl ServiceDefinition for EspressifDevice {
    fn name(&self) -> &'static str {
        "Espressif Device"
    }
    fn description(&self) -> &'static str {
        "Web server on vendor firmware for an ESP32 or ESP8266"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::IoT
    }
    // The product behind the chip is unknown; a specific definition (ESPHome, a vendor's own)
    // names it when one matches.
    fn is_generic(&self) -> bool {
        true
    }

    // ESP-IDF's `esp_http_server` answers a path with no handler with a 404 whose body is
    // "This URI does not exist", and sends no `Server` header. Vendor firmware that registers
    // only its own API paths leaves `/` to that default.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::MacVendor(Vendor::ESPRESSIF),
            Pattern::Endpoint(
                PortType::Http,
                "/",
                "This URI does not exist",
                Some(404..405),
            ),
        ])
    }

    fn logo_url(&self) -> &'static str {
        "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/svg/espressif.svg"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<EspressifDevice>
));
