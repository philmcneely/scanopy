//! The event a port binding changing hands is.
//!
//! Binding ids never move between services: when a port changes hands the old binding is deleted
//! and the new owner is saved with a binding of its own. A `Binding` `Deleted` event cannot tell a
//! move from a removal, and says nothing about where the port went, so a move is its own
//! operation. Published after the claimant is stored, so both ids are real.
//!
//! The dependency service subscribes: a dependency naming the old binding has lost that member,
//! and `DependencyMemberChange` says so.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use strum::IntoDiscriminant;
use strum_macros::{AsRefStr, EnumDiscriminants};
use uuid::Uuid;

use crate::server::shared::events::EventFlags;
use crate::server::shared::events::registry::to_snake_case;
use crate::server::shared::events::traits::{
    EventFilter, EventScope, Operation, ScopeOrganization,
};
use crate::server::shared::events::types::EventLogLevel;

/// What happened to a port binding a stored service held.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, EnumDiscriminants)]
#[strum_discriminants(derive(Hash, AsRefStr, Serialize, Deserialize))]
pub enum BindingOwnership {
    /// Another service answered on the port and took it.
    Moved {
        /// The service that took the port.
        to_service_id: Uuid,
        /// Its binding on the port.
        to_binding_id: Uuid,
    },
    /// Nothing answered on the port past the site's stale window, so it was released.
    Released,
}

/// The binding that changed hands, and where it was.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BindingOwnershipScope {
    pub site_id: Uuid,
    pub port_id: Uuid,
    pub ip_address_id: Option<Uuid>,
    pub from_service_id: Uuid,
    pub from_binding_id: Uuid,
}

impl Operation for BindingOwnership {
    type Scope = BindingOwnershipScope;
    type Flags = EventFlags;
    type Filter = EventFilter<BindingOwnership>;

    fn log_level(&self) -> EventLogLevel {
        // Discovery working as intended: the port is where the software now is.
        EventLogLevel::Info
    }

    /// `binding_moved` or `binding_released`.
    fn event_name(&self, _scope: &BindingOwnershipScope) -> Cow<'static, str> {
        format!("binding_{}", to_snake_case(self.discriminant().as_ref())).into()
    }

    fn metadata<'a>(&'a self, scope: &'a BindingOwnershipScope) -> impl Serialize + Send + 'a {
        BindingOwnershipMetadata {
            scope,
            ownership: self,
        }
    }
}

impl EventScope for BindingOwnershipScope {
    fn organization(&self) -> Option<ScopeOrganization> {
        Some(ScopeOrganization::Site(self.site_id))
    }
}

/// What leaves the server about a move: ids only.
#[derive(Serialize)]
struct BindingOwnershipMetadata<'a> {
    #[serde(flatten)]
    scope: &'a BindingOwnershipScope,
    ownership: &'a BindingOwnership,
}
