use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct OutlookWebAccess;

impl ServiceDefinition for OutlookWebAccess {
    fn name(&self) -> &'static str {
        "Outlook Web Access"
    }
    fn description(&self) -> &'static str {
        "Microsoft Exchange webmail"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Email
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Microsoft", "Outlook Web Access")], &[443])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Microsoft)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<OutlookWebAccess>
));
