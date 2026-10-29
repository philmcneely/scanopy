use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct NginxProxyManager;

impl ServiceDefinition for NginxProxyManager {
    fn name(&self) -> &'static str {
        "Nginx Proxy Manager"
    }
    fn description(&self) -> &'static str {
        "Web-based Nginx proxy management ip_address"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::ReverseProxy
    }

    /// The default site on 80 and the admin UI on 81 both name the product. Once one has, the
    /// proxy's TLS listener on 443 and the backend API on 3000 are bound too. The API answers `/`
    /// with its status and version (`{"status":"OK","setup":true,"version":{…,"revision":3}}`,
    /// `backend/routes/main.js`), which says nothing about the product without the page beside it.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::AnyOf(vec![
                Pattern::Endpoint(PortType::Http, "", "nginx proxy manager", None),
                Pattern::Endpoint(PortType::new_tcp(81), "", "nginx proxy manager", None),
            ]),
            Pattern::Optional(Box::new(Pattern::AnyOf(vec![
                Pattern::Port(PortType::Https),
                Pattern::Endpoint(PortType::Http3000, "/", "\"revision\"", None),
            ]))),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::NginxProxyManager)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<NginxProxyManager>
));
