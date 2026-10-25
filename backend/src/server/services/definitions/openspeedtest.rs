use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct OpenSpeedTest;

impl ServiceDefinition for OpenSpeedTest {
    fn name(&self) -> &'static str {
        "OpenSpeedTest"
    }
    fn description(&self) -> &'static str {
        "Self-hosted network speed test application"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AnyOf(vec![
            Pattern::Endpoint(PortType::Http3000, "/", "OpenSpeedTest-Server", None),
            Pattern::Endpoint(PortType::new_tcp(3001), "/", "OpenSpeedTest-Server", None),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::OpenSpeedtest)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<OpenSpeedTest>
));
