use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Dawarich;

impl ServiceDefinition for Dawarich {
    fn name(&self) -> &'static str {
        "Dawarich"
    }
    fn description(&self) -> &'static str {
        "Self-hosted location history tracker."
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::HomeAutomation
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    /// `/api/v1/health` needs no API key, and every API response carries `X-Dawarich-Version`.
    /// The endpoint makes the scanner fetch that path; a bare `{"status":"ok"}` from another app on
    /// 3000 lacks the header.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::Endpoint(
                PortType::Http3000,
                "/api/v1/health",
                "\"status\":\"ok\"",
                None,
            ),
            Pattern::Header(Some(PortType::Http3000), "x-dawarich-version", "", None),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Dawarich)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Dawarich>));
