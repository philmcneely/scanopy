use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct PiHole;

impl ServiceDefinition for PiHole {
    fn name(&self) -> &'static str {
        "Pi-Hole"
    }
    fn description(&self) -> &'static str {
        "Network-wide ad blocking DNS service"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::AdBlock
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::AnyOf(vec![
                Pattern::Port(PortType::DnsUdp),
                Pattern::Port(PortType::DnsTcp),
            ]),
            // Pi-hole v6's built-in web server serves the admin UI on 443 as well as 80, under a
            // self-signed certificate.
            Pattern::AnyOf(vec![
                Pattern::Endpoint(PortType::Http, "/admin", "pi-hole", None),
                Pattern::Endpoint(PortType::Https, "/admin", "pi-hole", None),
            ]),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::PiHole)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<PiHole>));
