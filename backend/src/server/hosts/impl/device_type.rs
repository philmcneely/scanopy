use crate::server::hosts::r#impl::base::Host;
use crate::server::services::r#impl::base::Service;
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::{
    ServiceDefinition, ServiceDefinitionExt, VirtualizationRole,
};
use crate::server::shared::entities::EntityDiscriminants;
use crate::server::shared::types::metadata::{EntityMetadataProvider, HasId, TypeMetadataProvider};
use crate::server::shared::types::{Color, Icon};
use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, IntoStaticStr};
use utoipa::ToSchema;

/// What a host is, as a device: the class its icon stands for on the map.
///
/// Declaration order is precedence, highest first. A host whose services claim several device types takes
/// the first one here, so a firewall that also serves files reads as a firewall, and `Server`,
/// which any application implies, never hides a more specific device type.
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
pub enum DeviceType {
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
    /// Also what a device type this build doesn't know reads as, so a newer server cannot break an older
    /// reader.
    #[default]
    #[serde(other)]
    Unidentified,
}

impl HasId for DeviceType {
    fn id(&self) -> &'static str {
        self.into()
    }
}

impl EntityMetadataProvider for DeviceType {
    /// Every device type wears the Host entity's colour: a device is a host first, and the glyph alone
    /// names its device type. Software takes its category's colour instead, so the two never compete.
    fn color(&self) -> Color {
        EntityDiscriminants::Host.color()
    }

    fn icon(&self) -> Icon {
        match self {
            DeviceType::Firewall => Icon::BrickWallFire,
            DeviceType::Router => Icon::Router,
            DeviceType::Switch => Icon::Network,
            DeviceType::AccessPoint => Icon::Wifi,
            DeviceType::Hypervisor => ServiceCategory::Hypervisor.icon(),
            DeviceType::Storage => Icon::HardDrive,
            DeviceType::Printer => Icon::Printer,
            DeviceType::Camera => Icon::Cctv,
            DeviceType::Phone => Icon::Phone,
            DeviceType::Workstation => Icon::Laptop,
            DeviceType::Mobile => Icon::Smartphone,
            DeviceType::Industrial => Icon::Factory,
            DeviceType::IoT => Icon::CircuitBoard,
            DeviceType::Server => Icon::Server,
            // The glyph every other unknown value draws, so a host nothing identifies never reads as
            // a device type. The Host icon is `Server`'s.
            DeviceType::Unidentified => Icon::CircleQuestionMark,
        }
    }
}

impl TypeMetadataProvider for DeviceType {
    fn name(&self) -> &'static str {
        match self {
            DeviceType::Firewall => "Firewall",
            DeviceType::Router => "Router",
            DeviceType::Switch => "Switch",
            DeviceType::AccessPoint => "Access point",
            DeviceType::Hypervisor => "Hypervisor",
            DeviceType::Storage => "Storage",
            DeviceType::Printer => "Printer",
            DeviceType::Camera => "Camera",
            DeviceType::Phone => "Phone",
            DeviceType::Workstation => "Workstation",
            DeviceType::Mobile => "Mobile device",
            DeviceType::Industrial => "Industrial controller",
            DeviceType::IoT => "IoT device",
            DeviceType::Server => "Server",
            DeviceType::Unidentified => "Unidentified",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            DeviceType::Firewall => "Filters traffic between networks",
            DeviceType::Router => "Routes traffic between networks",
            DeviceType::Switch => "Forwards frames between ports on one network",
            DeviceType::AccessPoint => "Bridges wireless clients onto the network",
            DeviceType::Hypervisor => "Runs virtual machines",
            DeviceType::Storage => "Holds files or blocks for other hosts",
            DeviceType::Printer => "Prints or scans",
            DeviceType::Camera => "Streams video",
            DeviceType::Phone => "Carries voice calls",
            DeviceType::Workstation => "A computer a person works at",
            DeviceType::Mobile => "A phone or tablet",
            DeviceType::Industrial => "Controls building or industrial equipment",
            DeviceType::IoT => "A smart-home or embedded device",
            DeviceType::Server => "Runs applications for other hosts",
            DeviceType::Unidentified => "Nothing found on this host names what it is",
        }
    }
}

