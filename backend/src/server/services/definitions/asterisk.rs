use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Asterisk;

impl ServiceDefinition for Asterisk {
    fn name(&self) -> &'static str {
        "Asterisk"
    }
    fn description(&self) -> &'static str {
        "PBX and VoIP phone server"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Telephony
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortType::new_tcp(8088), "/httpstatus", "asterisk", None)
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::Asterisk)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Asterisk>));
