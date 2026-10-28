use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct ManageEngineServiceDeskPlus;

impl ServiceDefinition for ManageEngineServiceDeskPlus {
    fn name(&self) -> &'static str {
        "ManageEngine ServiceDesk Plus"
    }
    fn description(&self) -> &'static str {
        "ManageEngine IT service desk"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::ProjectManagement
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(
            &[
                ("ManageEngine", "ManageEngine ServiceDesk Plus"),
                ("ManageEngine", "ServiceDesk Plus MSP"),
            ],
            &[443, 8080],
        )
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<ManageEngineServiceDeskPlus>
));
