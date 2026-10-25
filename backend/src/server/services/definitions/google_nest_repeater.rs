use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{Pattern, Vendor};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct GoogleHome;

impl ServiceDefinition for GoogleHome {
    fn name(&self) -> &'static str {
        "Google Nest Repeater"
    }

    fn description(&self) -> &'static str {
        "Google Nest Wifi repeater"
    }

    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkAccess
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::AnyOf(vec![
                Pattern::MacVendor(Vendor::NEST),
                Pattern::MacVendor(Vendor::GOOGLE),
            ]),
            Pattern::Not(Box::new(Pattern::IsGateway)),
            // Only the mesh's root unit serves the "Nest Wifi" page on `/`; a point answers `/`
            // with a 404. Every unit serves the status API, where a point reports itself `leaf`
            // (seen on a Google Wifi point, `hardwareId` GALE).
            Pattern::AnyOf(vec![
                Pattern::Endpoint(PortType::Http, "/", "Nest Wifi", None),
                Pattern::Endpoint(
                    PortType::Http,
                    "/api/v1/status",
                    "\"groupRole\": \"leaf\"",
                    None,
                ),
            ]),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::GoogleHome)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<GoogleHome>));
