use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Roundcube;

impl ServiceDefinition for Roundcube {
    fn name(&self) -> &'static str {
        "Roundcube"
    }
    fn description(&self) -> &'static str {
        "Roundcube webmail"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Email
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Roundcube", "Webmail")], &[80, 443])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Roundcube>));
