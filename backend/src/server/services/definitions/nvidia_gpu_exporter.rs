use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct NvidiaGpuExporter;

impl ServiceDefinition for NvidiaGpuExporter {
    fn name(&self) -> &'static str {
        "Nvidia GPU Exporter"
    }
    fn description(&self) -> &'static str {
        "Prometheus exporter for Nvidia GPU metrics"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Monitoring
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(
            PortType::new_tcp(9835),
            "/metrics",
            "nvidia_gpu_exporter",
            None,
        )
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::Nvidia)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<NvidiaGpuExporter>
));
