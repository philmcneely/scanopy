use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::apple_lockdown::AppleLockdownProbe;
use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{Pattern, probe_pattern};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct AppleMobileDevice;

impl ServiceDefinition for AppleMobileDevice {
    fn name(&self) -> &'static str {
        "Apple Mobile Device"
    }
    fn description(&self) -> &'static str {
        "Pairing and Wi-Fi sync service on iPhones and iPads (lockdown)"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Mobile
    }
    // iPhones, iPads and Apple TVs all run it, so the service says "an Apple device" and not which.
    // The probe reads the product type, whose OS (iOS, iPadOS, tvOS) types the host; a service
    // device type would outrank that.
    fn device_type(&self) -> Option<DeviceType> {
        None
    }
    /// Derived from the probe, so a listener on 62078 that does not answer `QueryType` as lockdown
    /// is not claimed as this service.
    fn discovery_pattern(&self) -> Pattern<'_> {
        probe_pattern(&AppleLockdownProbe)
    }
    fn app_probes(&self) -> Vec<Box<dyn AppProbe>> {
        vec![Box::new(AppleLockdownProbe)]
    }
    fn is_generic(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<AppleMobileDevice>
));
