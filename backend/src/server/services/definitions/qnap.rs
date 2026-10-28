use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct QNAP;

impl ServiceDefinition for QNAP {
    fn name(&self) -> &'static str {
        "QNAP NAS"
    }
    fn description(&self) -> &'static str {
        "QNAP network attached storage system"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Storage
    }
    fn device_type(&self) -> Option<DeviceType> {
        Some(DeviceType::Nas)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AnyOf(vec![
            // Original pattern: FTP + HTTP/8080 with "QNAP"
            Pattern::AllOf(vec![
                Pattern::Port(PortType::Ftp),
                Pattern::AnyOf(vec![
                    Pattern::Endpoint(PortType::Http, "/", "QNAP", None),
                    Pattern::Endpoint(PortType::Http8080, "/", "QNAP", None),
                ]),
            ]),
            // Alternative: QNAP's distinctive "http server" Server header (FTP may be disabled)
            Pattern::Header(Some(PortType::Https), "server", "http server", None),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Qnap)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<QNAP>));
