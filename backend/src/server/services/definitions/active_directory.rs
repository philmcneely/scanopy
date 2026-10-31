use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{ClientProbe, Pattern};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct ActiveDirectory;

impl ServiceDefinition for ActiveDirectory {
    fn name(&self) -> &'static str {
        "Active Directory"
    }
    fn description(&self) -> &'static str {
        "Microsoft directory service"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::IdentityAndAccess
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::ClientResponse(ClientProbe::Ldap),
            Pattern::ClientResponse(ClientProbe::Smb),
            Pattern::Port(PortType::Kerberos),
        ])
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::Microsoft)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<ActiveDirectory>
));
