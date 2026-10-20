use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::{ConnectOnly, ServiceDefinition};
use crate::server::services::r#impl::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Iscsi;

impl ServiceDefinition for Iscsi {
    fn name(&self) -> &'static str {
        "iSCSI Target"
    }
    fn description(&self) -> &'static str {
        "Block storage served over the network with iSCSI"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Storage
    }
    fn device_type(&self) -> Option<DeviceType> {
        Some(DeviceType::Server)
    }

    /// The target speaks only after a Login request naming an initiator, and a login is a session
    /// the target logs and may refuse or count against its limits. Nothing short of that tells it
    /// apart from another listener on 3260.
    fn connect_only_rationale(&self) -> Option<ConnectOnly> {
        Some(ConnectOnly::NoDistinguishingHandshake)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Port(PortType::new_tcp(3260))
    }

    fn is_generic(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Iscsi>));
