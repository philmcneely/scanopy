use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{ClientProbe, MatchConfidence, Pattern};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Switch;

impl ServiceDefinition for Switch {
    fn name(&self) -> &'static str {
        "Switch"
    }
    fn description(&self) -> &'static str {
        "Generic network switch for local area networking"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::NetworkCore
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Is(DeviceType::Switch)
    }

    // A web admin page and a Telnet console are what a managed switch shows, but
    // printers, cameras, phones, NAS units and Windows hosts can show both too. A port that names
    // one of those devices rules the host out. Industrial protocols do not: managed industrial
    // switches serve EtherNet/IP and Modbus themselves.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::Not(Box::new(Pattern::IsGateway)),
            Pattern::AllOf(vec![
                Pattern::Port(PortType::Http),
                Pattern::ClientResponse(ClientProbe::Telnet),
            ]),
            Pattern::Custom(
                |params| {
                    !params
                        .baseline_params
                        .all_ports
                        .iter()
                        .any(names_another_device)
                },
                |_| Vec::new(),
                "No port names a printer, camera, phone, file server or desktop",
                "A port names a printer, camera, phone, file server or desktop",
                MatchConfidence::Low,
            ),
        ])
    }

    fn is_generic(&self) -> bool {
        true
    }
}

/// Whether `port` is a service an end device runs and a switch does not.
fn names_another_device(port: &PortType) -> bool {
    port.is_printing()
        || port.is_media_streaming()
        || port.is_voip()
        || port.is_file_sharing()
        || port.is_remote_desktop()
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Switch>));
