use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct PrintServer;

impl ServiceDefinition for PrintServer {
    fn name(&self) -> &'static str {
        "Print Server"
    }
    fn description(&self) -> &'static str {
        "A generic printing service"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Printer
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    /// IPP is HTTP, and a CUPS server names itself in the `Server` header on every response
    /// (`CUPS/2.4 IPP/2.1`).
    ///
    /// The LPD arms are gone. 515/udp has no probe, so the network scan never reports it and a
    /// container that declares it is not evidence. 515/tcp rested on a bare connect, which any
    /// middlebox in the path satisfies. A daemon that answers the LPD probe is the LPD
    /// definition's.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Header(Some(PortType::Ipp), "server", "CUPS", None)
    }

    fn is_generic(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<PrintServer>));
