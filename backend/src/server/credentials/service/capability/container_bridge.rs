//! Which daemon is sent a credential override for a container's bridge address.
//!
//! A container on a Docker or Podman bridge has no address on the LAN. Its bridge address is an
//! IP row on the runtime host, so a host assignment covering it already becomes an override, but
//! only a daemon on that host can route to it. Two separate questions follow:
//!
//! - **Reachability** ([`retain_reachable_bridge_overrides`]): a daemon on another machine can
//!   never reach the address, so it is not sent it, silently. No upgrade would change that.
//! - **Capability** ([`TargetsContainerBridgeWithCredentials`]): the runtime host's own daemon
//!   reaches it, but only from the release that tries credentials at its bridge containers. An
//!   older one is not sent it, and the scan says which addresses an upgrade would add.

use super::DaemonCapability;
use crate::server::credentials::r#impl::capability::{CapabilityGap, ContainerBridgeGap};
use crate::server::credentials::service::TypedCredentialMapping;
use crate::server::daemons::r#impl::version::last_without_bridge_credentials;
use crate::server::ip_addresses::r#impl::base::IPAddress;
use crate::server::subnets::r#impl::base::Subnet;
use semver::Version;
use std::collections::{BTreeMap, HashSet};
use std::net::IpAddr;
use uuid::Uuid;

/// Where the daemon a dispatch is built for runs, as far as bridge addresses go.
#[derive(Debug, Clone, Default)]
pub struct DaemonPlacement {
    /// The host the daemon runs on.
    pub daemon_host_id: Option<Uuid>,
    /// The site's Docker and Podman bridge subnets.
    pub bridge_subnet_ids: HashSet<Uuid>,
}

impl DaemonPlacement {
    pub fn new(daemon_host_id: Option<Uuid>, site_subnets: &[Subnet]) -> Self {
        Self {
            daemon_host_id,
            bridge_subnet_ids: site_subnets
                .iter()
                .filter(|s| s.is_container_bridge_subnet())
                .map(|s| s.id)
                .collect(),
        }
    }
}

/// The site's bridge addresses, each with the host that holds it, and the daemon's host.
pub(crate) struct ContainerBridgeContext {
    daemon_host_id: Option<Uuid>,
    bridge_addresses: HashSet<(Uuid, IpAddr)>,
}

impl ContainerBridgeContext {
    pub(crate) fn new(placement: &DaemonPlacement, ip_addresses: &[IPAddress]) -> Self {
        Self {
            daemon_host_id: placement.daemon_host_id,
            bridge_addresses: ip_addresses
                .iter()
                .filter(|i| placement.bridge_subnet_ids.contains(&i.base.subnet_id))
                .map(|i| (i.base.host_id, i.base.ip_address))
                .collect(),
        }
    }

    /// The host holding `ip` on a bridge, for an override naming `host_id`. Bridge ranges repeat
    /// on every Docker host, so the address alone does not say whether it is a bridge one.
    fn is_bridge(&self, host_id: Option<Uuid>, ip: IpAddr) -> Option<Uuid> {
        host_id.filter(|h| self.bridge_addresses.contains(&(*h, ip)))
    }
}

/// Drop every host-assignment override at another host's bridge address: this daemon cannot
/// route to it. Overrides without a host (discovery `integration_targets`) name an address the
/// user typed and are left alone.
pub(crate) fn retain_reachable_bridge_overrides(
    ctx: &ContainerBridgeContext,
    mappings: &mut BTreeMap<Uuid, TypedCredentialMapping>,
) {
    for typed in mappings.values_mut() {
        typed
            .mapping
            .ip_overrides
            .retain(|o| match ctx.is_bridge(o.host_id, o.ip) {
                Some(host_id) => ctx.daemon_host_id == Some(host_id),
                None => true,
            });
    }
}

/// Trying a credential at a container's bridge address on the daemon's own host.
pub(crate) struct TargetsContainerBridgeWithCredentials;

impl From<ContainerBridgeGap> for CapabilityGap {
    fn from(gap: ContainerBridgeGap) -> Self {
        CapabilityGap::TargetsContainerBridgeWithCredentials(gap)
    }
}

impl DaemonCapability for TargetsContainerBridgeWithCredentials {
    type Context = ContainerBridgeContext;
    type Gap = ContainerBridgeGap;

    fn gap(ctx: &Self::Context, credential: &TypedCredentialMapping) -> Option<Self::Gap> {
        let addresses: Vec<IpAddr> = credential
            .mapping
            .ip_overrides
            .iter()
            .filter(|o| ctx.is_bridge(o.host_id, o.ip).is_some())
            .map(|o| o.ip)
            .collect();
        (!addresses.is_empty()).then_some(ContainerBridgeGap { addresses })
    }

    fn last_without(_: &Self::Gap) -> Version {
        last_without_bridge_credentials()
    }

