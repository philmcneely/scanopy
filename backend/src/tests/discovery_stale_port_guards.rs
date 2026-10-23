//! What a rescan must leave alone once stale port holders can lose their ports.
//!
//! Each test replays scans of one host through `discover_host` on a testcontainers Postgres. The
//! scans here carry no port coverage, as an older daemon's do, so every outcome below held before
//! ports could change hands and must still hold after: a port moves only on a scan that covered it.

use std::net::IpAddr;

use chrono::{DateTime, Utc};
use mac_address::MacAddress;
use uuid::Uuid;

use crate::server::auth::middleware::auth::AuthenticatedEntity;
use crate::server::bindings::r#impl::base::Binding;
use crate::server::dependencies::r#impl::base::DependencyMembers;
use crate::server::hosts::r#impl::base::{Host, HostBase};
use crate::server::interfaces::r#impl::base::InterfaceDataComplete;
use crate::server::ip_addresses::r#impl::base::{
    IPAddress, IPAddressBase, MacEvidence, MacEvidenceValue,
};
use crate::server::ports::r#impl::base::{Port, PortBase, PortType};
use crate::server::ports::r#impl::coverage::PortCoverage;
use crate::server::services::definitions::ServiceDefinitionRegistry;
use crate::server::services::r#impl::base::{Service, ServiceBase};
use crate::server::services::r#impl::virtualization::{
    DockerVirtualization, ServiceVirtualization,
};
use crate::server::shared::attribution::AttributeSource;
use crate::server::shared::services::factory::ServiceFactory;
use crate::server::shared::services::traits::CrudService;
use crate::server::shared::storage::filter::StorableFilter;
use crate::server::shared::storage::traits::{Storable, Storage};
use crate::server::shared::types::entities::EntitySource;
use crate::server::sites::r#impl::{Site, SiteBase};
use crate::server::subnets::r#impl::base::{Subnet, SubnetBase, SubnetCidr, SubnetCidrValue};
use crate::server::subnets::r#impl::types::SubnetType;

use super::{dependency, organization, test_services};
use crate::server::subnets::r#impl::base::{SubnetName, SubnetTypeAttributed};

pub(super) const LAN_IP: &str = "192.168.4.58";
pub(super) const SECOND_LAN_IP: &str = "192.168.4.59";

/// One host on a LAN with a Docker bridge, scanned repeatedly under the same host id.
pub(super) struct Lab<'a> {
    pub(super) services: &'a ServiceFactory,
    pub(super) site_id: Uuid,
    lan: Uuid,
    bridge: Uuid,
    pub(super) host_id: Uuid,
}

impl<'a> Lab<'a> {
    pub(super) async fn new(
        services: &'a ServiceFactory,
        storage: &crate::server::shared::storage::factory::StorageFactory,
    ) -> Self {
        let org = organization();
        storage.organizations.create(&org).await.unwrap();
        let site_id = services
            .site_service
            .create(
                Site::new(SiteBase::new(org.id)),
                AuthenticatedEntity::System,
            )
            .await
            .unwrap()
            .id;
        let lan = Self::subnet(services, site_id, "192.168.4.0/22", SubnetType::Lan).await;
        let bridge =
            Self::subnet(services, site_id, "172.17.0.0/16", SubnetType::DockerBridge).await;
        Self {
            services,
            site_id,
            lan,
            bridge,
            host_id: Uuid::new_v4(),
        }
    }

    pub(super) async fn subnet(
        services: &ServiceFactory,
        site_id: Uuid,
        cidr: &str,
        subnet_type: SubnetType,
    ) -> Uuid {
        services
            .subnet_service
            .create(
                Subnet::new(SubnetBase {
                    name: SubnetName::unattributed(cidr.to_string()),
                    site_id,
                    cidr: SubnetCidr::new(
                        SubnetCidrValue(cidr.parse().unwrap()),
                        AttributeSource::DaemonSelfReport,
                    ),
                    subnet_type: SubnetTypeAttributed::unattributed(subnet_type),
                    source: EntitySource::Discovery,
                    ..Default::default()
                }),
                AuthenticatedEntity::System,
            )
            .await
            .unwrap()
            .id
    }

