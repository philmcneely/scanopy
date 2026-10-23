//! Removing containers and VMs a run no longer reported.
//!
//! A collection that listed every guest of a host, stopped ones included, names that host in its
//! credential result (`CredentialRunOutcome::Collected::hosts_with_guests_listed`). A stored guest
//! of one of that host's runtime or hypervisor services that the run did not report was removed
//! from it, and is deleted. Nothing else is: a host the run didn't list, a listed host whose own
//! submission didn't land, a guest of a virtualizer the credential's integration doesn't report,
//! and anything a person created.
use std::collections::HashSet;

use super::*;
use crate::server::credentials::r#impl::run_results::{CredentialRunOutcome, CredentialRunResult};
use crate::server::credentials::r#impl::types::CredentialTypeDiscriminants;
use crate::server::daemons::r#impl::api::ScannedEntityIds;
use crate::server::shared::storage::filter::StorableFilter;

impl HostService {
    /// Delete the guests a run no longer reported under the hosts whose guests it listed in full.
    pub(crate) async fn remove_unreported_guests(
        &self,
        site_id: Uuid,
        scanned: &ScannedEntityIds,
        credential_results: &[CredentialRunResult],
        authentication: AuthenticatedEntity,
    ) -> Result<()> {
        let scanned_hosts: HashSet<Uuid> = scanned.host_ids.iter().copied().collect();
        let scanned_services: HashSet<Uuid> = scanned.service_ids.iter().copied().collect();

        for result in credential_results {
            let CredentialRunOutcome::Collected {
                hosts_with_guests_listed,
                ..
            } = &result.outcome
            else {
                continue;
            };
            if hosts_with_guests_listed.is_empty() {
                continue;
            }
            let Some(credential) = self
                .credential_service
                .get_by_id(&result.credential_id)
                .await?
            else {
                continue;
            };
            let declared = CredentialTypeDiscriminants::from(&credential.base.credential_type)
                .integration()
                .host_virtualizations();

            for host_id in hosts_with_guests_listed {
                if !scanned_hosts.contains(host_id) {
                    continue;
                }
                let parents: Vec<Uuid> = self
                    .service_service
                    .get_all(
                        StorableFilter::<Service>::new_from_site_ids(&[site_id])
                            .host_id(host_id)
                            .live(),
                    )
                    .await?
                    .into_iter()
                    .filter(|s| {
                        s.base
                            .service_definition
                            .virtualization_role()
                            .is_some_and(|role| declared.contains(&role.hosts()))
                    })
                    .map(|s| s.id)
                    .collect();
                if parents.is_empty() {
                    continue;
                }

                let guest_services = self
                    .service_service
                    .get_all(
                        StorableFilter::<Service>::new_from_site_ids(&[site_id])
                            .virtualization_service_in(&parents, false)
                            .live(),
                    )
                    .await?;
                for guest in guest_services.into_iter().filter(|s| {
                    s.base.source.is_from_discovery() && !scanned_services.contains(&s.id)
                }) {
                    if let Err(e) = self
                        .service_service
                        .delete(
                            &guest.id,
                            authentication.clone(),
                            no_longer_listed(&guest.base.virtualization_service_id),
                        )
                        .await
                    {
                        tracing::warn!(service_id = %guest.id, error = %e, "Failed to delete a removed container");
                    }
                }

                let guest_hosts = self
                    .get_all(
                        StorableFilter::<Host>::new_from_site_ids(&[site_id])
                            .virtualization_service_in(&parents, false)
                            .live(),
                    )
                    .await?;
                for guest in guest_hosts
                    .into_iter()
                    .filter(|h| h.base.source.is_from_discovery() && !scanned_hosts.contains(&h.id))
                {
                    if let Err(e) = self
                        .delete_host(
                            &guest.id,
                            authentication.clone(),
                            no_longer_listed(&guest.base.virtualization_service_id),
                        )
                        .await
                    {
                        tracing::warn!(host_id = %guest.id, error = %e, "Failed to delete a removed guest");
                    }
                }
            }
        }
        Ok(())
    }
}

/// The reason for deleting a guest of `parent`, which the filter that found it guarantees is set.
fn no_longer_listed(parent: &Option<Uuid>) -> DeletionReason {
    DeletionReason::EntityGuestNoLongerListed {
        parent_service_id: parent.unwrap_or_default(),
    }
}
