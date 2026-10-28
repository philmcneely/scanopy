use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct TrueNAS;

impl ServiceDefinition for TrueNAS {
    fn name(&self) -> &'static str {
        "TrueNAS"
    }
    fn description(&self) -> &'static str {
        "Open-source network attached storage system, formerly FreeNAS"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Storage
    }
    fn device_type(&self) -> Option<DeviceType> {
        Some(DeviceType::Nas)
    }

    // Every webui release from 12.0 (CORE) through SCALE carries `<title id="main-page-title">`
    // in index.html. SCALE's index.html has no "TrueNAS" text at all: the title is set by script.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AnyOf(vec![
            Pattern::Endpoint(PortType::Http, "/", "main-page-title", None),
            Pattern::Endpoint(PortType::Https, "/", "main-page-title", None),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::TrueNas)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<TrueNAS>));