    pub(super) fn lan_ip_at(&self, ip: &str, mac: &str) -> IPAddress {
        let mac: MacAddress = mac.parse().unwrap();
        IPAddress::new(IPAddressBase {
            site_id: self.site_id,
            subnet_id: self.lan,
            ip_address: ip.parse().unwrap(),
            mac_address: Some(MacEvidence::new(
                MacEvidenceValue(mac),
                AttributeSource::ArpReply,
            )),
            ..Default::default()
        })
    }

    pub(super) fn lan_ip(&self) -> IPAddress {
        self.lan_ip_at(LAN_IP, "02:1c:65:32:e5:dd")
    }

    pub(super) fn second_lan_ip(&self) -> IPAddress {
        self.lan_ip_at(SECOND_LAN_IP, "02:1c:65:32:e5:de")
    }

    pub(super) fn bridge_ip(&self, ip: &str) -> IPAddress {
        IPAddress::new(IPAddressBase {
            site_id: self.site_id,
            subnet_id: self.bridge,
            ip_address: ip.parse().unwrap(),
            ..Default::default()
        })
    }

    pub(super) fn tcp(&self, number: u16) -> Port {
        Port::new(PortBase::new(
            Uuid::nil(),
            self.site_id,
            PortType::new_tcp(number),
        ))
    }

    pub(super) fn service(&self, definition: &str, bindings: Vec<(&Port, &IPAddress)>) -> Service {
        let mut service = Service::new(ServiceBase {
            name: definition.to_string(),
            site_id: self.site_id,
            service_definition: ServiceDefinitionRegistry::find_by_id(definition)
                .unwrap_or_else(|| panic!("{definition} is registered")),
            source: EntitySource::Discovery,
            ..Default::default()
        });
        service.base.bindings = bindings
            .into_iter()
            .map(|(port, ip)| Binding::new_port_serviceless(port.id, Some(ip.id)))
            .collect();
        service
    }

    /// The Docker runtime, bound to the host's LAN address.
    pub(super) fn runtime(&self, lan: &IPAddress) -> Service {
        let mut runtime = self.service("Docker", vec![]);
        runtime.base.bindings = vec![Binding::new_ip_address_serviceless(lan.id)];
        runtime
    }

    /// A container the daemon identified as `definition`, named `name`.
    pub(super) fn container_as(
        &self,
        definition: &str,
        name: &str,
        runtime: &Service,
        bindings: Vec<(&Port, &IPAddress)>,
    ) -> Service {
        let mut container = self.service(definition, bindings);
        container.base.name = name.to_string();
        container.base.virtualization_service_id = Some(runtime.id);
        container.base.virtualization_metadata =
            Some(ServiceVirtualization::Docker(DockerVirtualization {
                state: Default::default(),
                container_name: Some(name.to_string()),
                container_id: Some(format!("{name}-id")),
                compose_project: None,
            }));
        container
    }

    pub(super) async fn scan(
        &self,
        ip_addresses: Vec<IPAddress>,
        ports: Vec<Port>,
        services: Vec<Service>,
    ) {
        let mut host = Host::new(HostBase {
            site_id: self.site_id,
            source: EntitySource::Discovery,
            ..Default::default()
        });
        host.id = self.host_id;
        self.services
            .host_service
            .discover_host(
                host,
                ip_addresses,
                ports,
                services,
                vec![],
                vec![],
                false,
                InterfaceDataComplete::none(),
                &PortCoverage::default(),
                None,
                AuthenticatedEntity::System,
                None,
            )
            .await
            .expect("the scan persists");
    }

    pub(super) async fn stored(&self) -> Vec<Service> {
        self.services
            .service_service
            .get_all(StorableFilter::<Service>::new_from_site_ids(&[self.site_id]).live())
            .await
            .unwrap()
    }

    /// The live service with this name, if any.
    pub(super) async fn named(&self, name: &str) -> Option<Service> {
        self.stored()
            .await
            .into_iter()
            .find(|s| s.base.name == name)
    }

    pub(super) async fn stored_ports(&self) -> Vec<Port> {
        self.services
            .port_service
            .get_for_host(&self.host_id)
            .await
            .unwrap()
    }

    pub(super) async fn stored_ip(&self, ip: &str) -> IPAddress {
        let ip: IpAddr = ip.parse().unwrap();
        self.services
            .ip_address_service
            .get_for_host(&self.host_id)
            .await
            .unwrap()
            .into_iter()
            .find(|i| i.base.ip_address == ip)
            .expect("address stored")
    }

