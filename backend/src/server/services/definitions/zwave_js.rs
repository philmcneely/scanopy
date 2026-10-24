use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct ZWaveJS;

impl ServiceDefinition for ZWaveJS {
    fn name(&self) -> &'static str {
        "Z-Wave JS"
    }
    fn description(&self) -> &'static str {
        "Z-Wave controller server"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::HomeAutomation
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        // `/health` answers a bare `Ok`, which any catch-all page can contain; the UI's own page is
        // titled "Z-Wave JS UI".
        Pattern::Endpoint(PortType::new_tcp(8091), "/", "Z-Wave JS UI", None)
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::ZWaveJsUi)
    }
    fn is_generic(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<ZWaveJS>));
