use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Ceph;

impl ServiceDefinition for Ceph {
    fn name(&self) -> &'static str {
        "Ceph"
    }
    fn description(&self) -> &'static str {
        "Distributed storage"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Storage
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::Http8080, "/", "ceph dashboard", None)
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::Ceph)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Ceph>));
