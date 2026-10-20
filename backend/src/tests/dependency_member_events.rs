//! A dependency member outlives what it names, and the change is announced once.
//!
//! | What happened | Bindings member | Services member |
//! |---|---|---|
//! | binding moved to another service | `MemberMoved` | `MemberMoved`, if the move emptied it |
//! | binding released | `MemberReleased` | nothing (deleted if emptied) |
//! | service deleted | `MemberDeleted` per binding it still held | `MemberDeleted` |
//! | placeholder absorbed into its identification | repointed, no event | repointed, no event |

use crate::server::shared::events::types::UpdateReason;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use uuid::Uuid;

use crate::server::auth::middleware::auth::AuthenticatedEntity;
use crate::server::dependencies::r#impl::base::{Dependency, DependencyMembers};
use crate::server::dependencies::r#impl::member_events::DependencyMemberChange;
use crate::server::shared::events::traits::{Event, EventFilter, Subscriber};
use crate::server::shared::services::factory::ServiceFactory;
use crate::server::shared::services::traits::CrudService;

use super::discovery_stale_port_guards::{LAN_IP, Lab};
use super::discovery_stale_port_holders::covering;
use super::{dependency, test_services};

/// Every `DependencyMemberChange` published, in order.
#[derive(Default)]
struct Heard(Mutex<Vec<(Uuid, Option<Uuid>, DependencyMemberChange)>>);

#[async_trait]
impl Subscriber<DependencyMemberChange> for Heard {
    fn filter(&self) -> EventFilter<DependencyMemberChange> {
        EventFilter::all()
    }

    async fn handle(&self, events: Vec<Event<DependencyMemberChange>>) -> anyhow::Result<()> {
        let mut heard = self.0.lock().unwrap();
        for event in events {
            heard.push((
                event.scope.dependency_id,
                event.scope.binding_id,
                event.operation,
            ));
        }
        Ok(())
    }
}

async fn listen(services: &ServiceFactory) -> Arc<Heard> {
    let heard = Arc::new(Heard::default());
    services
        .event_bus
        .register::<DependencyMemberChange>(heard.clone(), "test_heard")
        .await;
    heard
}

/// What was heard, once every subscriber has run.
async fn heard(
    services: &ServiceFactory,
    heard: &Heard,
) -> Vec<(Uuid, Option<Uuid>, DependencyMemberChange)> {
    services.event_bus.flush().await;
    heard.0.lock().unwrap().clone()
}

async fn store(
    services: &ServiceFactory,
    site_id: &Uuid,
    members: DependencyMembers,
) -> Dependency {
    let mut chain = dependency(site_id);
    chain.base.members = members;
    services
        .dependency_service
        .create(chain, AuthenticatedEntity::System)
        .await
        .unwrap()
}

async fn members_of(services: &ServiceFactory, id: &Uuid) -> DependencyMembers {
    services
        .dependency_service
        .get_by_id(id)
        .await
        .unwrap()
        .expect("dependency stored")
        .base
        .members
}

/// Caddy takes Traefik's only port, so Traefik is also deleted for it. Both members hear the move to
/// Caddy, once each, and both rows stay.
#[tokio::test]
async fn a_handover_that_empties_the_holder_is_heard_once_per_member() {
    let (storage, services, _container) = test_services().await;
    let lab = Lab::new(&services, &storage).await;
    let (lan, p443) = (lab.lan_ip(), lab.tcp(443));
    let traefik = lab.service("Traefik", vec![(&p443, &lan)]);
    lab.scan(vec![lan], vec![p443], vec![traefik]).await;
    let traefik = lab.named("Traefik").await.unwrap();
    let binding_id = traefik.base.bindings[0].id();
    let by_binding = store(
        &services,
        &lab.site_id,
        DependencyMembers::Bindings {
            binding_ids: vec![binding_id],
        },
    )
    .await;
    let by_service = store(
        &services,
        &lab.site_id,
        DependencyMembers::Services {
            service_ids: vec![traefik.id],
        },
    )
    .await;
    let listener = listen(&services).await;

    let (lan, p443) = (lab.lan_ip(), lab.tcp(443));
    let caddy = lab.service("Caddy", vec![(&p443, &lan)]);
    lab.scan_covering(vec![lan], vec![p443], vec![caddy], covering(LAN_IP, &[443]))
        .await;
    let caddy = lab.named("Caddy").await.expect("Caddy took the port");

    // The two come from separate subscribers, in either order.
    let mut events = heard(&services, &listener).await;
    events.sort_by_key(|(dependency_id, ..)| *dependency_id != by_binding.id);
    assert_eq!(
        events,
        vec![
            (
                by_binding.id,
                Some(binding_id),
                DependencyMemberChange::MemberMoved {
                    to_service_id: caddy.id,
                },
            ),
            (
                by_service.id,
                None,
                DependencyMemberChange::MemberMoved {
                    to_service_id: caddy.id,
                },
            ),
        ]
    );
    assert_eq!(
        members_of(&services, &by_binding.id).await,
        DependencyMembers::Bindings {
            binding_ids: vec![binding_id]
        }
    );
    assert_eq!(
        members_of(&services, &by_service.id).await,
        DependencyMembers::Services {
            service_ids: vec![traefik.id]
        }
    );
}

