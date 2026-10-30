use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct LinkStack;

impl ServiceDefinition for LinkStack {
    fn name(&self) -> &'static str {
        "LinkStack"
    }
    fn description(&self) -> &'static str {
        "A highly customizable link sharing platform"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Dashboard
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::Header(None, "set-cookie", "linkstack_session", None),
            Pattern::Endpoint(PortType::Http8080, "/", "LinkStack", None),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Linkstack)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<LinkStack>));
