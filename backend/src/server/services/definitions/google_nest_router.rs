use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{ClientProbe, Pattern, Vendor};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct GoogleHome;

impl ServiceDefinition for GoogleHome {
    fn name(&self) -> &'static str {
        "Google Nest Router"
    }

    fn description(&self) -> &'static str {
        "Google Nest Wifi router"
    }

    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkAccess
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Is(DeviceType::Router)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::AnyOf(vec![
                Pattern::MacVendor(Vendor::NEST),
                Pattern::MacVendor(Vendor::GOOGLE),
            ]),
            Pattern::IsGateway,
            // The root serves the "Nest Wifi" page on `/` and also answers the status API the
            // Google Wifi probe reads (the probe is registered by the repeater definition and runs
            // against every port-80 host). Match on either, so the probe alone can type the root
            // when the `/` page is not reached. `IsGateway` keeps this to the root; the repeater's
            // `Not(IsGateway)` takes the leaves.
            Pattern::AnyOf(vec![
                Pattern::Endpoint(PortType::Http, "/", "Nest Wifi", None),
                Pattern::ClientResponse(ClientProbe::GoogleWifi),
            ]),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::GoogleHome)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<GoogleHome>));
