use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{Pattern, Vendor};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct TpLinkEap;

impl ServiceDefinition for TpLinkEap {
    fn name(&self) -> &'static str {
        "TP-Link EAP"
    }
    fn description(&self) -> &'static str {
        "TP-Link EAP wireless access point"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkAccess
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::AnyOf(vec![
                Pattern::MacVendor(Vendor::TPLINK),
                Pattern::MacVendor(Vendor::TPLINK_SYSTEMS),
            ]),
            Pattern::Endpoint(PortType::Http, "/", "tp-link", None),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::TpLink)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<TpLinkEap>));
