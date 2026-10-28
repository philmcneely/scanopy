use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct SkypeForBusiness;

impl ServiceDefinition for SkypeForBusiness {
    fn name(&self) -> &'static str {
        "Skype For Business Server"
    }
    fn description(&self) -> &'static str {
        "Microsoft Skype for Business and Lync server"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Conferencing
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(
            &[
                ("Microsoft", "Skype for Business"),
                ("Microsoft", "Lync Server"),
            ],
            &[443],
        )
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Microsoft)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<SkypeForBusiness>
));
