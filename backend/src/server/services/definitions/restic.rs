use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Restic;

impl ServiceDefinition for Restic {
    fn name(&self) -> &'static str {
        "Restic"
    }
    fn description(&self) -> &'static str {
        "Fast and secure backup program"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Backup
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::Port(PortType::new_tcp(8000)),
            Pattern::Endpoint(PortType::Http, "/", "restic", None),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Restic)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Restic>));
