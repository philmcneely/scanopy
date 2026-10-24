use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct UptimeKuma;

impl ServiceDefinition for UptimeKuma {
    fn name(&self) -> &'static str {
        "UptimeKuma"
    }
    fn description(&self) -> &'static str {
        "Self-hosted uptime monitoring tool"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AnyOf(vec![
            Pattern::Endpoint(PortType::Http, "/", "Uptime Kuma", None),
            Pattern::Endpoint(PortType::new_tcp(3001), "/", "Uptime Kuma", None),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::UptimeKuma)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<UptimeKuma>));
