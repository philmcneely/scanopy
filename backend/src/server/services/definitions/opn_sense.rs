use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct OpnSense;

impl ServiceDefinition for OpnSense {
    fn name(&self) -> &'static str {
        "OPNsense"
    }
    fn description(&self) -> &'static str {
        "Open-source firewall and routing platform"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkAppliance
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::AnyOf(vec![
                Pattern::Endpoint(PortType::Http, "/", "opnsense", None),
                Pattern::Endpoint(PortType::Https, "/", "opnsense", None),
            ]),
            Pattern::AnyOf(vec![
                Pattern::Port(PortType::DnsTcp),
                Pattern::Port(PortType::DnsUdp),
                Pattern::Port(PortType::Ssh),
                Pattern::Port(PortType::Ntp),
                Pattern::Port(PortType::Dhcp),
            ]),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::OpnSense)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<OpnSense>));
