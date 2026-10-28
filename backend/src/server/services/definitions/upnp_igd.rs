use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct UpnpIgd;

impl ServiceDefinition for UpnpIgd {
    fn name(&self) -> &'static str {
        "UPnP IGD"
    }
    fn description(&self) -> &'static str {
        "UPnP Internet Gateway Device: lets LAN clients open port forwards on the router"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkCore
    }
    // MiniUPnPd runs on routers of every brand.
    fn is_generic(&self) -> bool {
        true
    }

    // MiniUPnPd's HTTP control port names itself in `Server` on any response, a 404 for `/`
    // included (seen on a Google Nest Wifi router on 5000).
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Header(
            Some(PortType::Http5000),
            "server",
            "MiniUPnPd",
            Some(200..500),
        )
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<UpnpIgd>));
