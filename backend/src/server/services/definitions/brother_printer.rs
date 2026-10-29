use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{Pattern, Vendor};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct BrotherPrinter;

impl ServiceDefinition for BrotherPrinter {
    fn name(&self) -> &'static str {
        "Brother Printer"
    }
    fn description(&self) -> &'static str {
        "A Brother printer: its web admin page and LPD queue"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Printer
    }
    // Most current Brother network units print, scan and copy.
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Is(DeviceType::MultifunctionDevice)
    }

    // Brother's network products are printers, multifunctions and scanners, so the OUI names the
    // product and an open port gives the service something to bind, as for myQ and Bose. The web
    // server's `Server: debut` header would say the same, but `/` redirects to a page an HL-2270DW
    // took ~470 ms to serve when idle, too close to the scan's 800 ms request budget to rely on.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::MacVendor(Vendor::BROTHER),
            Pattern::AnyOf(vec![
                Pattern::Port(PortType::Http),
                Pattern::Port(PortType::LdpTcp),
            ]),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Brother)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<BrotherPrinter>
));
