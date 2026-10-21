//! Which daemon is sent a credential override for a container's bridge address.
//!
//! A container on a Docker or Podman bridge has no address on the LAN. Its bridge address is an
//! IP row on the runtime host, so a host assignment covering it already becomes an override, but
//! only a daemon on that host can route to it. Any other daemon would report the address as a
//! target it did not scan.

use super::TypedCredentialMapping;
use crate::server::ip_addresses::r#impl::base::IPAddress;
use crate::server::subnets::r#impl::base::Subnet;
use std::collections::{BTreeMap, HashSet};
use uuid::Uuid;

/// The daemon a set of credential mappings is being built for, as far as bridge addresses go.
#[derive(Debug, Clone, Default)]
pub struct BridgeReach {
    /// The host the daemon runs on.
    pub daemon_host_id: Option<Uuid>,
    /// The site's Docker and Podman bridge subnets.
    pub bridge_subnet_ids: HashSet<Uuid>,
    /// Whether the daemon tries credentials at its own bridge containers
    /// (`daemons::impl::version::tries_bridge_credentials`).
    pub daemon_tries_bridges: bool,
}

impl BridgeReach {
    pub fn new(
        daemon_host_id: Option<Uuid>,
        site_subnets: &[Subnet],
        daemon_version: Option<&semver::Version>,
    ) -> Self {
        Self {
            daemon_host_id,
            bridge_subnet_ids: site_subnets
                .iter()
                .filter(|s| s.is_container_bridge_subnet())
                .map(|s| s.id)
                .collect(),
            daemon_tries_bridges: crate::server::daemons::r#impl::version::tries_bridge_credentials(
                daemon_version,
            ),
        }
    }

    /// Whether this daemon can reach a bridge address held by `host_id`.
    fn reaches(&self, host_id: Uuid) -> bool {
        self.daemon_tries_bridges && self.daemon_host_id == Some(host_id)
    }
}