    pub(super) async fn stored_tcp(&self, number: u16) -> Port {
        self.stored_ports()
            .await
            .into_iter()
            .find(|p| p.base.port_type == PortType::new_tcp(number))
            .expect("port stored")
    }

    /// The (port, address) pairs a service binds, sorted. An all-addresses binding has no address.
    pub(super) async fn ports_of(&self, service: &Service) -> Vec<(u16, Option<IpAddr>)> {
        let ports = self.stored_ports().await;
        let ips = self
            .services
            .ip_address_service
            .get_for_host(&self.host_id)
            .await
            .unwrap();
        let mut pairs: Vec<(u16, Option<IpAddr>)> = service
            .base
            .bindings
            .iter()
            .filter_map(|b| {
                let port = ports.iter().find(|p| Some(p.id) == b.port_id())?;
                let ip = b
                    .ip_address_id()
                    .and_then(|id| ips.iter().find(|i| i.id == id))
                    .map(|i| i.base.ip_address);
                Some((port.base.port_type.number(), ip))
            })
            .collect();
        pairs.sort();
        pairs
    }

    /// The ports the service with this name binds, or `None` when it is not stored.
    pub(super) async fn ports_named(&self, name: &str) -> Option<Vec<(u16, Option<IpAddr>)>> {
        let service = self.named(name).await?;
        Some(self.ports_of(&service).await)
    }

    /// Store a service the way the API does, with `source: Manual`, bound to stored rows.
    pub(super) async fn create_manual(&self, definition: &str, port: u16, ip: &str) -> Service {
        let port = self.stored_tcp(port).await;
        let ip = self.stored_ip(ip).await;
        let mut service = self.service(definition, vec![]);
        service.base.host_id = self.host_id;
        service.base.source = EntitySource::Manual;
        service.base.bindings = vec![Binding::new_port_serviceless(port.id, Some(ip.id))];
        self.services
            .service_service
            .create(service, AuthenticatedEntity::System)
            .await
            .expect("manual service stored")
    }
}

pub(super) fn at_lan(number: u16) -> (u16, Option<IpAddr>) {
    (number, Some(LAN_IP.parse().unwrap()))
}

pub(super) fn at(number: u16, ip: &str) -> (u16, Option<IpAddr>) {
    (number, Some(ip.parse().unwrap()))
}

/// R1. Two services answering on one port in the same scan: the stored one keeps it.
#[tokio::test]
async fn a_rediscovered_holder_keeps_its_port_when_another_service_also_reports_it() {
    let (storage, services, _container) = test_services().await;
    let lab = Lab::new(&services, &storage).await;
    let (lan, p443) = (lab.lan_ip(), lab.tcp(443));
    let traefik = lab.service("Traefik", vec![(&p443, &lan)]);
    lab.scan(vec![lan], vec![p443], vec![traefik]).await;

    let (lan, p443) = (lab.lan_ip(), lab.tcp(443));
    let traefik = lab.service("Traefik", vec![(&p443, &lan)]);
    let caddy = lab.service("Caddy", vec![(&p443, &lan)]);
    lab.scan(vec![lan], vec![p443], vec![traefik, caddy]).await;

    assert_eq!(lab.ports_named("Traefik").await, Some(vec![at_lan(443)]));
    assert_eq!(lab.named("Caddy").await, None);
}

/// A scan with no coverage that reports a different identified service on a stored port leaves
/// the port with the stored service and drops the newcomer.
#[tokio::test]
async fn without_coverage_a_replacement_on_a_stored_port_is_dropped() {
    let (storage, services, _container) = test_services().await;
    let lab = Lab::new(&services, &storage).await;
    let (lan, p443) = (lab.lan_ip(), lab.tcp(443));
    let traefik = lab.service("Traefik", vec![(&p443, &lan)]);
    lab.scan(vec![lan], vec![p443], vec![traefik]).await;

    let (lan, p443) = (lab.lan_ip(), lab.tcp(443));
    let caddy = lab.service("Caddy", vec![(&p443, &lan)]);
    lab.scan(vec![lan], vec![p443], vec![caddy]).await;

    assert_eq!(lab.ports_named("Traefik").await, Some(vec![at_lan(443)]));
    assert_eq!(lab.named("Caddy").await, None);
}

