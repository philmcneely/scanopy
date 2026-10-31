use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::DeviceTypeImplication;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct MinIO;

impl ServiceDefinition for MinIO {
    fn name(&self) -> &'static str {
        "MinIO"
    }
    fn description(&self) -> &'static str {
        "Object storage"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Storage
    }
    // Software on a general-purpose host, as Samba and NFS are.
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Server
    }
    fn discovery_pattern(&self) -> Pattern<'_> {
        // The liveness probe answers with an empty body, so the `Server: MinIO` header every MinIO
        // response carries (`cmd/api-headers.go`) is what names it.
        Pattern::AllOf(vec![
            Pattern::Endpoint(PortType::Http9000, "/minio/health/live", "", Some(200..300)),
            Pattern::Header(Some(PortType::Http9000), "server", "MinIO", Some(200..300)),
        ])
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::Minio)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<MinIO>));
