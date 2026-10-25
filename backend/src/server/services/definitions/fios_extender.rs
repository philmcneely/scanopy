use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct FiosExtender;

impl ServiceDefinition for FiosExtender {
    fn name(&self) -> &'static str {
        "Fios Extender"
    }
    fn description(&self) -> &'static str {
        "Fios device providing mesh networking services"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkAccess
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::Endpoint(PortType::Http, "/#/login/", "fios", None),
            Pattern::Not(Box::new(Pattern::IsGateway)),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Fios)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<FiosExtender>
));
