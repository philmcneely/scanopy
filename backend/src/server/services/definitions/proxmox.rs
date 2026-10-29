use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{ClientProbe, Pattern};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Proxmox;

impl ServiceDefinition for Proxmox {
    fn name(&self) -> &'static str {
        "Proxmox VE"
    }
    fn description(&self) -> &'static str {
        "Open-source KVM and LXC virtualization management platform"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Hypervisor
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Is(DeviceType::Hypervisor)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        // The bare-port arm is gone. As an `AnyOf` alternative it meant any listener on 8006 was
        // claimed as Proxmox whether or not the page said so, which made the endpoint match beside
        // it decorative.
        //
        // The credentialed probe is the second way in: an API token that answers `/version` is a
        // Proxmox VE node whatever its web UI serves, and `execute_integrations` requires this
        // service to be matched before the Proxmox integration may run.
        Pattern::AnyOf(vec![
            Pattern::Endpoint(PortType::new_tcp(8006), "/", "proxmox", None),
            Pattern::ClientResponse(ClientProbe::Proxmox),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Proxmox)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Proxmox>));
