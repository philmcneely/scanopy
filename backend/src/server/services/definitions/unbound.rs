use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::unbound_control::UnboundControlProbe;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{ClientProbe, Pattern};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Unbound;

impl ServiceDefinition for Unbound {
    fn name(&self) -> &'static str {
        "Unbound DNS"
    }
    fn description(&self) -> &'static str {
        "Recursive DNS resolver with control ip_address"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::DNS
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    /// The DNS service, qualified by a TLS listener on the remote-control port. Unbound's control
    /// certificate is self-signed with nothing dependable to match on, so this establishes that
    /// 8953 speaks TLS rather than what it is — which is still more than a bare connect, and is
    /// what a middlebox cannot fake.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::Port(PortType::DnsUdp),
            Pattern::ClientResponse(ClientProbe::Tls),
        ])
    }
    fn app_probes(&self) -> Vec<Box<dyn AppProbe>> {
        vec![Box::new(UnboundControlProbe)]
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Unbound)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Unbound>));
