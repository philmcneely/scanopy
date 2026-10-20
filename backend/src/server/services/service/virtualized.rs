//! Deleting what a service virtualizes.
//!
//! A runtime's containers, a hypervisor's VMs, a guest's network identities and a runtime's
//! bridge subnets exist only under the service that runs them. The FK from a child's
//! `virtualization_service_id` is `ON DELETE SET NULL`, which would leave them unlinked; and a
//! row the database removes publishes no event. So a service deletes its children through their
//! own services first, each emitting its own `Deleted` and deleting its own children in turn.
use super::*;
use crate::server::subnets::r#impl::base::Subnet;

impl ServiceService {
    /// Delete the services, hosts and subnets `service` virtualizes.
    pub(super) async fn delete_virtualized(
        &self,
        service: &Service,
        authentication: AuthenticatedEntity,
    ) -> Result<()> {
        let site_id = service.base.site_id;
        let parent = [service.id];
        let reason = DeletionReason::EntityVirtualizerDeleted {
            virtualizer_service_id: service.id,
        };

        let children = self
            .get_all(
                StorableFilter::<Service>::new_from_site_ids(&[site_id])
                    .virtualization_service_in(&parent, false)
                    .live(),
            )
            .await?;
        for child in children.into_iter().filter(|c| c.id != service.id) {
            self.delete(&child.id, authentication.clone(), reason.clone())
                .await?;
        }

        if let Some(host_service) = self.host_service.get() {
            let hosts = host_service
                .get_all(
                    StorableFilter::<Host>::new_from_site_ids(&[site_id])
                        .virtualization_service_in(&parent, false)
                        .live(),
                )
                .await?;
            // The service's own host is the one being deleted when this runs from a host delete.
            for host in hosts.into_iter().filter(|h| h.id != service.base.host_id) {
                host_service
                    .delete_host(&host.id, authentication.clone(), reason.clone())
                    .await?;
            }
        }

        let subnets = self
            .subnet_service
            .get_all(
                StorableFilter::<Subnet>::new_from_site_ids(&[site_id])
                    .virtualization_service_in(&parent, false)
                    .live(),
            )
            .await?;
        for subnet in subnets {
            self.subnet_service
                .delete(&subnet.id, authentication.clone(), reason.clone())
                .await?;
        }
        Ok(())
    }
}
