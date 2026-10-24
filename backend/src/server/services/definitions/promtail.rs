use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Promtail;

impl ServiceDefinition for Promtail {
    fn name(&self) -> &'static str {
        "Promtail"
    }
    fn description(&self) -> &'static str {
        "Agent which ships logs to Grafana Loki"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AnyOf(vec![
            Pattern::Endpoint(
                PortType::new_tcp(9080),
                "/metrics",
                "promtail_build_info",
                None,
            ),
            Pattern::Endpoint(PortType::new_tcp(9080), "/targets", "Promtail", None),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Loki)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Promtail>));
