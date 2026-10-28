use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{Pattern, Vendor};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct TpLinkKasa;

impl ServiceDefinition for TpLinkKasa {
    fn name(&self) -> &'static str {
        "TP-Link Kasa"
    }
    fn description(&self) -> &'static str {
        "TP-Link Kasa smart plug or switch"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::IoT
    }
    fn device_type(&self) -> Option<DeviceType> {
        Some(DeviceType::PowerRelay)
    }

    // Kasa's local HTTP API on 80 answers with `Server: SHIP 2.0` (seen on an HS103 and a KS200)
    // and a bare "200 OK" page, so the header is the only thing to match.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::AnyOf(vec![
                Pattern::MacVendor(Vendor::TPLINK),
                Pattern::MacVendor(Vendor::TPLINK_SYSTEMS),
            ]),
            Pattern::Header(Some(PortType::Http), "server", "SHIP 2.0", None),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::TpLink)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<TpLinkKasa>));
