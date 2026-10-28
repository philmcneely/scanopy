//! Retiring addresses a daemon no longer reports for its own host.
//!
//! The daemon reads its own NICs every scan and reports every address it holds on each subnet it
//! reports. That list is complete and deterministic within those subnets, so an address the daemon
//! once reported there and no longer does is gone: a VM network on the daemon's machine that was
//! torn down, a NIC re-addressed. Left in place, the address keeps matching whatever device holds
//! it next, and that device's services land on the daemon's host.
use super::*;

/// The daemon host's stored rows the incoming self-report no longer lists.
///
/// Only when every incoming address is the daemon's own report, and only among stored rows the
/// daemon itself filed, on a subnet the report covers: an address on a subnet the report does not
/// cover was not observed either way, and loopback is seeded at registration.
pub(crate) fn stale_self_reported_rows(
    stored: &[IPAddress],
    incoming: &[IPAddress],
) -> HashSet<Uuid> {
    if incoming.is_empty()
        || incoming
            .iter()
            .any(|i| i.base.ip_address_source != AttributeSource::DaemonSelfReport)
    {
        return HashSet::new();
    }
    let covered: HashSet<Uuid> = incoming.iter().map(|i| i.base.subnet_id).collect();
    stored
        .iter()
        .filter(|row| {
            row.base.ip_address_source == AttributeSource::DaemonSelfReport
                && !row.base.ip_address.is_loopback()
                && covered.contains(&row.base.subnet_id)
                && !incoming.iter().any(|i| ip_addresses_share_address(i, row))
        })
        .map(|row| row.id)
        .collect()
}

impl HostService {
    /// Delete `rows` from the daemon's `host`, with the services observed only at them and the
    /// ports nothing else binds. A service bound to them and other addresses keeps the rest.
    pub(crate) async fn retire_self_reported_addresses(
        &self,
        host: &Host,
        rows: &HashSet<Uuid>,
        authentication: &AuthenticatedEntity,
    ) -> Result<()> {
        if rows.is_empty() {
            return Ok(());
        }
        let binds_retired_row = |binding: &Binding| match binding.base.binding_type {
            BindingType::IPAddress { ip_address_id } => rows.contains(&ip_address_id),
            BindingType::Port { ip_address_id, .. } => {
                ip_address_id.is_some_and(|id| rows.contains(&id))
            }
        };
        let port_of = |binding: &Binding| match binding.base.binding_type {
            BindingType::Port { port_id, .. } => Some(port_id),
            BindingType::IPAddress { .. } => None,
        };
        let services = self
            .service_service
            .get_all(StorableFilter::<Service>::new_from_host_ids(&[host.id]).live())
            .await?;
        let (retiring, staying): (Vec<Service>, Vec<Service>) =
            services.into_iter().partition(|s| {
                !s.base.bindings.is_empty() && s.base.bindings.iter().all(binds_retired_row)
            });
        let ports_still_bound: HashSet<Uuid> = staying
            .iter()
            .flat_map(|s| &s.base.bindings)
            .filter(|b| !binds_retired_row(b))
            .filter_map(port_of)
            .collect();
        let orphaned_ports: HashSet<Uuid> = retiring
            .iter()
            .flat_map(|s| &s.base.bindings)
            .filter_map(port_of)
            .filter(|id| !ports_still_bound.contains(id))
            .collect();

        tracing::info!(
            host_id = %host.id,
            addresses = rows.len(),
            services = retiring.len(),
            "Retiring addresses the daemon no longer reports for its own host"
        );
        for service in retiring {
            self.service_service
                .delete(
                    &service.id,
                    authentication.clone(),
                    DeletionReason::EntityNoLongerReported,
                )
                .await?;
        }
        for port_id in orphaned_ports {
            self.port_service
                .delete(
                    &port_id,
                    authentication.clone(),
                    DeletionReason::EntityNoLongerReported,
                )
                .await?;
        }
        for id in rows {
            self.ip_address_service
                .delete(
                    id,
                    authentication.clone(),
                    DeletionReason::EntityNoLongerReported,
                )
                .await?;
        }
        self.ip_address_service
            .renumber_ip_addresses_for_host(&host.id, authentication.clone())
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::ip_addresses::r#impl::base::IPAddressBase;

    fn own(ip: &str, subnet_id: Uuid, source: AttributeSource) -> IPAddress {
        IPAddress::new(IPAddressBase {
            ip_address: ip.parse().unwrap(),
            subnet_id,
            ip_address_source: source,
            ..Default::default()
        })
    }

    /// The Mac daemon host on 2026-10-22: `bridge102`'s 192.168.4.1 from a VMware network that no
    /// longer exists, beside en0's 192.168.5.93 on the same /22. Today's report lists en0 and lo.
    #[test]
    fn an_address_the_report_no_longer_lists_on_a_covered_subnet_is_stale() {
        let (lan, loopback, docker, old_lan) = (
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
        );
        let bridge102 = own("192.168.4.1", lan, AttributeSource::DaemonSelfReport);
        let stored = vec![
            own("192.168.5.93", lan, AttributeSource::DaemonSelfReport),
            bridge102.clone(),
            own("127.0.0.1", loopback, AttributeSource::DaemonSelfReport),
            own(
                "172.17.0.2",
                docker,
                AttributeSource::Probe(
                    crate::server::services::r#impl::patterns::ClientProbe::Docker,
                ),
            ),
            // The Mac's address on the network it was on the day before, which today's report
            // does not cover: not observed, so kept.
            own("192.168.1.133", old_lan, AttributeSource::DaemonSelfReport),
        ];
        let report = [
            own("192.168.5.93", lan, AttributeSource::DaemonSelfReport),
            own("127.0.0.1", loopback, AttributeSource::DaemonSelfReport),
        ];

        assert_eq!(
            stale_self_reported_rows(&stored, &report),
            HashSet::from([bridge102.id])
        );
    }

    /// Any payload that is not the daemon's own report (a sweep, an SNMP read) retires nothing.
    #[test]
    fn a_payload_that_is_not_a_self_report_retires_nothing() {
        let lan = Uuid::new_v4();
        let stored = vec![
            own("192.168.5.93", lan, AttributeSource::DaemonSelfReport),
            own("192.168.4.1", lan, AttributeSource::DaemonSelfReport),
        ];
        let sweep = [own("192.168.5.93", lan, AttributeSource::ArpReply)];
        assert!(stale_self_reported_rows(&stored, &sweep).is_empty());
        assert!(stale_self_reported_rows(&stored, &[]).is_empty());
    }
}
