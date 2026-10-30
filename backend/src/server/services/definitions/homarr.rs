use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Homarr;

impl ServiceDefinition for Homarr {
    fn name(&self) -> &'static str {
        "Homarr"
    }
    fn description(&self) -> &'static str {
        "A sleek, modern dashboard"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Dashboard
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    /// A fresh install redirects `/` to `/init` and a configured one serves the dashboard; both
    /// pages name the application, and the endpoint scan follows redirects.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::new_tcp(7575), "/", "Homarr", None)
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Homarr)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Homarr>));
