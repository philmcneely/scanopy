use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Zimbra;

impl ServiceDefinition for Zimbra {
    fn name(&self) -> &'static str {
        "Zimbra"
    }
    fn description(&self) -> &'static str {
        "Zimbra email and collaboration suite"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Email
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(
            &[
                ("Zimbra", "Collaboration"),
                ("Zimbra", "Collaboration Server"),
            ],
            &[443, 7071],
        )
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Zimbra>));
