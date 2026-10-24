use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{Pattern, Vendor};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct BrotherPrinter;

impl ServiceDefinition for BrotherPrinter {
    fn name(&self) -> &'static str {
        "Brother Printer"
    }
    fn description(&self) -> &'static str {
        "A Brother printer's embedded web server"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Printer
    }

    // Brother's embedded web server answers with `Server: debut/<version>` (an HL-2270DW redirects
    // `/` to `/printer/main.html`). LPD on 515 is claimed alongside, the way HP Printer does: a
    // bare connect proves nothing, but on a host already identified as the printer it is the
    // printer's queue.
    fn discovery_pattern(&self) -> Pattern<'_> {
        let identified = Pattern::AllOf(vec![
            Pattern::MacVendor(Vendor::BROTHER),
            Pattern::Header(Some(PortType::Http), "server", "debut", None),
        ]);
        Pattern::AnyOf(vec![
            identified.clone(),
            Pattern::AllOf(vec![identified, Pattern::Port(PortType::LdpTcp)]),
        ])
    }

    fn logo_url(&self) -> &'static str {
        "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/svg/printer.svg"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<BrotherPrinter>
));
