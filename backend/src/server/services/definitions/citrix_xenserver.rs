use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct CitrixXenServer;

impl ServiceDefinition for CitrixXenServer {
    fn name(&self) -> &'static str {
        "Citrix Hypervisor"
    }
    fn description(&self) -> &'static str {
        "Citrix Hypervisor, formerly XenServer"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Hypervisor
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Citrix", "XenServer")], &[443])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<CitrixXenServer>
));