/// Drop every host-assignment override at a bridge address this daemon cannot reach.
///
/// An override is at a bridge address when the host it names holds that address on a bridge
/// subnet. Overrides without a host (discovery `integration_targets`) name an address the user
/// typed and are left alone.
pub(crate) fn retain_reachable_bridge_overrides(
    mappings: &mut BTreeMap<Uuid, TypedCredentialMapping>,
    ip_addresses: &[IPAddress],
    reach: &BridgeReach,
) {
    if reach.bridge_subnet_ids.is_empty() {
        return;
    }
    let bridge_addresses: HashSet<(Uuid, std::net::IpAddr)> = ip_addresses
        .iter()
        .filter(|i| reach.bridge_subnet_ids.contains(&i.base.subnet_id))
        .map(|i| (i.base.host_id, i.base.ip_address))
        .collect();

    for typed in mappings.values_mut() {
        typed.mapping.ip_overrides.retain(|o| match o.host_id {
            Some(host_id) if bridge_addresses.contains(&(host_id, o.ip)) => reach.reaches(host_id),
            _ => true,
        });
    }
    // A mapping left with nothing to try would still be sent, and would be ordered as a
    // site default. Only a host assignment created it, so it goes with its last override.
    mappings.retain(|_, typed| {
        typed.mapping.default_credential.is_some() || !typed.mapping.ip_overrides.is_empty()
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::credentials::r#impl::{
        base::{Credential, CredentialBase},
        types::{CredentialAssignment, CredentialType, CredentialTypeDiscriminants},
    };
    use crate::server::credentials::service::apply_host_assignment;
    use crate::server::ip_addresses::r#impl::base::IPAddressBase;
    use std::net::IpAddr;

    const LAN: &str = "10.0.20.20";
    const BRIDGE: &str = "172.17.0.3";

    struct Fixture {
        runtime_host: Uuid,
        bridge_subnet: Uuid,
        bridge_ip_id: Uuid,
        credential: Credential,
        ip_addresses: Vec<IPAddress>,
    }

    fn ip(id: Uuid, host_id: Uuid, subnet_id: Uuid, address: &str) -> IPAddress {
        IPAddress {
            id,
            base: IPAddressBase {
                host_id,
                subnet_id,
                ip_address: address.parse().unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn fixture() -> Fixture {
        let runtime_host = Uuid::new_v4();
        let bridge_subnet = Uuid::new_v4();
        let bridge_ip_id = Uuid::new_v4();
        Fixture {
            runtime_host,
            bridge_subnet,
            bridge_ip_id,
            credential: Credential {
                id: Uuid::new_v4(),
                base: CredentialBase {
                    credential_type: CredentialTypeDiscriminants::SshPassword.to_credential_type(),
                    ..Default::default()
                },
                ..Default::default()
            },
            ip_addresses: vec![
                ip(Uuid::new_v4(), runtime_host, Uuid::new_v4(), LAN),
                ip(bridge_ip_id, runtime_host, bridge_subnet, BRIDGE),
            ],
        }
    }

    fn reach(f: &Fixture, daemon_host_id: Uuid, tries: bool) -> BridgeReach {
        BridgeReach {
            daemon_host_id: Some(daemon_host_id),
            bridge_subnet_ids: HashSet::from([f.bridge_subnet]),
            daemon_tries_bridges: tries,
        }
    }

    /// The overrides a host assignment produces for one daemon, after the bridge filter.
    fn dispatched(
        f: &Fixture,
        ip_address_ids: Option<Vec<Uuid>>,
        reach: &BridgeReach,
    ) -> Vec<IpAddr> {
        let mut map = BTreeMap::new();
        apply_host_assignment(
            &mut map,
            f.runtime_host,
            &CredentialAssignment {
                credential_id: f.credential.id,
                ip_address_ids,
            },
            &f.credential,
            &f.ip_addresses,
        );
        retain_reachable_bridge_overrides(&mut map, &f.ip_addresses, reach);
        let mut ips: Vec<IpAddr> = map
            .values()
            .flat_map(|t| t.mapping.ip_overrides.iter().map(|o| o.ip))
            .collect();
        ips.sort();
        ips
    }

    fn addr(s: &str) -> IpAddr {
        s.parse().unwrap()
    }

    /// Pins the expansion the filter works on: a host-wide assignment covers the bridge address
    /// of a container counted on its host, as it did before bridge addresses were tried at all.
    #[test]
    fn host_wide_assignment_expands_to_bridge_addresses() {
        let f = fixture();
        let mut map = BTreeMap::new();
        apply_host_assignment(
            &mut map,
            f.runtime_host,
            &CredentialAssignment {
                credential_id: f.credential.id,
                ip_address_ids: None,
            },
            &f.credential,
            &f.ip_addresses,
        );
        let ips: HashSet<IpAddr> = map
            .values()
            .flat_map(|t| t.mapping.ip_overrides.iter().map(|o| o.ip))
            .collect();
        assert!(ips.contains(&addr(BRIDGE)));
    }

    #[test]
    fn the_runtime_hosts_daemon_gets_bridge_overrides_host_wide_and_scoped() {
        let f = fixture();
        let local = reach(&f, f.runtime_host, true);
        assert_eq!(dispatched(&f, None, &local), vec![addr(LAN), addr(BRIDGE)]);
        assert_eq!(
            dispatched(&f, Some(vec![f.bridge_ip_id]), &local),
            vec![addr(BRIDGE)]
        );
    }

    #[test]
    fn another_daemon_gets_no_bridge_overrides() {
        let f = fixture();
        let remote = reach(&f, Uuid::new_v4(), true);
        assert_eq!(dispatched(&f, None, &remote), vec![addr(LAN)]);
        // A scoped bridge-only assignment leaves nothing to send, so no mapping at all, and
        // nothing for the remote daemon to call a target it did not scan.
        assert!(dispatched(&f, Some(vec![f.bridge_ip_id]), &remote).is_empty());
    }

    #[test]
    fn a_daemon_that_cannot_try_bridges_gets_none_even_on_the_runtime_host() {
        let f = fixture();
        let old_local = reach(&f, f.runtime_host, false);
        assert_eq!(dispatched(&f, None, &old_local), vec![addr(LAN)]);
    }

    /// Bridge subnets reuse the same ranges on every Docker host, so the same address on another
    /// host's bridge must not decide whether this host's override is a bridge one.
    #[test]
    fn the_same_bridge_address_on_another_host_does_not_filter_a_lan_override() {
        let mut f = fixture();
        let other_host = Uuid::new_v4();
        f.ip_addresses
            .push(ip(Uuid::new_v4(), other_host, f.bridge_subnet, LAN));
        let remote = reach(&f, Uuid::new_v4(), true);
        assert_eq!(dispatched(&f, None, &remote), vec![addr(LAN)]);
    }

    #[test]
    fn a_site_default_survives_losing_its_bridge_overrides() {
        let f = fixture();
        let mut map = BTreeMap::new();
        apply_host_assignment(
            &mut map,
            f.runtime_host,
            &CredentialAssignment {
                credential_id: f.credential.id,
                ip_address_ids: Some(vec![f.bridge_ip_id]),
            },
            &f.credential,
            &f.ip_addresses,
        );
        map.get_mut(&f.credential.id)
            .unwrap()
            .mapping
            .default_credential = Some(CredentialType::to_query_payload(
            &f.credential.base.credential_type,
        ));
        retain_reachable_bridge_overrides(
            &mut map,
            &f.ip_addresses,
            &reach(&f, Uuid::new_v4(), true),
        );
        let typed = map
            .get(&f.credential.id)
            .expect("site default keeps the mapping");
        assert!(typed.mapping.ip_overrides.is_empty());
    }
}
