use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct ApacheSuperset;

impl ServiceDefinition for ApacheSuperset {
    fn name(&self) -> &'static str {
        "Apache Superset"
    }
    fn description(&self) -> &'static str {
        "Data exploration and visualization platform"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Dashboard
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Apache", "Superset")], &[8088])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<ApacheSuperset>
));