/// A handover that leaves the holder its other port: the binding member hears the move, and the
/// service member hears nothing, since the service is still there.
#[tokio::test]
async fn a_handover_leaving_the_holder_is_heard_by_the_binding_member_only() {
    let (storage, services, _container) = test_services().await;
    let lab = Lab::new(&services, &storage).await;
    let (lan, p80, p443) = (lab.lan_ip(), lab.tcp(80), lab.tcp(443));
    let traefik = lab.service("Traefik", vec![(&p80, &lan), (&p443, &lan)]);
    lab.scan(vec![lan], vec![p80, p443], vec![traefik]).await;
    let traefik = lab.named("Traefik").await.unwrap();
    let p443_id = lab.stored_tcp(443).await.id;
    let binding_id = traefik
        .base
        .bindings
        .iter()
        .find(|b| b.port_id() == Some(p443_id))
        .unwrap()
        .id();
    let by_binding = store(
        &services,
        &lab.site_id,
        DependencyMembers::Bindings {
            binding_ids: vec![binding_id],
        },
    )
    .await;
    store(
        &services,
        &lab.site_id,
        DependencyMembers::Services {
            service_ids: vec![traefik.id],
        },
    )
    .await;
    let listener = listen(&services).await;

    let (lan, p443) = (lab.lan_ip(), lab.tcp(443));
    let caddy = lab.service("Caddy", vec![(&p443, &lan)]);
    lab.scan_covering(vec![lan], vec![p443], vec![caddy], covering(LAN_IP, &[443]))
        .await;

    let events = heard(&services, &listener).await;
    assert_eq!(events.len(), 1, "{events:?}");
    assert_eq!(events[0].0, by_binding.id);
    assert!(matches!(
        events[0].2,
        DependencyMemberChange::MemberMoved { .. }
    ));
}

/// Nothing answers on a covered port past the stale window: the binding member hears the
/// release.
#[tokio::test]
async fn a_released_binding_is_heard_by_its_member() {
    let (storage, services, _container) = test_services().await;
    let lab = Lab::new(&services, &storage).await;
    let (lan, p443, p8443) = (lab.lan_ip(), lab.tcp(443), lab.tcp(8443));
    let traefik = lab.service("Traefik", vec![(&p443, &lan), (&p8443, &lan)]);
    lab.scan(vec![lan], vec![p443, p8443], vec![traefik]).await;
    let traefik = lab.named("Traefik").await.unwrap();
    let p8443_id = lab.stored_tcp(8443).await.id;
    let binding_id = traefik
        .base
        .bindings
        .iter()
        .find(|b| b.port_id() == Some(p8443_id))
        .unwrap()
        .id();
    let by_binding = store(
        &services,
        &lab.site_id,
        DependencyMembers::Bindings {
            binding_ids: vec![binding_id],
        },
    )
    .await;
    lab.age_port(8443).await;
    let listener = listen(&services).await;

    let (lan, p443) = (lab.lan_ip(), lab.tcp(443));
    let traefik = lab.service("Traefik", vec![(&p443, &lan)]);
    lab.scan_covering(
        vec![lan],
        vec![p443],
        vec![traefik],
        covering(LAN_IP, &[443, 8443]),
    )
    .await;

    assert_eq!(
        heard(&services, &listener).await,
        vec![(
            by_binding.id,
            Some(binding_id),
            DependencyMemberChange::MemberReleased
        )]
    );
}

/// Deleting a service by hand: both member types hear it, and both rows stay, readable.
#[tokio::test]
async fn a_deleted_service_is_heard_by_both_member_types_and_its_members_stay() {
    let (storage, services, _container) = test_services().await;
    let lab = Lab::new(&services, &storage).await;
    let (lan, p443) = (lab.lan_ip(), lab.tcp(443));
    let traefik = lab.service("Traefik", vec![(&p443, &lan)]);
    lab.scan(vec![lan], vec![p443], vec![traefik]).await;
    let traefik = lab.named("Traefik").await.unwrap();
    let binding_id = traefik.base.bindings[0].id();
    let by_binding = store(
        &services,
        &lab.site_id,
        DependencyMembers::Bindings {
            binding_ids: vec![binding_id],
        },
    )
    .await;
    let by_service = store(
        &services,
        &lab.site_id,
        DependencyMembers::Services {
            service_ids: vec![traefik.id],
        },
    )
    .await;
    let listener = listen(&services).await;

    services
        .service_service
        .delete(
            &traefik.id,
            AuthenticatedEntity::System,
            crate::server::shared::events::types::DeletionReason::EntityRequested,
        )
        .await
        .unwrap();

    let mut events = heard(&services, &listener).await;
    events.sort_by_key(|(dependency_id, ..)| *dependency_id != by_binding.id);
    assert_eq!(
        events,
        vec![
            (
                by_binding.id,
                Some(binding_id),
                DependencyMemberChange::MemberDeleted
            ),
            (by_service.id, None, DependencyMemberChange::MemberDeleted),
        ]
    );
    assert_eq!(
        members_of(&services, &by_binding.id).await,
        DependencyMembers::Bindings {
            binding_ids: vec![binding_id]
        }
    );
    assert_eq!(
        members_of(&services, &by_service.id).await,
        DependencyMembers::Services {
            service_ids: vec![traefik.id]
        }
    );
}

