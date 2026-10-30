use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{Pattern, Vendor};
use crate::server::shared::logos::Logo;

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
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Is(DeviceType::IoT)
    }
    // The product behind the chip is unknown; a specific definition (ESPHome, a vendor's own)
    // names it when one matches.
    fn is_generic(&self) -> bool {
        true
    }

    // ESP-IDF's `esp_http_server` answers a path with no handler with a 404 whose body is its
    // default message (`httpd_resp_send_err` in `httpd_txrx.c`), and sends no `Server` header.
    // The message is "This URI does not exist" through v4.4 and "Nothing matches the given URI"
    // from v5.0. Vendor firmware that registers only its own API paths leaves `/` to that default.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::MacVendor(Vendor::ESPRESSIF),
            Pattern::AnyOf(vec![
                Pattern::Endpoint(
                    PortType::Http,
                    "/",
                    "This URI does not exist",
                    Some(404..405),
                ),
                Pattern::Endpoint(
                    PortType::Http,
                    "/",
                    "Nothing matches the given URI",
                    Some(404..405),
                ),
            ]),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Espressif)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<EspressifDevice>
));
