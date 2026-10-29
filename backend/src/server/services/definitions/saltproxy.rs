use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::saltmaster::{
    SALT_PUBLISH_PORT, SALT_REQUEST_PORT, SALT_SSH_PORT,
};
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{ClientProbe, Pattern};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct SaltProxy;

impl ServiceDefinition for SaltProxy {
    fn name(&self) -> &'static str {
        "Salt Proxy"
    }
    fn description(&self) -> &'static str {
        "A Salt Proxy server acts as a proxy between the Salt Master and the minions."
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Development
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    /// The same two ZeroMQ ports as a master, plus Salt SSH on 8022 — the port that distinguishes
    /// the two.
    ///
    /// The ZMTP probe is registered by [`SaltMaster`], since a port may only be claimed once, and
    /// the evidence it produces is keyed by [`ClientProbe`] rather than by which definition asked
    /// for it.
    ///
    /// [`SaltMaster`]: crate::server::services::definitions::saltmaster::SaltMaster
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::Port(PortType::new_tcp(SALT_PUBLISH_PORT)),
            Pattern::Port(PortType::new_tcp(SALT_REQUEST_PORT)),
            Pattern::ClientResponse(ClientProbe::Zmtp),
            Pattern::Port(PortType::new_tcp(SALT_SSH_PORT)),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::SaltProject)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<SaltProxy>));
