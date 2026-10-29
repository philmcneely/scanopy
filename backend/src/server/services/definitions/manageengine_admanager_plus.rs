use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct ManageEngineAdManagerPlus;

impl ServiceDefinition for ManageEngineAdManagerPlus {
    fn name(&self) -> &'static str {
        "ManageEngine ADManager Plus"
    }
    fn description(&self) -> &'static str {
        "ManageEngine Active Directory management"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::IdentityAndAccess
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(
            &[
                ("ManageEngine", "ADManager Plus"),
                ("ManageEngine", "AD360"),
            ],
            &[8080, 8443],
        )
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<ManageEngineAdManagerPlus>
));
