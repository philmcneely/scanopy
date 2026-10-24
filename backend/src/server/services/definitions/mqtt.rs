use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::mqtt::MqttProbe;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{Pattern, probe_pattern};
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
    /// Derived from the probe, so a listener on this port that does not speak the protocol is
    /// not claimed as this service.
    fn discovery_pattern(&self) -> Pattern<'_> {
        probe_pattern(&MqttProbe)
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
