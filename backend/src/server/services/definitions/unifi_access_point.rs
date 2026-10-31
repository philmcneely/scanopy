use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{Pattern, UnifiDeviceType, Vendor};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct UnifiAccessPoint;

impl ServiceDefinition for UnifiAccessPoint {
    fn name(&self) -> &'static str {
        "Unifi Access Point"
    }
    fn description(&self) -> &'static str {
        "Ubiquiti UniFi wireless access point"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkAccess
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Is(DeviceType::AccessPoint)
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        // Either the AP answers an HTTP probe itself, or a UniFi controller we authenticated
        // to reports it as an AP. Adopted APs commonly serve no useful management page, so
        // the controller is the only evidence available for most of them.
        Pattern::AllOf(vec![
            Pattern::MacVendor(Vendor::UBIQUITI),
            Pattern::AnyOf(vec![
                Pattern::Endpoint(PortType::Http, "/", "Unifi", None),
                Pattern::ManagedDeviceType(UnifiDeviceType::ACCESS_POINT),
            ]),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::UniFi)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<UnifiAccessPoint>
));
