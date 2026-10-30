use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Jupyter;

impl ServiceDefinition for Jupyter {
    fn name(&self) -> &'static str {
        "Jupyter"
    }
    fn description(&self) -> &'static str {
        "Jupyter notebook server or JupyterHub"
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
                ("Jupyter", "JupyterHub"),
                ("Jupyter", "Jupyter Server"),
                ("Jupyter", "Notebook"),
                ("Jupyter", "BinderHub"),
            ],
            &[8000, 8888],
        )
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Jupyter>));
