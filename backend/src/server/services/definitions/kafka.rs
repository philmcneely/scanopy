use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::kafka::KafkaProbe;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{ClientProbe, Pattern};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Kafka;

impl ServiceDefinition for Kafka {
    fn name(&self) -> &'static str {
        "Kafka"
    }
    fn description(&self) -> &'static str {
        "Event streaming platform"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::MessageQueue
    }
    /// Matches on the probe's answer, so a listener on this port that does not speak the protocol is
    /// not claimed as this service.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::ClientResponse(ClientProbe::Kafka)
    }
    fn app_probes(&self) -> Vec<Box<dyn AppProbe>> {
        vec![Box::new(KafkaProbe)]
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::Kafka)
    }
    fn is_generic(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Kafka>));
