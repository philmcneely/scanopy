use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Openfire;

impl ServiceDefinition for Openfire {
    fn name(&self) -> &'static str {
        "Openfire"
    }
    fn description(&self) -> &'static str {
        "Openfire XMPP chat server"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Messaging
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Ignite Realtime", "OpenFire")], &[9090])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Openfire>));
