//! Deleting a runtime or hypervisor deletes what it runs, and port turnover never deletes one.

use crate::server::auth::middleware::auth::AuthenticatedEntity;
use crate::server::bindings::r#impl::base::Binding;
use crate::server::hosts::r#impl::base::{Host, HostBase};
use crate::server::hosts::r#impl::virtualization::{HostVirtualization, ProxmoxVirtualization};
use crate::server::shared::attribution::AttributeSource;
use crate::server::shared::services::traits::CrudService;
use crate::server::shared::storage::traits::Storable;
use crate::server::shared::types::entities::EntitySource;
use crate::server::shared::types::guest_state::GuestState;
use crate::server::subnets::r#impl::base::{Subnet, SubnetBase, SubnetCidr, SubnetCidrValue};
use crate::server::subnets::r#impl::types::SubnetType;

use super::discovery_stale_port_guards::{LAN_IP, Lab};
use super::discovery_stale_port_holders::covering;
use super::test_services;

/// Deleting the Docker service deletes its containers and the bridge subnet it owns.
#[tokio::test]
async fn deleting_a_runtime_deletes_its_containers_and_bridge_subnets() {
    let (storage, services, _container) = test_services().await;
    let lab = Lab::new(&services, &storage).await;
    let (lan, p3001) = (lab.lan_ip(), lab.tcp(3001));
    let runtime = lab.runtime(&lan);
    let kuma = lab.container_as("UptimeKuma", "kuma", &runtime, vec![(&p3001, &lan)]);
    lab.scan(vec![lan], vec![p3001], vec![runtime, kuma]).await;
    let runtime_id = lab.named("Docker").await.unwrap().id;
    let bridge = services
        .subnet_service
        .create(
            Subnet::new(SubnetBase {
                name: "docker-compose_default".to_string(),
                site_id: lab.site_id,
                cidr: SubnetCidr::new(
                    SubnetCidrValue("172.18.0.0/16".parse().unwrap()),
                    AttributeSource::DaemonSelfReport,
                ),
                subnet_type: SubnetType::DockerBridge,
                source: EntitySource::Discovery,
                virtualization_service_id: Some(runtime_id),
                ..Default::default()
            }),
            AuthenticatedEntity::System,
        )
        .await
        .unwrap();

    services
        .service_service
        .delete(&runtime_id, AuthenticatedEntity::System)
        .await
        .unwrap();

    assert!(lab.named("kuma").await.is_none());
    assert!(
        services
            .subnet_service
            .get_by_id(&bridge.id)
            .await
            .unwrap()
            .is_none()
    );
}

/// Deleting a Proxmox VE node's host deletes the VMs its Proxmox VE service runs.
#[tokio::test]
async fn deleting_a_node_host_deletes_its_guests() {
    let (storage, services, _container) = test_services().await;
    let lab = Lab::new(&services, &storage).await;
    let lan = lab.lan_ip();
    let mut pve = lab.service("Proxmox VE", vec![]);
    pve.base.bindings = vec![Binding::new_ip_address_serviceless(lan.id)];
    lab.scan(vec![lan], vec![], vec![pve]).await;
    let pve_id = lab.named("Proxmox VE").await.unwrap().id;
    let guest = lab
        .submit_host(
            Host::new(HostBase {
                source: EntitySource::Discovery,
                virtualization_metadata: Some(HostVirtualization::Proxmox(ProxmoxVirtualization {
                    vm_name: Some("vm".to_string()),
                    vm_id: Some("100".to_string()),
                    guest_type: None,
                    state: GuestState::Running,
                })),
                virtualization_service_id: Some(pve_id),
                ..Default::default()
            }),
            vec![lab.lan_ip_at("192.168.4.70", "02:00:00:00:00:70")],
        )
        .await;

    services
        .host_service
        .delete_host(&lab.host_id, AuthenticatedEntity::System)
        .await
        .unwrap();

    assert!(!lab.host_exists(&guest).await);
}

/// A runtime whose last port is handed over keeps running its containers: port turnover never
/// deletes a runtime.
#[tokio::test]
async fn an_emptied_runtime_is_not_deleted_by_a_handover() {
    let (storage, services, _container) = test_services().await;
    let lab = Lab::new(&services, &storage).await;
    let (lan, p2375, p3001) = (lab.lan_ip(), lab.tcp(2375), lab.tcp(3001));
    let mut runtime = lab.service("Docker", vec![(&p2375, &lan)]);
    runtime.base.name = "Docker".to_string();
    let kuma = lab.container_as("UptimeKuma", "kuma", &runtime, vec![(&p3001, &lan)]);
    lab.scan(vec![lan], vec![p2375, p3001], vec![runtime, kuma])
        .await;

    let (lan, p2375) = (lab.lan_ip(), lab.tcp(2375));
    let traefik = lab.service("Traefik", vec![(&p2375, &lan)]);
    lab.scan_covering(
        vec![lan],
        vec![p2375],
        vec![traefik],
        covering(LAN_IP, &[2375]),
    )
    .await;

    assert!(
        lab.named("Traefik").await.is_some(),
        "the newcomer took the port"
    );
    assert!(lab.named("Docker").await.is_some());
    assert!(lab.named("kuma").await.is_some());
}
