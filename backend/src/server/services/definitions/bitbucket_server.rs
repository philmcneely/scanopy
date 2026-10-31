use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct BitbucketServer;

impl ServiceDefinition for BitbucketServer {
    fn name(&self) -> &'static str {
        "Bitbucket Server"
    }
    fn description(&self) -> &'static str {
        "Git repository management"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Development
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::new_tcp(7990), "/", "bitbucket", None)
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Bitbucket)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<BitbucketServer>
));
