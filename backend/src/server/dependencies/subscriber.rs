//! Telling the user a dependency lost a member.
//!
//! Members outlive what they name. When a member's binding or service is deleted, the member stays
//! and this publishes a `DependencyMemberChange` for it, read from the reason the `Deleted` event
//! carries: a port that changed hands is a move, one nothing answered on is a release, anything
//! else that deletes a service is a delete.
//!
//! Each member hears once per change. A service losing a binding to another gives up the binding
//! first and is deleted after, if at all, so its `Deleted` carries only the bindings it still
//! held: a member on a moved binding hears `MemberMoved` from the binding's own event.

use anyhow::Error;
use async_trait::async_trait;
use std::collections::HashMap;

use crate::server::{
    dependencies::{
        r#impl::member_events::{DependencyMemberChange, DependencyMemberChangeScope},
        service::DependencyService,
    },
    shared::{
        entities::{Entity, EntityDiscriminants},
        events::{
            registry::SubscriberRegistration,
            traits::{EntityEventFilter, Event, Subscriber},
            types::{DeletionReason, EntityOperation, EntityOperationDiscriminants},
        },
        services::traits::EventBusService,
    },
};

/// What a port changing hands or going quiet means to a member on it, or `None` when the
/// deletion says nothing about where the port went.
fn port_change(reason: &DeletionReason) -> Option<DependencyMemberChange> {
    match reason {
        DeletionReason::ServicePortsHandedOver { to_service_id, .. }
        | DeletionReason::ServiceOpenPortsReclaimed { to_service_id, .. } => {
            Some(DependencyMemberChange::MemberMoved {
                to_service_id: *to_service_id,
            })
        }
        DeletionReason::ServicePortsReleased => Some(DependencyMemberChange::MemberReleased),
        _ => None,
    }
}

impl DependencyService {
    async fn announce(
        &self,
        scope: DependencyMemberChangeScope,
        change: DependencyMemberChange,
        event: &Event<EntityOperation>,
    ) -> Result<(), Error> {
        self.event_bus()
            .publish(Event::new(scope, change, event.authentication.clone()))
            .await
    }
}

#[async_trait]
impl Subscriber<EntityOperation> for DependencyService {
    fn filter(&self) -> EntityEventFilter {
        let deleted = Some(vec![EntityOperationDiscriminants::Deleted]);
        EntityEventFilter::by_entity(HashMap::from([
            (EntityDiscriminants::Service, deleted.clone()),
            (EntityDiscriminants::Binding, deleted),
        ]))
    }

    async fn handle(&self, events: Vec<Event<EntityOperation>>) -> Result<(), Error> {
        for event in events {
            let EntityOperation::Deleted { reason } = &event.operation else {
                continue;
            };
            match event.scope.entity_type() {
                // A binding removed from a living service is that service's own edit, unless its
                // port changed hands or went quiet.
                Entity::Binding(binding) => {
                    let Some(change) = port_change(reason) else {
                        continue;
                    };
                    for record in self
                        .member_storage()
                        .records_for_binding(&binding.id)
                        .await?
                    {
                        self.announce(
                            DependencyMemberChangeScope {
                                site_id: binding.base.site_id,
                                dependency_id: record.dependency_id(),
                                service_id: binding.base.service_id,
                                binding_id: Some(binding.id),
                            },
                            change.clone(),
                            &event,
                        )
                        .await?;
                    }
                }
                Entity::Service(service) => {
                    // Absorption repoints the placeholder's members to the identified service
                    // before deleting it, so none are left to hear.
                    if matches!(reason, DeletionReason::ServiceIdentifiedAs { .. }) {
                        continue;
                    }
                    let change = match reason {
                        DeletionReason::ServicePortsHandedOver { to_service_id, .. } => {
                            DependencyMemberChange::MemberMoved {
                                to_service_id: *to_service_id,
                            }
                        }
                        _ => DependencyMemberChange::MemberDeleted,
                    };
                    let held: Vec<_> = service.base.bindings.iter().map(|b| b.id()).collect();
                    for record in self
                        .member_storage()
                        .records_for_service(&service.id)
                        .await?
                    {
                        // A member on a binding the service gave up before it was deleted has
                        // already heard where that binding went.
                        if let Some(binding_id) = record.binding_id()
                            && !held.contains(&binding_id)
                        {
                            continue;
                        }
                        self.announce(
                            DependencyMemberChangeScope {
                                site_id: service.base.site_id,
                                dependency_id: record.dependency_id(),
                                service_id: service.id,
                                binding_id: record.binding_id(),
                            },
                            change.clone(),
                            &event,
                        )
                        .await?;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}
inventory::submit!(SubscriberRegistration::new::<
    DependencyService,
    EntityOperation,
>());
