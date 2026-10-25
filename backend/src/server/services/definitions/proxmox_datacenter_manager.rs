use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct ProxmoxDatacenterManager;

impl ServiceDefinition for ProxmoxDatacenterManager {
    fn name(&self) -> &'static str {
        "Proxmox Datacenter Manager"
    }
    fn description(&self) -> &'static str {
        "A single pane of glass for managing clustered & non-clustered Proxmox nodes"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Dashboard
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::Https8443, "/", "pdm-ui_bundle.js", None)
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Proxmox)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<ProxmoxDatacenterManager>
));
