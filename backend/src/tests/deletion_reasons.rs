//! Every delete says why, and the Deleted event carries it.

use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use uuid::Uuid;

use crate::server::auth::middleware::auth::AuthenticatedEntity;
use crate::server::bindings::r#impl::base::Binding;
use crate::server::credentials::r#impl::types::CredentialType;
use crate::server::services::definitions::ServiceDefinitionRegistry;
use crate::server::shared::entities::EntityDiscriminants;
use crate::server::shared::events::traits::{EntityEventFilter, Event, Subscriber};
use crate::server::shared::events::types::{DeletionReason, EntityOperation};
use crate::server::shared::services::factory::ServiceFactory;
use crate::server::shared::services::traits::CrudService;

use super::discovery_stale_port_guards::{LAN_IP, Lab};
use super::discovery_stale_port_holders::covering;
use super::test_services;

/// Every Deleted event published: what went, and why.
#[derive(Default)]
struct Deletions(Mutex<Vec<(EntityDiscriminants, Uuid, DeletionReason)>>);

#[async_trait]
impl Subscriber<EntityOperation> for Deletions {
    fn filter(&self) -> EntityEventFilter {
        EntityEventFilter::all()
    }

    async fn handle(&self, events: Vec<Event<EntityOperation>>) -> anyhow::Result<()> {
        let mut deletions = self.0.lock().unwrap();
        for event in events {
            if let EntityOperation::Deleted { reason } = event.operation {
                deletions.push((
                    event.scope.entity_discriminant(),
                    event.scope.entity_id(),
                    reason,
                ));
            }
        }
        Ok(())
    }
}

async fn listen(services: &ServiceFactory) -> Arc<Deletions> {
    let deletions = Arc::new(Deletions::default());
    services
        .event_bus
        .register::<EntityOperation>(deletions.clone(), "test_deletions")
        .await;
    deletions
}

/// The reason the entity `id` was deleted for, once every subscriber has run.
async fn reason_for(services: &ServiceFactory, deletions: &Deletions, id: Uuid) -> DeletionReason {
    services.event_bus.flush().await;
    deletions
        .0
        .lock()
        .unwrap()
        .iter()
        .find(|(_, deleted, _)| *deleted == id)
        .map(|(_, _, reason)| reason.clone())
        .unwrap_or_else(|| panic!("{id} was not deleted"))
}

fn definition(
    id: &str,
) -> Box<dyn crate::server::services::r#impl::definitions::ServiceDefinition> {
    ServiceDefinitionRegistry::find_by_id(id).unwrap()
}

/// Caddy takes Traefik's only port: the binding and then Traefik go, both for Caddy.
#[tokio::test]
async fn a_handover_names_who_took_the_port() {
    let (storage, services, _container) = test_services().await;
    let lab = Lab::new(&services, &storage).await;
    let (lan, p443) = (lab.lan_ip(), lab.tcp(443));
    let traefik = lab.service("Traefik", vec![(&p443, &lan)]);
    lab.scan(vec![lan], vec![p443], vec![traefik]).await;
    let traefik = lab.named("Traefik").await.unwrap();
    let deletions = listen(&services).await;

    let (lan, p443) = (lab.lan_ip(), lab.tcp(443));
    let caddy = lab.service("Caddy", vec![(&p443, &lan)]);
    lab.scan_covering(vec![lan], vec![p443], vec![caddy], covering(LAN_IP, &[443]))
        .await;
    let caddy = lab.named("Caddy").await.unwrap();

    let handed_over = DeletionReason::ServicePortsHandedOver {
        to_service_id: caddy.id,
        to: definition("Caddy"),
    };
    assert_eq!(
        reason_for(&services, &deletions, traefik.id).await,
        handed_over
    );
    assert_eq!(
        reason_for(&services, &deletions, traefik.base.bindings[0].id()).await,
        handed_over
    );
}

/// Nothing answers on Traefik's only port past the stale window, so it is released.
#[tokio::test]
async fn a_stale_release_says_nothing_answered() {
    let (storage, services, _container) = test_services().await;
    let lab = Lab::new(&services, &storage).await;
    let (lan, p443) = (lab.lan_ip(), lab.tcp(443));
    let traefik = lab.service("Traefik", vec![(&p443, &lan)]);
    lab.scan(vec![lan], vec![p443], vec![traefik]).await;
    let traefik = lab.named("Traefik").await.unwrap();
    lab.age_port(443).await;
    let deletions = listen(&services).await;

    lab.scan_covering(vec![lab.lan_ip()], vec![], vec![], covering(LAN_IP, &[443]))
        .await;

    assert_eq!(
        reason_for(&services, &deletions, traefik.id).await,
        DeletionReason::ServicePortsReleased
    );
}

