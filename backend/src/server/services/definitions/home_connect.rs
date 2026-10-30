use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{Pattern, Vendor};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct HomeConnect;

impl ServiceDefinition for HomeConnect {
    fn name(&self) -> &'static str {
        "Home Connect Appliance"
    }
    fn description(&self) -> &'static str {
        "Bosch, Siemens or other BSH home appliance with Home Connect"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::IoT
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Is(DeviceType::IoT)
    }

    // The appliance serves the Home Connect local protocol as a websocket at `/homeconnect` on 80;
    // a plain GET gets only an embedded web server's generic 404, so nothing in a response
    // identifies it. The BSH OUI does: it is registered to the appliance maker alone.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::MacVendor(Vendor::BSH),
            Pattern::Port(PortType::Http),
        ])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<HomeConnect>));