/// A dependency with a member whose binding is gone can still be saved, with the member kept.
#[tokio::test]
async fn a_dependency_with_a_missing_member_can_be_saved() {
    let (storage, services, _container) = test_services().await;
    let lab = Lab::new(&services, &storage).await;
    let (lan, p443) = (lab.lan_ip(), lab.tcp(443));
    let traefik = lab.service("Traefik", vec![(&p443, &lan)]);
    lab.scan(vec![lan], vec![p443], vec![traefik]).await;
    let traefik = lab.named("Traefik").await.unwrap();
    let binding_id = traefik.base.bindings[0].id();
    let by_binding = store(
        &services,
        &lab.site_id,
        DependencyMembers::Bindings {
            binding_ids: vec![binding_id],
        },
    )
    .await;
    services
        .service_service
        .delete(
            &traefik.id,
            AuthenticatedEntity::System,
            crate::server::shared::events::types::DeletionReason::EntityRequested,
        )
        .await
        .unwrap();

    let mut chain = services
        .dependency_service
        .get_by_id(&by_binding.id)
        .await
        .unwrap()
        .unwrap();
    chain.base.name = "Renamed".to_string();
    services
        .dependency_service
        .update(&mut chain, AuthenticatedEntity::System, UpdateReason::EntityRequested)
        .await
        .expect("a missing member does not block a save");

    assert_eq!(
        members_of(&services, &by_binding.id).await,
        DependencyMembers::Bindings {
            binding_ids: vec![binding_id]
        }
    );
}

/// A scan that hands nothing over announces nothing.
#[tokio::test]
async fn a_rescan_that_moves_nothing_is_silent() {
    let (storage, services, _container) = test_services().await;
    let lab = Lab::new(&services, &storage).await;
    let (lan, p443) = (lab.lan_ip(), lab.tcp(443));
    let traefik = lab.service("Traefik", vec![(&p443, &lan)]);
    lab.scan(vec![lan], vec![p443], vec![traefik]).await;
    let traefik = lab.named("Traefik").await.unwrap();
    store(
        &services,
        &lab.site_id,
        DependencyMembers::Services {
            service_ids: vec![traefik.id],
        },
    )
    .await;
    let listener = listen(&services).await;

    let (lan, p443) = (lab.lan_ip(), lab.tcp(443));
    let traefik = lab.service("Traefik", vec![(&p443, &lan)]);
    lab.scan_covering(
        vec![lan],
        vec![p443],
        vec![traefik],
        covering(LAN_IP, &[443]),
    )
    .await;

    assert!(heard(&services, &listener).await.is_empty());
}

/// A generic placeholder folded into its identified container: the members follow the
/// container to its identified service, and nothing is announced.
#[tokio::test]
async fn an_absorbed_placeholder_repoints_its_members_silently() {
    let (storage, services, _container) = test_services().await;
    let lab = Lab::new(&services, &storage).await;
    let (lan, p3001) = (lab.lan_ip(), lab.tcp(3001));
    let runtime = lab.runtime(&lan);
    let placeholder = lab.container_as("Docker Container", "kuma", &runtime, vec![(&p3001, &lan)]);
    lab.scan(vec![lan], vec![p3001], vec![runtime, placeholder])
        .await;
    let placeholder = lab.named("kuma").await.unwrap();
    let by_binding = store(
        &services,
        &lab.site_id,
        DependencyMembers::Bindings {
            binding_ids: vec![placeholder.base.bindings[0].id()],
        },
    )
    .await;
    let by_service = store(
        &services,
        &lab.site_id,
        DependencyMembers::Services {
            service_ids: vec![placeholder.id],
        },
    )
    .await;
    let listener = listen(&services).await;

    let lan = lab.lan_ip();
    let runtime = lab.runtime(&lan);
    let mut kuma = lab.container_as("UptimeKuma", "kuma", &runtime, vec![]);
    kuma.base.name = "Uptime Kuma".to_string();
    lab.scan(vec![lan], vec![], vec![runtime, kuma]).await;
    let kuma = lab.named("Uptime Kuma").await.expect("identified");

    assert!(heard(&services, &listener).await.is_empty());
    assert_eq!(
        members_of(&services, &by_service.id).await,
        DependencyMembers::Services {
            service_ids: vec![kuma.id]
        }
    );
    assert_eq!(
        members_of(&services, &by_binding.id).await,
        DependencyMembers::Bindings {
            binding_ids: vec![kuma.base.bindings[0].id()]
        }
    );
}
