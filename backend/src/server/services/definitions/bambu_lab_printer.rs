use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::{DeviceTypeImplication, ServiceDefinition};
use crate::server::services::r#impl::patterns::Pattern;

/// The printer's TLS listeners: MQTT control on 8883, and the slicer's bind port on 3002. Both
/// present the unit's certificate under Bambu's own CA.
const BAMBU_TLS_PORTS: &[u16] = &[8883, 3002];

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct BambuLabPrinter;

impl ServiceDefinition for BambuLabPrinter {
    fn name(&self) -> &'static str {
        "Bambu Lab Printer"
    }
    fn description(&self) -> &'static str {
        "Bambu Lab 3D printer"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Printer
    }
    fn device_type(&self) -> DeviceTypeImplication {
        DeviceTypeImplication::Is(DeviceType::Printer)
    }
    /// The certificates name the issuer `CN=BBL CA, O=BBL Technologies Co., Ltd`, Bambu Lab's
    /// company. Once one has, the slicer's other bind port, 3000, is bound too: it speaks TLS 1.3
    /// and presents nothing to read.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::CertificateIssuer("O=BBL Technologies", BAMBU_TLS_PORTS),
            Pattern::Optional(Box::new(Pattern::Port(PortType::Http3000))),
        ])
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<BambuLabPrinter>
));
