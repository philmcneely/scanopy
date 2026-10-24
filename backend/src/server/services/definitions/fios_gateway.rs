use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct FiosGateway;

impl ServiceDefinition for FiosGateway {
    fn name(&self) -> &'static str {
        "Fios Gateway"
    }
    fn description(&self) -> &'static str {
        "Fios device providing routing and gateway services"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkAccess
    }
    fn device_type(&self) -> Option<DeviceType> {
        Some(DeviceType::Router)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::Endpoint(PortType::Http, "/#/login/", "fios", None),
            Pattern::IsGateway,
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Fios)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<FiosGateway>));
