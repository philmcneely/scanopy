use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{Pattern, Vendor};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct UbiquitiDiscovery;

impl ServiceDefinition for UbiquitiDiscovery {
    fn name(&self) -> &'static str {
        "Ubiquiti Discovery"
    }
    fn description(&self) -> &'static str {
        "Ubiquiti device discovery service"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkAccess
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Nothing
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        // The port arm is gone: 10001/udp has no probe, so the network scan never reports it and a
        // declared container port is not evidence. The OUI is the evidence that remains, and it is
        // real — a Ubiquiti MAC is read from an ARP reply on the host's own segment.
        Pattern::MacVendor(Vendor::UBIQUITI)
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Ubiquiti)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<UbiquitiDiscovery>
));