/// R3. A generic port-only match never displaces an identified stored service.
#[tokio::test]
async fn a_generic_match_on_an_identified_services_port_is_dropped() {
    let (storage, services, _container) = test_services().await;
    let lab = Lab::new(&services, &storage).await;
    let (lan, p5432) = (lab.lan_ip(), lab.tcp(5432));
    let grafana = lab.service("Grafana", vec![(&p5432, &lan)]);
    lab.scan(vec![lan], vec![p5432], vec![grafana]).await;

    let (lan, p5432) = (lab.lan_ip(), lab.tcp(5432));
    let postgres = lab.service("PostgreSQL", vec![(&p5432, &lan)]);
    lab.scan(vec![lan], vec![p5432], vec![postgres]).await;

    assert_eq!(lab.ports_named("Grafana").await, Some(vec![at_lan(5432)]));
    assert_eq!(lab.named("PostgreSQL").await, None);
}

/// R7. A manually created service keeps its port when a scan reports something else on it.
#[tokio::test]
async fn a_manual_service_keeps_its_port_when_another_service_reports_it() {
    let (storage, services, _container) = test_services().await;
    let lab = Lab::new(&services, &storage).await;
    let (lan, p443) = (lab.lan_ip(), lab.tcp(443));
    lab.scan(vec![lan], vec![p443], vec![]).await;
    lab.create_manual("Traefik", 443, LAN_IP).await;

    let (lan, p443) = (lab.lan_ip(), lab.tcp(443));
    let caddy = lab.service("Caddy", vec![(&p443, &lan)]);
    lab.scan(vec![lan], vec![p443], vec![caddy]).await;

    let traefik = lab.named("Traefik").await.expect("manual service stored");
    assert_eq!(traefik.base.source, EntitySource::Manual);
    assert_eq!(lab.ports_of(&traefik).await, vec![at_lan(443)]);
    assert_eq!(lab.named("Caddy").await, None);
}

/// R8. A service bound to an address and no port is untouched by a scan that reports other
/// services on that address.
#[tokio::test]
async fn an_address_only_service_survives_a_rescan_that_omits_it() {
    let (storage, services, _container) = test_services().await;
    let lab = Lab::new(&services, &storage).await;
    let lan = lab.lan_ip();
    let runtime = lab.runtime(&lan);
    lab.scan(vec![lan], vec![], vec![runtime]).await;

    let (lan, p3000) = (lab.lan_ip(), lab.tcp(3000));
    let grafana = lab.service("Grafana", vec![(&p3000, &lan)]);
    lab.scan(vec![lan], vec![p3000], vec![grafana]).await;

    let runtime = lab.named("Docker").await.expect("runtime stored");
    assert_eq!(runtime.base.bindings.len(), 1);
    assert_eq!(
        runtime.base.bindings[0].ip_address_id(),
        Some(lab.stored_ip(LAN_IP).await.id)
    );
}

/// R12. Rescanning an unchanged host re-matches every service, keeps every binding, deletes
/// nothing and advances each service's `last_seen_at`.
#[tokio::test]
async fn an_unchanged_rescan_keeps_every_service_and_binding() {
    let (storage, services, _container) = test_services().await;
    let lab = Lab::new(&services, &storage).await;
    let (lan, p22, p443, p3000) = (lab.lan_ip(), lab.tcp(22), lab.tcp(443), lab.tcp(3000));
    let traefik = lab.service("Traefik", vec![(&p443, &lan)]);
    let grafana = lab.service("Grafana", vec![(&p3000, &lan)]);
    let open_ports = lab.service("Unclaimed Open Ports", vec![(&p22, &lan)]);
    lab.scan(
        vec![lan],
        vec![p22, p443, p3000],
        vec![traefik, grafana, open_ports],
    )
    .await;
    let before: Vec<(Uuid, DateTime<Utc>)> = {
        let mut rows: Vec<_> = lab
            .stored()
            .await
            .into_iter()
            .map(|s| (s.id, s.last_seen_at))
            .collect();
        rows.sort();
        rows
    };

    let (lan, p22, p443, p3000) = (lab.lan_ip(), lab.tcp(22), lab.tcp(443), lab.tcp(3000));
    let traefik = lab.service("Traefik", vec![(&p443, &lan)]);
    let grafana = lab.service("Grafana", vec![(&p3000, &lan)]);
    let open_ports = lab.service("Unclaimed Open Ports", vec![(&p22, &lan)]);
    lab.scan(
        vec![lan],
        vec![p22, p443, p3000],
        vec![traefik, grafana, open_ports],
    )
    .await;

    let mut after: Vec<(Uuid, DateTime<Utc>)> = lab
        .stored()
        .await
        .into_iter()
        .map(|s| (s.id, s.last_seen_at))
        .collect();
    after.sort();
    assert_eq!(
        after.iter().map(|(id, _)| *id).collect::<Vec<_>>(),
        before.iter().map(|(id, _)| *id).collect::<Vec<_>>()
    );
    for ((_, seen_before), (_, seen_after)) in before.iter().zip(after.iter()) {
        assert!(seen_after > seen_before);
    }
    assert_eq!(lab.ports_named("Traefik").await, Some(vec![at_lan(443)]));
    assert_eq!(lab.ports_named("Grafana").await, Some(vec![at_lan(3000)]));
    assert_eq!(
        lab.ports_named("Unclaimed Open Ports").await,
        Some(vec![at_lan(22)])
    );
}

