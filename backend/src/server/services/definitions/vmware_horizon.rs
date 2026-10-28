use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct VmwareHorizon;

impl ServiceDefinition for VmwareHorizon {
    fn name(&self) -> &'static str {
        "VMware Horizon"
    }
    fn description(&self) -> &'static str {
        "VMware Horizon virtual desktop connection server"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::RemoteAccess
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("VMware", "Horizon"), ("VMware", "View")], &[443])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::VMware)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<VmwareHorizon>
));
