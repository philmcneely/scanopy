use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct IbmSpectrumProtect;

impl ServiceDefinition for IbmSpectrumProtect {
    fn name(&self) -> &'static str {
        "IBM Spectrum Protect"
    }
    fn description(&self) -> &'static str {
        "IBM Spectrum Protect, formerly Tivoli Storage Manager"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Backup
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(
            &[
                ("IBM", "Tivoli Storage Manager"),
                ("IBM", "Tivoli Storage FlashCopy Manager"),
            ],
            &[443, 11090],
        )
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<IbmSpectrumProtect>
));
