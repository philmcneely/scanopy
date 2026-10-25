use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Emby;

impl ServiceDefinition for Emby {
    fn name(&self) -> &'static str {
        "Emby"
    }
    fn description(&self) -> &'static str {
        "Personal media server with streaming capabilities"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Media
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(
            PortType::new_tcp(8096),
            "/emby/System/Info/Public",
            "Emby",
            None,
        )
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Emby)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Emby>));
