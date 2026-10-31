use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// Matched by Recog's fingerprints of its web interface (`Pattern::Recog`).
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct TwonkyMediaServer;

impl ServiceDefinition for TwonkyMediaServer {
    fn name(&self) -> &'static str {
        "Twonky Media Server"
    }
    fn description(&self) -> &'static str {
        "DLNA media server"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Media
    }
    // Embedded in NAS and router firmware, and also installed on PCs.
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Suggests(DeviceType::MediaServer)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Recog(&[("Lynx Technology", "Twonky Media Server")], &[9000])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Twonky)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<TwonkyMediaServer>
));
