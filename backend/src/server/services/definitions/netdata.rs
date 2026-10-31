use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Netdata;

impl ServiceDefinition for Netdata {
    fn name(&self) -> &'static str {
        "Netdata"
    }
    fn description(&self) -> &'static str {
        "Real-time performance monitoring"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::new_tcp(19999), "/api/v1/info", "netdata", None)
    }
    fn is_generic(&self) -> bool {
        true
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::Netdata)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Netdata>));