/// Caddy claims the only port Unclaimed Open Ports held.
#[tokio::test]
async fn an_emptied_open_ports_names_its_claimant() {
    let (storage, services, _container) = test_services().await;
    let lab = Lab::new(&services, &storage).await;
    let (lan, p443) = (lab.lan_ip(), lab.tcp(443));
    let open_ports = lab.service("Unclaimed Open Ports", vec![(&p443, &lan)]);
    lab.scan(vec![lan], vec![p443], vec![open_ports]).await;
    let open_ports = lab.named("Unclaimed Open Ports").await.unwrap();
    let deletions = listen(&services).await;

    let (lan, p443) = (lab.lan_ip(), lab.tcp(443));
    let caddy = lab.service("Caddy", vec![(&p443, &lan)]);
    lab.scan(vec![lan], vec![p443], vec![caddy]).await;
    let caddy = lab.named("Caddy").await.unwrap();

    assert_eq!(
        reason_for(&services, &deletions, open_ports.id).await,
        DeletionReason::ServiceOpenPortsReclaimed {
            to_service_id: caddy.id,
            by: definition("Caddy"),
        }
    );
}

/// A generic container placeholder goes as the service it was identified as.
#[tokio::test]
async fn an_absorbed_placeholder_names_its_identification() {
    let (storage, services, _container) = test_services().await;
    let lab = Lab::new(&services, &storage).await;
    let (lan, p3001) = (lab.lan_ip(), lab.tcp(3001));
    let runtime = lab.runtime(&lan);
    let placeholder = lab.container_as("Docker Container", "kuma", &runtime, vec![(&p3001, &lan)]);
    lab.scan(vec![lan], vec![p3001], vec![runtime, placeholder])
        .await;
    let placeholder = lab.named("kuma").await.unwrap();
    let deletions = listen(&services).await;

    let lan = lab.lan_ip();
    let runtime = lab.runtime(&lan);
    let mut kuma = lab.container_as("UptimeKuma", "kuma", &runtime, vec![]);
    kuma.base.name = "Uptime Kuma".to_string();
    lab.scan(vec![lan], vec![], vec![runtime, kuma]).await;
    let kuma = lab.named("Uptime Kuma").await.unwrap();

    assert_eq!(
        reason_for(&services, &deletions, placeholder.id).await,
        DeletionReason::ServiceIdentifiedAs {
            service_id: kuma.id
        }
    );
}

/// A container its runtime listed in full and no longer has.
#[tokio::test]
async fn a_removed_container_names_its_runtime() {
    let (storage, services, _container) = test_services().await;
    let lab = Lab::new(&services, &storage).await;
    let docker = lab
        .credential(CredentialType::DockerSocket { socket_path: None })
        .await;
    let (lan, p3001) = (lab.lan_ip(), lab.tcp(3001));
    let runtime = lab.runtime(&lan);
    let kuma = lab.container_as("UptimeKuma", "kuma", &runtime, vec![(&p3001, &lan)]);
    lab.scan(vec![lan], vec![p3001], vec![runtime, kuma]).await;
    let (runtime, kuma) = (
        lab.named("Docker").await.unwrap(),
        lab.named("kuma").await.unwrap(),
    );
    let deletions = listen(&services).await;

    let scanned = lab.reported(&[lab.host_id], &["Docker"]).await;
    lab.finish_run(docker, &[lab.host_id], &scanned).await;

    assert_eq!(
        reason_for(&services, &deletions, kuma.id).await,
        DeletionReason::EntityGuestNoLongerListed {
            parent_service_id: runtime.id
        }
    );
}

/// Deleting a runtime deletes its container for it.
#[tokio::test]
async fn a_cascaded_container_names_its_runtime() {
    let (storage, services, _container) = test_services().await;
    let lab = Lab::new(&services, &storage).await;
    let (lan, p3001) = (lab.lan_ip(), lab.tcp(3001));
    let runtime = lab.runtime(&lan);
    let kuma = lab.container_as("UptimeKuma", "kuma", &runtime, vec![(&p3001, &lan)]);
    lab.scan(vec![lan], vec![p3001], vec![runtime, kuma]).await;
    let (runtime, kuma) = (
        lab.named("Docker").await.unwrap(),
        lab.named("kuma").await.unwrap(),
    );
    let deletions = listen(&services).await;

    services
        .service_service
        .delete(
            &runtime.id,
            AuthenticatedEntity::System,
            DeletionReason::EntityRequested,
        )
        .await
        .unwrap();

    assert_eq!(
        reason_for(&services, &deletions, runtime.id).await,
        DeletionReason::EntityRequested
    );
    assert_eq!(
        reason_for(&services, &deletions, kuma.id).await,
        DeletionReason::EntityVirtualizerDeleted {
            virtualizer_service_id: runtime.id
        }
    );
}

/// Deleting a host deletes its services for it.
#[tokio::test]
async fn a_hosts_services_name_the_host() {
    let (storage, services, _container) = test_services().await;
    let lab = Lab::new(&services, &storage).await;
    let lan = lab.lan_ip();
    let mut pve = lab.service("Proxmox VE", vec![]);
    pve.base.bindings = vec![Binding::new_ip_address_serviceless(lan.id)];
    lab.scan(vec![lan], vec![], vec![pve]).await;
    let pve = lab.named("Proxmox VE").await.unwrap();
    let deletions = listen(&services).await;

    services
        .host_service
        .delete_host(
            &lab.host_id,
            AuthenticatedEntity::System,
            DeletionReason::EntityRequested,
        )
        .await
        .unwrap();

    assert_eq!(
        reason_for(&services, &deletions, pve.id).await,
        DeletionReason::ServiceHostDeleted {
            host_id: lab.host_id
        }
    );
}
