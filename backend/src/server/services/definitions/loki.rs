use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Loki;

impl ServiceDefinition for Loki {
    fn name(&self) -> &'static str {
        "Loki"
    }
    fn description(&self) -> &'static str {
        "Log aggregation system inspired by Prometheus"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AnyOf(vec![Pattern::Endpoint(
            PortType::new_tcp(3100),
            "/metrics",
            "loki_build_info",
            None,
        )])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Loki)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Loki>));
