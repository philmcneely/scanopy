//! Telling the user a dependency lost a member.
//!
//! Members outlive what they name. When a member's binding changes hands (`BindingOwnership`) or
//! its service is deleted (`Service` `Deleted`), the member stays and this publishes a
//! `DependencyMemberChange` for it, one per affected member.
//!
//! Each member gets one event per change. A service losing a binding to another first gives up the
//! binding and is deleted after, if at all, so its `Deleted` carries only the bindings it still
//! held: a member on a moved binding hears `MemberMoved`, never also `MemberDeleted`.

use anyhow::Error;
use async_trait::async_trait;
use std::collections::HashMap;

use crate::server::{
    bindings::r#impl::ownership_events::BindingOwnership,
    dependencies::{
        r#impl::member_events::{DependencyMemberChange, DependencyMemberChangeScope},
        service::DependencyService,
    },
    shared::{
        entities::{Entity, EntityDiscriminants},
        events::{
            registry::SubscriberRegistration,
            traits::{EntityEventFilter, Event, EventFilter, Subscriber},
            types::{EntityOperation, EntityOperationDiscriminants},
        },
        services::traits::EventBusService,
    },
};

impl DependencyService {
    async fn announce(
        &self,
        scope: DependencyMemberChangeScope,
        change: DependencyMemberChange,
        event: &Event<impl crate::server::shared::events::traits::Operation>,
    ) -> Result<(), Error> {
        self.event_bus()
            .publish(Event::new(scope, change, event.authentication.clone()))
            .await
    }
}

#[async_trait]
impl Subscriber<BindingOwnership> for DependencyService {
    fn filter(&self) -> EventFilter<BindingOwnership> {
        EventFilter::all()
    }

    async fn handle(&self, events: Vec<Event<BindingOwnership>>) -> Result<(), Error> {
        for event in events {
            let change = match &event.operation {
                BindingOwnership::Moved {
                    to_service_id,
                    to_binding_id,
                } => DependencyMemberChange::MemberMoved {
                    to_service_id: *to_service_id,
                    to_binding_id: *to_binding_id,
                },
                BindingOwnership::Released => DependencyMemberChange::MemberReleased,
            };
            for record in self
                .member_storage()
                .records_for_binding(&event.scope.from_binding_id)
                .await?
            {
                self.announce(
                    DependencyMemberChangeScope {
                        site_id: event.scope.site_id,
                        dependency_id: record.dependency_id(),
                        service_id: event.scope.from_service_id,
                        binding_id: Some(event.scope.from_binding_id),
                    },
                    change.clone(),
                    &event,
                )
                .await?;
            }
        }
        Ok(())
    }
}
inventory::submit!(SubscriberRegistration::new::<
    DependencyService,
    BindingOwnership,
>());

#[async_trait]
impl Subscriber<EntityOperation> for DependencyService {
    fn filter(&self) -> EntityEventFilter {
        EntityEventFilter::by_entity(HashMap::from([(
            EntityDiscriminants::Service,
            Some(vec![EntityOperationDiscriminants::Deleted]),
        )]))
    }

    async fn handle(&self, events: Vec<Event<EntityOperation>>) -> Result<(), Error> {
        for event in events {
            let Entity::Service(service) = event.scope.entity_type() else {
                continue;
            };
            let held: Vec<_> = service.base.bindings.iter().map(|b| b.id()).collect();
            for record in self
                .member_storage()
                .records_for_service(&service.id)
                .await?
            {
                // A member on a binding the service no longer held when deleted has already heard
                // where that binding went.
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
                    DependencyMemberChange::MemberDeleted,
                    &event,
                )
                .await?;
            }
        }
        Ok(())
    }
}
inventory::submit!(SubscriberRegistration::new::<
    DependencyService,
    EntityOperation,
>());
