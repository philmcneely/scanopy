use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{Pattern, Vendor};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct MyQ;

impl ServiceDefinition for MyQ {
    fn name(&self) -> &'static str {
        "myQ"
    }
    fn description(&self) -> &'static str {
        "Chamberlain myQ garage door opener or hub"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::IoT
    }
    // A garage door opener or its hub.
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Is(DeviceType::AccessControl)
    }

    // The web server on 80 is Marvell's Wi-Fi microcontroller SDK (`Server: Marvell-WM`), which
    // other vendors' devices run too, so the response identifies the chip rather than the product.
    // The Chamberlain OUI identifies the product.
    // Its setup server on 443 presents a self-signed certificate for `setup.myqdevice.com`, issued
    // by `O=CGI` (The Chamberlain Group).
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::MacVendor(Vendor::CHAMBERLAIN),
            Pattern::Port(PortType::Http),
            Pattern::Optional(Box::new(Pattern::CertificateIssuer(
                "CN=setup.myqdevice.com",
                &[443],
            ))),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::MyQ)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<MyQ>));
