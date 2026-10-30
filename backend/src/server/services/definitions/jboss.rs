use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct JBoss;

impl ServiceDefinition for JBoss {
    fn name(&self) -> &'static str {
        "JBoss"
    }
    fn description(&self) -> &'static str {
        "Red Hat JBoss and WildFly Java application server"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Development
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(
            &[
                ("Red Hat", "JBoss AS"),
                ("Red Hat", "JBoss EAP"),
                ("Red Hat", "JBoss Server"),
                ("Red Hat", "WildFly"),
            ],
            &[8080, 9990],
        )
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<JBoss>));
