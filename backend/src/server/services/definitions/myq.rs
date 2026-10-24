use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{Pattern, Vendor};

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

    // The web server on 80 is Marvell's Wi-Fi microcontroller SDK (`Server: Marvell-WM`), which
    // other vendors' devices run too, so the response identifies the chip rather than the product.
    // The Chamberlain OUI identifies the product.
    // Its setup server on 443 presents a self-signed certificate for `setup.myqdevice.com`, issued
    // by `O=CGI` (The Chamberlain Group).
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::MacVendor(Vendor::CHAMBERLAIN),
            Pattern::Port(PortType::Http),
        ])
        .also_binding(Pattern::CertificateIssuer("CN=setup.myqdevice.com", &[443]))
    }

    // No icon set carries myQ; this is the app icon from myq.com. The `context` token is what the
    // site's media server needs to serve it: without it the request gets a 400.
    fn logo_url(&self) -> &'static str {
        "https://www.myq.com/medias/myQ-app-logo.png?context=bWFzdGVyfHJvb3R8MjcxN3xpbWFnZS9wbmd8YURjekwyZzNPUzg1TWpnNU5UVXhNVFV4TVRNMEwyMTVVUzFoY0hBdGJHOW5ieTV3Ym1jfGYwM2U2OWE1OWM3NGE3OTBkYmJlYmE3NmNkNDlhYThhNDg4YjUyZjY4MDVkZjY4OGUzNzBiOGRlOTlkZDM1MGM"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<MyQ>));
