//! Taking ports back from Unclaimed Open Ports while a discovery payload is stored.
//!
//! Unclaimed Open Ports holds the ports a scan found open with no service to give them to. A
//! service that later claims one of them takes it, whether the service is new to the host or was
//! stored by an earlier scan, so the placeholder never outranks a real owner (GH #784).
use super::stale_ports::{TakenBinding, overlaps};
use super::*;

/// A port binding as the (port, address) claim the conflict checks compare.
pub(crate) fn port_claim(binding: &Binding) -> Option<(Uuid, Option<Uuid>)> {
    match binding.base.binding_type {
        BindingType::Port {
            port_id,
            ip_address_id,
        } => Some((port_id, ip_address_id)),
        BindingType::IPAddress { .. } => None,
    }
}

/// Whether an Unclaimed Open Ports service among `services` binds the port of `claim` at an
/// overlapping address (an unscoped binding overlaps every address).
pub(crate) fn held_by_open_ports(claim: &(Uuid, Option<Uuid>), services: &[Service]) -> bool {
    let (port_id, claim_iface) = claim;
    services.iter().any(|s| {
        ServiceDefinitionExt::is_open_ports(&s.base.service_definition)
            && s.base.bindings.iter().any(|b| {
                b.port_id() == Some(*port_id) && bindings_overlap(claim_iface, &b.ip_address_id())
            })
    })
}

impl HostService {
    /// Remove `claims` from the host's Unclaimed Open Ports service, deleting the service when no
    /// binding is left, and mirror the change in `services` so later services in the payload see
    /// it. Returns the bindings taken, for the caller to announce once it knows where they went.
    pub(crate) async fn reclaim_from_open_ports(
        &self,
        claims: &[(Uuid, Option<Uuid>)],
        services: &mut [Service],
        claimant: &str,
        authentication: AuthenticatedEntity,
    ) -> Vec<TakenBinding> {
        let Some(open_ports) = services
            .iter_mut()
            .find(|s| ServiceDefinitionExt::is_open_ports(&s.base.service_definition))
        else {
            return Vec::new();
        };
        let taken = TakenBinding::from_holder(open_ports, claims);
        if taken.is_empty() {
            return taken;
        }
        let open_ports_id = open_ports.id;
        open_ports
            .base
            .bindings
            .retain(|b| !claims.iter().any(|claim| overlaps(b, claim)));
        let remaining_bindings = open_ports.base.bindings.len();

        tracing::info!(
            service_name = %claimant,
            reclaimed_ports = ?claims,
            remaining_bindings,
            "Reclaiming ports from Unclaimed Open Ports service"
        );
        // The taken bindings go first, so deleting the emptied service below reports none of them
        // as deleted with it.
        let reclaimed: Vec<(Uuid, Option<Uuid>)> =
            taken.iter().map(|t| (t.port_id, t.ip_address_id)).collect();
        let _ = self
            .service_service
            .remove_port_bindings(&open_ports_id, &reclaimed, authentication.clone())
            .await;
        if remaining_bindings == 0 {
            tracing::info!(
                service_name = %claimant,
                "Deleting Unclaimed Open Ports service after all ports reclaimed"
            );
            let _ = self
                .service_service
                .delete(&open_ports_id, authentication)
                .await;
        }
        taken
    }
}