impl Host {
    /// The device type this host's icon shows.
    ///
    /// The ladder, first answer wins:
    /// 1. The highest-precedence device type the host's own services declare. A matched definition names
    ///    the product the host runs, so it outranks the OS.
    /// 2. The device type its OS family implies.
    /// 3. `Server`, when the host runs containers: a container runtime, or software inside one.
    /// 4. `Unidentified`.
    ///
    /// Containerized software sits below the OS because it says what the container does, not what
    /// the machine is: a laptop running Postgres in Docker is still a laptop. A Linux host gives no
    /// OS device type, so the same containers still make it a server.
    pub fn device_type<'a>(&self, services: impl IntoIterator<Item = &'a Service>) -> DeviceType {
        let (containerized, own): (Vec<&Service>, Vec<&Service>) = services
            .into_iter()
            .partition(|s| s.base.virtualization_metadata.is_some());
        let runs_containers = !containerized.is_empty()
            || own.iter().any(|s| {
                matches!(
                    s.base.service_definition.virtualization_role(),
                    Some(VirtualizationRole::ContainerRuntime { .. })
                )
            });

        own.iter()
            .filter_map(|s| s.base.service_definition.device_type())
            .min()
            .or_else(|| {
                self.base
                    .os
                    .as_ref()
                    .and_then(|os| os.value().0.device_type())
            })
            .or(runs_containers.then_some(DeviceType::Server))
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::hosts::r#impl::attributes::HostOsValue;
    use crate::server::hosts::r#impl::os::{HostOs, HostOsFamily};
    use crate::server::services::definitions::{
        docker_daemon::Docker, pf_sense::PfSense, postgresql::PostgreSQL, proxmox::Proxmox,
        samba::Samba, snmp::Snmp, true_nas::TrueNAS,
    };
    use crate::server::services::r#impl::base::ServiceBase;
    use crate::server::services::r#impl::virtualization::{
        DockerVirtualization, ServiceVirtualization,
    };
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

    fn containerized(definition: Box<dyn ServiceDefinition>) -> Service {
        let mut service = service(definition);
        service.base.virtualization_metadata =
            Some(ServiceVirtualization::Docker(DockerVirtualization {
                state: Default::default(),
                container_name: None,
                container_id: None,
                compose_project: None,
            }));
        service
    }

    /// Software in a container says what the container does, not what the machine is: it ranks
    /// below the OS, and makes a host a server only when nothing else names it.
    #[test]
    fn containers_rank_below_the_os_and_above_nothing() {
        let os = |family| Some(HostOs::family_only(family));
        let db_in_docker = [
            service(Box::new(Docker)),
            containerized(Box::new(PostgreSQL)),
        ];

        // A Mac running Postgres in Docker is a workstation.
        assert_eq!(
            host_with_os(os(HostOsFamily::MacOs)).device_type(&db_in_docker),
            DeviceType::Workstation
        );
        // A Linux host gives no OS device type, so the same containers make it a server, and so does the
        // runtime alone.
        assert_eq!(
            host_with_os(os(HostOsFamily::Linux)).device_type(&db_in_docker),
            DeviceType::Server
        );
        assert_eq!(
            host_with_os(os(HostOsFamily::Linux)).device_type(&db_in_docker[..1]),
            DeviceType::Server
        );
        // Software installed on the host itself still outranks the OS.
        assert_eq!(
            host_with_os(os(HostOsFamily::MacOs)).device_type(&[service(Box::new(PostgreSQL))]),
            DeviceType::Server
        );
        // And a host's own device type outranks its containers.
        let nas_with_containers = [
            service(Box::new(TrueNAS)),
            containerized(Box::new(PostgreSQL)),
        ];
        assert_eq!(
            host_with_os(os(HostOsFamily::Linux)).device_type(&nas_with_containers),
            DeviceType::Storage
        );
    }

    /// The ladder from most specific evidence to none: a service's device type beats every lower-ranked
    /// service device type and the OS, and `Server` never hides a device type.
    #[test]
    fn services_outrank_each_other_by_precedence_then_the_os() {
        let macos = Some(HostOs::family_only(HostOsFamily::MacOs));

        let firewall_and_nas = [service(Box::new(TrueNAS)), service(Box::new(PfSense))];
        assert_eq!(
            host_with_os(None).device_type(&firewall_and_nas),
            DeviceType::Firewall
        );

        let hypervisor_with_db = [service(Box::new(PostgreSQL)), service(Box::new(Proxmox))];
        assert_eq!(
            host_with_os(None).device_type(&hypervisor_with_db),
            DeviceType::Hypervisor
        );

        // A service device type wins over the OS device type, even when the service's device type ranks lower.
        let db = [service(Box::new(PostgreSQL))];
        assert_eq!(
            host_with_os(macos.clone()).device_type(&db),
            DeviceType::Server
        );

        // Software any host runs implies nothing, so the OS decides.
        let plumbing = [service(Box::new(Snmp)), service(Box::new(Samba))];
        assert_eq!(
            host_with_os(macos.clone()).device_type(&plumbing[..1]),
            DeviceType::Workstation
        );
        assert_eq!(
            host_with_os(macos).device_type(&plumbing),
            DeviceType::Server
        );
    }

    #[test]
    fn a_host_with_no_evidence_is_unidentified() {
        let linux = Some(HostOs::family_only(HostOsFamily::Linux));
        let snmp = [service(Box::new(Snmp))];
        assert_eq!(
            host_with_os(linux).device_type(&snmp),
            DeviceType::Unidentified
        );
        assert_eq!(
            host_with_os(None).device_type(&[]),
            DeviceType::Unidentified
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
            windows(Some("Windows Server 2022"), None).device_type(),
            Some(DeviceType::Server)
        );
        assert_eq!(
            windows(Some("Windows 11"), Some("Pro")).device_type(),
            Some(DeviceType::Workstation)
        );
    }
}
