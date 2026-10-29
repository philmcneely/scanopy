use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct BigBlueButton;

impl ServiceDefinition for BigBlueButton {
    fn name(&self) -> &'static str {
        "BigBlueButton"
    }
    fn description(&self) -> &'static str {
        "Web conferencing system"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Conferencing
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        // Every API reply is `<response><returncode>…`, the index included.
        Pattern::Endpoint(
            PortType::Http,
            "/bigbluebutton/api",
            "<returncode>",
            Some(200..300),
        )
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::BigBlueButton)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<BigBlueButton>
));
