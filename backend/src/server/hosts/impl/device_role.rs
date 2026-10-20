use crate::server::hosts::r#impl::base::Host;
use crate::server::services::r#impl::base::Service;
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::shared::entities::EntityDiscriminants;
use crate::server::shared::types::metadata::{EntityMetadataProvider, HasId, TypeMetadataProvider};
use crate::server::shared::types::{Color, Icon};
use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, IntoStaticStr};
use utoipa::ToSchema;

/// What a host is, as a device: the class its icon stands for on the map.
///
/// Declaration order is precedence, highest first. A host whose services claim several roles takes
/// the first one here, so a firewall that also serves files reads as a firewall, and `Server`,
/// which any application implies, never hides a more specific role.
///
/// `Unidentified` means no evidence yet, not a device class outside this list: every class the
/// evidence names gets its own variant.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    ToSchema,
    EnumIter,
    IntoStaticStr,
)]
pub enum DeviceRole {
    Firewall,
    Router,
    Switch,
    AccessPoint,
    Hypervisor,
    Storage,
    Printer,
    Camera,
    Phone,
    Workstation,
    Mobile,
    Industrial,
    IoT,
    Server,
    /// Also what a role this build doesn't know reads as, so a newer server cannot break an older
    /// reader.
    #[default]
    #[serde(other)]
    Unidentified,
}

impl HasId for DeviceRole {
    fn id(&self) -> &'static str {
        self.into()
    }
}

impl EntityMetadataProvider for DeviceRole {
    /// Every role wears the Host entity's colour: a device is a host first, and the glyph alone
    /// names its role. Software takes its category's colour instead, so the two never compete.
    fn color(&self) -> Color {
        EntityDiscriminants::Host.color()
    }

    fn icon(&self) -> Icon {
        match self {
            DeviceRole::Firewall => Icon::BrickWallFire,
            DeviceRole::Router => Icon::Router,
            DeviceRole::Switch => Icon::Network,
            DeviceRole::AccessPoint => Icon::Wifi,
            DeviceRole::Hypervisor => ServiceCategory::Hypervisor.icon(),
            DeviceRole::Storage => Icon::HardDrive,
            DeviceRole::Printer => Icon::Printer,
            DeviceRole::Camera => Icon::Cctv,
            DeviceRole::Phone => Icon::Phone,
            DeviceRole::Workstation => Icon::Monitor,
            DeviceRole::Mobile => Icon::Smartphone,
            DeviceRole::Industrial => Icon::Factory,
            DeviceRole::IoT => Icon::CircuitBoard,
            DeviceRole::Server => Icon::ServerCog,
            // A host nothing identifies looks like any host: the generic Host icon.
            DeviceRole::Unidentified => EntityDiscriminants::Host.icon(),
        }
    }
}

impl TypeMetadataProvider for DeviceRole {
    fn name(&self) -> &'static str {
        match self {
            DeviceRole::Firewall => "Firewall",
            DeviceRole::Router => "Router",
            DeviceRole::Switch => "Switch",
            DeviceRole::AccessPoint => "Access point",
            DeviceRole::Hypervisor => "Hypervisor",
            DeviceRole::Storage => "Storage",
            DeviceRole::Printer => "Printer",
            DeviceRole::Camera => "Camera",
            DeviceRole::Phone => "Phone",
            DeviceRole::Workstation => "Workstation",
            DeviceRole::Mobile => "Mobile device",
            DeviceRole::Industrial => "Industrial controller",
            DeviceRole::IoT => "IoT device",
            DeviceRole::Server => "Server",
            DeviceRole::Unidentified => "Unidentified",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            DeviceRole::Firewall => "Filters traffic between networks",
            DeviceRole::Router => "Routes traffic between networks",
            DeviceRole::Switch => "Forwards frames between ports on one network",
            DeviceRole::AccessPoint => "Bridges wireless clients onto the network",
            DeviceRole::Hypervisor => "Runs virtual machines",
            DeviceRole::Storage => "Holds files or blocks for other hosts",
            DeviceRole::Printer => "Prints or scans",
            DeviceRole::Camera => "Streams video",
            DeviceRole::Phone => "Carries voice calls",
            DeviceRole::Workstation => "A computer a person works at",
            DeviceRole::Mobile => "A phone or tablet",
            DeviceRole::Industrial => "Controls building or industrial equipment",
            DeviceRole::IoT => "A smart-home or embedded device",
            DeviceRole::Server => "Runs applications for other hosts",
            DeviceRole::Unidentified => "Nothing found on this host names what it is",
        }
    }
}

