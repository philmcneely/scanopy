use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::google_wifi::GoogleWifiProbe;
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
        "Google Nest Repeater"
    }

    fn description(&self) -> &'static str {
        "Google Nest Wifi repeater"
    }

    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkAccess
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Is(DeviceType::WlanRepeater)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::AnyOf(vec![
                Pattern::MacVendor(Vendor::NEST),
                Pattern::MacVendor(Vendor::GOOGLE),
            ]),
            Pattern::Not(Box::new(Pattern::IsGateway)),
            // Only the mesh's root unit serves the "Nest Wifi" page on `/`; a point answers `/`
            // with a 404. Every unit serves the status API, which the Google Wifi probe reads for
            // the unit's board and firmware (seen on a Google Wifi point, `hardwareId` GALE); the
            // outer `Not(IsGateway)` keeps the gateway root out of this definition.
            Pattern::AnyOf(vec![
                Pattern::Endpoint(PortType::Http, "/", "Nest Wifi", None),
                Pattern::ClientResponse(ClientProbe::GoogleWifi),
            ]),
        ])
    }

    fn app_probes(&self) -> Vec<Box<dyn AppProbe>> {
        vec![Box::new(GoogleWifiProbe)]
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::GoogleHome)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<GoogleHome>));
