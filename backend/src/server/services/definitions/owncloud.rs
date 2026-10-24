use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct OwnCloud;

impl ServiceDefinition for OwnCloud {
    fn name(&self) -> &'static str {
        "ownCloud"
    }
    fn description(&self) -> &'static str {
        "File sync and share"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Office
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::Http, "/status.php", "owncloud", None)
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::Owncloud)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<OwnCloud>));
