use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Karakeep;

impl ServiceDefinition for Karakeep {
    fn name(&self) -> &'static str {
        "Karakeep"
    }
    fn description(&self) -> &'static str {
        "The Bookmark Everything App"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Media
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::Http3000, "/manifest.json", "Karakeep", None)
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Karakeep)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Karakeep>));
