use crate::server::hosts::r#impl::device_type::DeviceType;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::{DeviceTypeImplication, ServiceDefinition};
use crate::server::services::r#impl::patterns::Pattern;
use crate::server::shared::logos::Logo;

/// The printer's TLS listeners: FTPS on 990, MQTT control on 8883, and the slicer's bind port on
/// 3002. Each presents the unit's certificate under Bambu's own CA.
const BAMBU_TLS_PORTS: &[u16] = &[990, 8883, 3002];

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
    /// company, on every TLS port the printer opens (FTPS on 990 among them). Once one has, its
    /// other LAN ports are bound too, whether or not their own certificate was read this scan: the
    /// slicer's bind ports 3000 (TLS 1.3, presenting nothing) and 3002, and the LAN video stream
    /// on 6000.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::CertificateIssuer("O=BBL Technologies", BAMBU_TLS_PORTS),
            Pattern::Optional(Box::new(Pattern::AnyOf(vec![
                Pattern::Port(PortType::Http3000),
                Pattern::Port(PortType::new_tcp(3002)),
                Pattern::Port(PortType::new_tcp(6000)),
            ]))),
        ])
    }

    fn logo(&self) -> Option<Logo> {
        Some(Logo::BambuLab)
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<BambuLabPrinter>
));