    fn withhold(gap: &Self::Gap, credential: &mut TypedCredentialMapping) {
        credential
            .mapping
            .ip_overrides
            .retain(|o| !gap.addresses.contains(&o.ip));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::daemon::discovery::types::warnings::DiscoveryWarning;
    use crate::server::credentials::r#impl::{
        base::{Credential, CredentialBase},
        types::{CredentialAssignment, CredentialTypeDiscriminants},
    };
    use crate::server::credentials::service::apply_host_assignment;
    use crate::server::credentials::service::capability::retain_capable;
    use crate::server::ip_addresses::r#impl::base::IPAddressBase;

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

    fn placement(f: &Fixture, daemon_host_id: Uuid) -> DaemonPlacement {
        DaemonPlacement {
            daemon_host_id: Some(daemon_host_id),
            bridge_subnet_ids: HashSet::from([f.bridge_subnet]),
        }
    }

    fn version(tries: bool) -> Version {
        let last = last_without_bridge_credentials();
        if tries {
            Version::new(last.major, last.minor, last.patch + 1)
        } else {
            last
        }
    }

    /// Reachability then the capability, as `build_all_credential_mappings` runs them: the
    /// overrides a host assignment leaves for one daemon, and the warnings raised.
    fn dispatched(
        f: &Fixture,
        ip_address_ids: Option<Vec<Uuid>>,
        daemon_host_id: Uuid,
        tries: bool,
    ) -> (Vec<IpAddr>, Vec<DiscoveryWarning>) {
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
        let ctx = ContainerBridgeContext::new(&placement(f, daemon_host_id), &f.ip_addresses);
        retain_reachable_bridge_overrides(&ctx, &mut map);
        let warnings = retain_capable::<TargetsContainerBridgeWithCredentials>(
            &ctx,
            &mut map,
            Some(&version(tries)),
        );
        let mut ips: Vec<IpAddr> = map
            .values()
            .flat_map(|t| t.mapping.ip_overrides.iter().map(|o| o.ip))
            .collect();
        ips.sort();
        (ips, warnings)
    }

    fn addr(s: &str) -> IpAddr {
        s.parse().unwrap()
    }

    /// Pins the expansion the filters work on: a host-wide assignment covers the bridge address
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
        let (ips, warnings) = dispatched(&f, None, f.runtime_host, true);
        assert_eq!(ips, vec![addr(LAN), addr(BRIDGE)]);
        assert!(warnings.is_empty());
        let (ips, _) = dispatched(&f, Some(vec![f.bridge_ip_id]), f.runtime_host, true);
        assert_eq!(ips, vec![addr(BRIDGE)]);
    }

    /// A daemon elsewhere could never reach the container, so it is told nothing, whatever its
    /// version.
    #[test]
    fn another_daemon_gets_no_bridge_overrides_and_no_warning() {
        let f = fixture();
        for tries in [true, false] {
            let (ips, warnings) = dispatched(&f, None, Uuid::new_v4(), tries);
            assert_eq!(ips, vec![addr(LAN)]);
            assert!(warnings.is_empty());
            let (ips, warnings) = dispatched(&f, Some(vec![f.bridge_ip_id]), Uuid::new_v4(), tries);
            assert!(ips.is_empty());
            assert!(warnings.is_empty());
        }
    }

    /// The runtime host's own daemon, when too old, keeps the LAN address and is told which bridge
    /// addresses an upgrade would add, whether the assignment is scoped or host-wide.
    #[test]
    fn an_old_runtime_host_daemon_is_told_which_bridge_addresses_it_missed() {
        let f = fixture();
        for scope in [None, Some(vec![f.bridge_ip_id])] {
            let host_wide = scope.is_none();
            let (ips, warnings) = dispatched(&f, scope, f.runtime_host, false);
            assert_eq!(ips, if host_wide { vec![addr(LAN)] } else { vec![] });
            match warnings.as_slice() {
                [
                    DiscoveryWarning::CredentialNeedsNewerDaemon {
                        credential_id: Some(id),
                        gap: CapabilityGap::TargetsContainerBridgeWithCredentials(gap),
                        last_unsupported_daemon_version,
                        ..
                    },
                ] => {
                    assert_eq!(*id, f.credential.id);
                    assert_eq!(gap.addresses, vec![addr(BRIDGE)]);
                    assert_eq!(*last_unsupported_daemon_version, version(false));
                }
                other => panic!("expected one bridge gap, got {other:?}"),
            }
        }
    }

    /// Bridge ranges repeat on every Docker host, so the same address on another host's bridge
    /// must not decide whether this host's override is a bridge one.
    #[test]
    fn the_same_bridge_address_on_another_host_does_not_filter_a_lan_override() {
        let mut f = fixture();
        f.ip_addresses
            .push(ip(Uuid::new_v4(), Uuid::new_v4(), f.bridge_subnet, LAN));
        let (ips, warnings) = dispatched(&f, None, Uuid::new_v4(), true);
        assert_eq!(ips, vec![addr(LAN)]);
        assert!(warnings.is_empty());
    }
}
