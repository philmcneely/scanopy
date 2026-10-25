use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Libvirt;

impl ServiceDefinition for Libvirt {
    fn name(&self) -> &'static str {
        "libvirt (KVM)"
    }
    fn description(&self) -> &'static str {
        "Linux KVM/QEMU virtualization managed through libvirt"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Hypervisor
    }

    /// Assigned by hand until the libvirt integration lands with `ClientResponse(ClientProbe::Libvirt)`.
    ///
    /// libvirtd's TCP (16509) and TLS (16514) listeners are off by default, and a listener on
    /// either port alone is the bare-port evidence `proxmox.rs` dropped.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::None
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Libvirt)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Libvirt>));
