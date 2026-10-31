use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::mqtt::MqttProbe;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{ClientProbe, Pattern};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct MQTT;

impl ServiceDefinition for MQTT {
    fn name(&self) -> &'static str {
        "MQTT"
    }
    fn description(&self) -> &'static str {
        "Generic MQTT broker, such as Mosquitto or EMQX"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::MessageQueue
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }
    /// Matches on the probe's answer, so a listener on this port that does not speak the protocol is
    /// not claimed as this service.
    ///
    /// Once the broker answered on 1883, its other listeners are bound too: MQTT over TLS on 8883,
    /// and MQTT over WebSockets on 1884 and, with TLS, 8884 (Mosquitto's usual layout, as the
    /// Home Assistant add-on ships it). Each answers a WebSocket upgrade with `101` and the `mqtt`
    /// subprotocol, but the 1883 answer already names the broker.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::ClientResponse(ClientProbe::Mqtt),
            Pattern::Optional(Box::new(Pattern::AnyOf(vec![
                Pattern::Port(PortType::MqttTls),
                Pattern::Port(PortType::new_tcp(1884)),
                Pattern::Port(PortType::new_tcp(8884)),
            ]))),
        ])
    }
    fn app_probes(&self) -> Vec<Box<dyn AppProbe>> {
        vec![Box::new(MqttProbe)]
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::Mqtt)
    }
    fn is_generic(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<MQTT>));