/// The freshness signal the release rule reads: a port the scan reports again advances its
/// `last_seen_at`, and a port it leaves out keeps the old value.
#[tokio::test]
async fn only_a_reported_port_advances_its_last_seen_at() {
    let (storage, services, _container) = test_services().await;
    let lab = Lab::new(&services, &storage).await;
    let (lan, p22, p443) = (lab.lan_ip(), lab.tcp(22), lab.tcp(443));
    let traefik = lab.service("Traefik", vec![(&p443, &lan)]);
    let open_ports = lab.service("Unclaimed Open Ports", vec![(&p22, &lan)]);
    lab.scan(vec![lan], vec![p22, p443], vec![traefik, open_ports])
        .await;
    let p22_before = lab.stored_tcp(22).await.last_seen_at;
    let p443_before = lab.stored_tcp(443).await.last_seen_at;

    let (lan, p443) = (lab.lan_ip(), lab.tcp(443));
    let traefik = lab.service("Traefik", vec![(&p443, &lan)]);
    lab.scan(vec![lan], vec![p443], vec![traefik]).await;

    assert_eq!(lab.stored_tcp(22).await.last_seen_at, p22_before);
    assert!(lab.stored_tcp(443).await.last_seen_at > p443_before);
}

/// R13. A network scan of the runtime's LAN address leaves a container's bridge and published
/// bindings alone.
#[tokio::test]
async fn a_lan_scan_leaves_a_containers_bindings_alone() {
    let (storage, services, _container) = test_services().await;
    let lab = Lab::new(&services, &storage).await;
    let (lan, bridge, p3000) = (lab.lan_ip(), lab.bridge_ip("172.17.0.4"), lab.tcp(3000));
    let runtime = lab.runtime(&lan);
    let grafana = lab.container_as(
        "Grafana",
        "grafana",
        &runtime,
        vec![(&p3000, &lan), (&p3000, &bridge)],
    );
    lab.scan(vec![lan, bridge], vec![p3000], vec![runtime, grafana])
        .await;

    let (lan, p22) = (lab.lan_ip(), lab.tcp(22));
    let open_ports = lab.service("Unclaimed Open Ports", vec![(&p22, &lan)]);
    lab.scan(vec![lan], vec![p22], vec![open_ports]).await;

    assert_eq!(
        lab.ports_named("grafana").await,
        Some(vec![at(3000, "172.17.0.4"), at_lan(3000)])
    );
}

/// R14. A service bound to a port on every address keeps it when a newcomer claims that port at
/// one of the host's two addresses.
#[tokio::test]
async fn an_all_addresses_binding_survives_a_claim_at_one_address() {
    let (storage, services, _container) = test_services().await;
    let lab = Lab::new(&services, &storage).await;
    let (lan, second, p443) = (lab.lan_ip(), lab.second_lan_ip(), lab.tcp(443));
    let mut traefik = lab.service("Traefik", vec![]);
    traefik.base.bindings = vec![Binding::new_port_serviceless(p443.id, None)];
    lab.scan(vec![lan, second], vec![p443], vec![traefik]).await;

    let (lan, second, p443) = (lab.lan_ip(), lab.second_lan_ip(), lab.tcp(443));
    let caddy = lab.service("Caddy", vec![(&p443, &second)]);
    lab.scan(vec![lan, second], vec![p443], vec![caddy]).await;

    assert_eq!(lab.ports_named("Traefik").await, Some(vec![(443, None)]));
    assert_eq!(lab.named("Caddy").await, None);
}