impl Host {
    /// The device role this host's icon shows.
    ///
    /// The ladder: the highest-precedence role any of its services' definitions declares, then
    /// the role its OS family implies, then `Unidentified`. Services come first because a matched
    /// definition names the product the host runs; an OS family only narrows the device class.
    pub fn device_role<'a>(&self, services: impl IntoIterator<Item = &'a Service>) -> DeviceRole {
        services
            .into_iter()
            .filter_map(|s| s.base.service_definition.device_role())
            .min()
            .or_else(|| {
                self.base
                    .os
                    .as_ref()
                    .and_then(|os| os.value().0.device_role())
            })
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::hosts::r#impl::attributes::HostOsValue;
    use crate::server::hosts::r#impl::os::{HostOs, HostOsFamily};
    use crate::server::services::definitions::{
        pf_sense::PfSense, postgresql::PostgreSQL, proxmox::Proxmox, samba::Samba, snmp::Snmp,
        true_nas::TrueNAS,
    };
    use crate::server::services::r#impl::base::ServiceBase;
    use crate::server::shared::attribution::{AttributeSource, Attributed};

    fn host_with_os(os: Option<HostOs>) -> Host {
        let mut host = crate::server::shared::types::examples::host();
        host.base.os =
            os.map(|os| Attributed::new(HostOsValue(os), AttributeSource::SshBannerMatch));
        host
    }

    fn service(definition: Box<dyn ServiceDefinition>) -> Service {
        Service {
            base: ServiceBase {
                service_definition: definition,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// The ladder from most specific evidence to none: a service's role beats every lower-ranked
    /// service role and the OS, and `Server` never hides a device role.
    #[test]
    fn services_outrank_each_other_by_precedence_then_the_os() {
        let macos = Some(HostOs::family_only(HostOsFamily::MacOs));

        let firewall_and_nas = [service(Box::new(TrueNAS)), service(Box::new(PfSense))];
        assert_eq!(
            host_with_os(None).device_role(&firewall_and_nas),
            DeviceRole::Firewall
        );

        let hypervisor_with_db = [service(Box::new(PostgreSQL)), service(Box::new(Proxmox))];
        assert_eq!(
            host_with_os(None).device_role(&hypervisor_with_db),
            DeviceRole::Hypervisor
        );

        // A service role wins over the OS role, even when the service's role ranks lower.
        let db = [service(Box::new(PostgreSQL))];
        assert_eq!(
            host_with_os(macos.clone()).device_role(&db),
            DeviceRole::Server
        );

        // Software any host runs implies nothing, so the OS decides.
        let plumbing = [service(Box::new(Snmp)), service(Box::new(Samba))];
        assert_eq!(
            host_with_os(macos.clone()).device_role(&plumbing[..1]),
            DeviceRole::Workstation
        );
        assert_eq!(
            host_with_os(macos).device_role(&plumbing),
            DeviceRole::Server
        );
    }

    #[test]
    fn a_host_with_no_evidence_is_unidentified() {
        let linux = Some(HostOs::family_only(HostOsFamily::Linux));
        let snmp = [service(Box::new(Snmp))];
        assert_eq!(
            host_with_os(linux).device_role(&snmp),
            DeviceRole::Unidentified
        );
        assert_eq!(
            host_with_os(None).device_role(&[]),
            DeviceRole::Unidentified
        );
    }

    #[test]
    fn windows_is_a_server_only_when_its_edition_or_name_says_so() {
        let windows = |name: Option<&str>, edition: Option<&str>| HostOs {
            name: name.map(str::to_string),
            edition: edition.map(str::to_string),
            ..HostOs::family_only(HostOsFamily::Windows)
        };
        assert_eq!(
            windows(Some("Windows Server 2022"), None).device_role(),
            Some(DeviceRole::Server)
        );
        assert_eq!(
            windows(Some("Windows 11"), Some("Pro")).device_role(),
            Some(DeviceRole::Workstation)
        );
    }
}