/// R15. A dependency keeps its members across a rescan that moves nothing.
#[tokio::test]
async fn a_dependency_keeps_its_members_when_nothing_moves() {
    let (storage, services, _container) = test_services().await;
    let lab = Lab::new(&services, &storage).await;
    let (lan, p443, p3000) = (lab.lan_ip(), lab.tcp(443), lab.tcp(3000));
    let traefik = lab.service("Traefik", vec![(&p443, &lan)]);
    let grafana = lab.service("Grafana", vec![(&p3000, &lan)]);
    lab.scan(vec![lan], vec![p443, p3000], vec![traefik, grafana])
        .await;
    let traefik = lab.named("Traefik").await.unwrap();
    let grafana = lab.named("Grafana").await.unwrap();
    let mut chain = dependency(&lab.site_id);
    chain.base.members = DependencyMembers::Services {
        service_ids: vec![traefik.id, grafana.id],
    };
    let chain = services
        .dependency_service
        .create(chain, AuthenticatedEntity::System)
        .await
        .unwrap();

    let (lan, p443) = (lab.lan_ip(), lab.tcp(443));
    let caddy = lab.service("Caddy", vec![(&p443, &lan)]);
    lab.scan(vec![lan], vec![p443], vec![caddy]).await;

    let chain = services
        .dependency_service
        .get_by_id(&chain.id)
        .await
        .unwrap()
        .expect("dependency stored");
    assert_eq!(
        chain.base.members,
        DependencyMembers::Services {
            service_ids: vec![traefik.id, grafana.id],
        }
    );
}

/// R16. Unclaimed Open Ports keeps a port a later scan doesn't report.
#[tokio::test]
async fn open_ports_keeps_a_port_a_later_scan_leaves_out() {
    let (storage, services, _container) = test_services().await;
    let lab = Lab::new(&services, &storage).await;
    let (lan, p22, p8080) = (lab.lan_ip(), lab.tcp(22), lab.tcp(8080));
    let open_ports = lab.service("Unclaimed Open Ports", vec![(&p22, &lan), (&p8080, &lan)]);
    lab.scan(vec![lan], vec![p22, p8080], vec![open_ports])
        .await;

    let (lan, p22) = (lab.lan_ip(), lab.tcp(22));
    let open_ports = lab.service("Unclaimed Open Ports", vec![(&p22, &lan)]);
    lab.scan(vec![lan], vec![p22], vec![open_ports]).await;

    assert_eq!(
        lab.ports_named("Unclaimed Open Ports").await,
        Some(vec![at_lan(22), at_lan(8080)])
    );
}

/// R6. An identified container on a port another container holds is dropped, and the holder
/// keeps the port.
#[tokio::test]
async fn without_coverage_a_new_container_on_a_held_port_is_dropped() {
    let (storage, services, _container) = test_services().await;
    let lab = Lab::new(&services, &storage).await;
    let (lan, p3001) = (lab.lan_ip(), lab.tcp(3001));
    let runtime = lab.runtime(&lan);
    let kuma = lab.container_as("UptimeKuma", "uptime-kuma", &runtime, vec![(&p3001, &lan)]);
    lab.scan(vec![lan], vec![p3001], vec![runtime, kuma]).await;

    let (lan, p3001) = (lab.lan_ip(), lab.tcp(3001));
    let runtime = lab.runtime(&lan);
    let gitea = lab.container_as("Gitea", "gitea", &runtime, vec![(&p3001, &lan)]);
    lab.scan(vec![lan], vec![p3001], vec![runtime, gitea]).await;

    assert_eq!(
        lab.ports_named("uptime-kuma").await,
        Some(vec![at_lan(3001)])
    );
    assert_eq!(lab.named("gitea").await, None);
}

/// R18. Two new services claiming one port in one scan: the first keeps it.
#[tokio::test]
async fn the_first_of_two_new_claimants_in_one_scan_keeps_the_port() {
    let (storage, services, _container) = test_services().await;
    let lab = Lab::new(&services, &storage).await;
    let (lan, p443) = (lab.lan_ip(), lab.tcp(443));
    let traefik = lab.service("Traefik", vec![(&p443, &lan)]);
    let caddy = lab.service("Caddy", vec![(&p443, &lan)]);
    lab.scan(vec![lan], vec![p443], vec![traefik, caddy]).await;

    assert_eq!(lab.ports_named("Traefik").await, Some(vec![at_lan(443)]));
    assert_eq!(lab.named("Caddy").await, None);
}
