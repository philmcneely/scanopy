//! Credentials at the daemon's own bridge containers.
//!
//! A container on a Docker or Podman bridge has no LAN address. The container scan records its
//! bridge address on the runtime host, and only this daemon, on that host, can route to it. The
//! network phase never visits bridge subnets, so after the container scan this runs the same
//! per-address integration dispatch the network phase runs, at each running container that holds
//! a credential override.
//!
//! Results attach to the runtime host: the services, ports and credential assignment the
//! integration finds, bound to the bridge address and marked as the container's. What it reads
//! about the machine it reached (hostname, OS, interfaces, addresses) describes the container, not
//! the runtime host, so it is dropped.

use super::base::DiscoveryRunner;
use super::ops::{DiscoveryOps, HostData};
use crate::daemon::discovery::integration::dispatch;
use crate::server::bindings::r#impl::base::BindingType;
use crate::server::credentials::r#impl::mapping::{CredentialMapping, CredentialQueryPayload};
use crate::server::ip_addresses::r#impl::base::IPAddress;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::r#impl::base::ServiceMatchBaselineParams;
use crate::server::services::r#impl::virtualization::ServiceVirtualization;
use crate::server::shared::attribution::AttributeSource;
use crate::server::shared::types::guest_state::GuestState;
use crate::server::subnets::r#impl::base::Subnet;
use anyhow::Error;
use std::collections::{HashMap, HashSet};
use std::net::IpAddr;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

/// A running container's bridge address that holds at least one credential override.
#[derive(Debug, Clone)]
pub(crate) struct BridgeTarget {
    pub ip_address: IPAddress,
    pub subnet: Subnet,
    /// The ports the container scan bound to the container at this address.
    pub open_ports: Vec<PortType>,
    pub virtualization: ServiceVirtualization,
    pub virtualization_service_id: Option<Uuid>,
}

#[derive(Debug, Default)]
pub(crate) struct BridgeSelection {
    pub targets: Vec<BridgeTarget>,
    /// Every override address on one of this host's bridges, tried or not. A stopped or paused
    /// container's address is here and not in `targets`, which is how it is skipped silently: the
    /// network phase leaves these out of its "not scanned" check.
    pub handled: HashSet<IpAddr>,
}

/// Pick the bridge addresses to try credentials at, from what the container scan put in
/// `host_data`.
pub(crate) fn select_bridge_targets(
    host_data: &HostData,
    mappings: &[CredentialMapping<CredentialQueryPayload>],
) -> BridgeSelection {
    let bridges: Vec<&Subnet> = host_data
        .subnets
        .iter()
        .filter(|s| s.is_container_bridge_subnet())
        .collect();
    let bridge_of = |ip: &IpAddr| bridges.iter().find(|s| s.base.cidr.contains(ip)).copied();

    let override_ips: HashSet<IpAddr> = mappings
        .iter()
        .flat_map(|m| m.ip_overrides.iter().map(|o| o.ip))
        .filter(|ip| bridge_of(ip).is_some())
        .collect();
    if override_ips.is_empty() {
        return BridgeSelection::default();
    }

    let mut targets: Vec<BridgeTarget> = Vec::new();
    for service in &host_data.services {
        let Some(virtualization) = &service.base.virtualization_metadata else {
            continue;
        };
        // A paused or restarting container holds its address and answers on nothing.
        if virtualization.guest_state() != GuestState::Running {
            continue;
        }
        for ip_address_id in service
            .base
            .bindings
            .iter()
            .filter_map(|b| b.ip_address_id())
        {
            let Some(ip_address) = host_data
                .ip_addresses
                .iter()
                .find(|i| i.id == ip_address_id)
            else {
                continue;
            };
            let ip = ip_address.base.ip_address;
            let Some(subnet) = bridge_of(&ip) else {
                continue;
            };
            if !override_ips.contains(&ip)
                || targets.iter().any(|t| t.ip_address.base.ip_address == ip)
            {
                continue;
            }
            let open_ports = service
                .base
                .bindings
                .iter()
                .filter(|b| b.ip_address_id().is_none_or(|id| id == ip_address_id))
                .filter_map(|b| b.port_id())
                .filter_map(|port_id| host_data.ports.iter().find(|p| p.id == port_id))
                .map(|p| p.base.port_type)
                .collect();
            targets.push(BridgeTarget {
                ip_address: ip_address.clone(),
                subnet: subnet.clone(),
                open_ports,
                virtualization: virtualization.clone(),
                virtualization_service_id: service.base.virtualization_service_id,
            });
        }
    }

    BridgeSelection {
        targets,
        handled: override_ips,
    }
}

/// The mappings to dispatch at one bridge address: only the overrides naming it.
///
/// A site default is dropped. It is a guess tried at every address a sweep reaches, and a sweep
/// never reaches a bridge; a credential reaches a container only by being assigned to it.
pub(crate) fn mappings_at(
    mappings: &[CredentialMapping<CredentialQueryPayload>],
    ip: IpAddr,
) -> Vec<CredentialMapping<CredentialQueryPayload>> {
    mappings
        .iter()
        .filter(|m| m.ip_overrides.iter().any(|o| o.ip == ip))
        .map(|m| CredentialMapping {
            default_credential: None,
            default_credential_id: None,
            daemon_os: m.daemon_os,
            ip_overrides: m
                .ip_overrides
                .iter()
                .filter(|o| o.ip == ip)
                .cloned()
                .collect(),
        })
        .collect()
}

impl HostData {
    /// Fold what an integration found at a bridge container into its runtime host.
    ///
    /// Keeps the services (marked as the container's), the ports they bind and the credential
    /// assignments. A service the container scan already holds at this address is not added
    /// twice. A port the runtime host already lists is reused, since a host holds one row per port.
    pub(crate) fn merge_bridge_container_results(
        &mut self,
        found: HostData,
        target: &BridgeTarget,
    ) {
        let mut port_ids: HashMap<Uuid, Uuid> = HashMap::new();
        for port in found.ports {
            match self
                .ports
                .iter()
                .find(|p| p.base.port_type == port.base.port_type)
            {
                Some(existing) => {
                    port_ids.insert(port.id, existing.id);
                }
                None => {
                    port_ids.insert(port.id, port.id);
                    self.ports.push(port);
                }
            }
        }

        let target_id = target.ip_address.id;
        for mut service in found.services {
            let definition = service.base.service_definition.id();
            let already_held = self.services.iter().any(|s| {
                s.base.service_definition.id() == definition
                    && s.base
                        .bindings
                        .iter()
                        .any(|b| b.ip_address_id() == Some(target_id))
            });
            if already_held {
                continue;
            }
            for binding in &mut service.base.bindings {
                if let BindingType::Port { port_id, .. } = &mut binding.base.binding_type
                    && let Some(kept) = port_ids.get(port_id)
                {
                    *port_id = *kept;
                }
            }
            service.base.host_id = self.host.id;
            service.base.virtualization_metadata = Some(target.virtualization.clone());
            service.base.virtualization_service_id = target.virtualization_service_id;
            self.services.push(service);
        }

        self.host
            .base
            .credential_assignments
            .extend(found.host.base.credential_assignments);
    }
}

impl DiscoveryRunner {
    /// Try credentials at each running bridge container in `host_data`, which holds the container
    /// scan's results for this daemon's own host.
    ///
    /// Returns every override address on this host's bridges, tried or not, for the network phase
    /// to leave out of its reachability checks. A container that fails is logged and the rest are
    /// still tried; it never fails the runtime host's submission.
    pub(crate) async fn run_bridge_container_integrations(
        &self,
        ops: &DiscoveryOps,
        host_data: &mut HostData,
        known_subnets: &[Subnet],
        cancel: &CancellationToken,
    ) -> HashSet<IpAddr> {
        let selection = select_bridge_targets(host_data, &self.credential_mappings);
        if !selection.targets.is_empty() {
            tracing::info!(
                containers = selection.targets.len(),
                "Trying credentials at bridge containers"
            );
        }

        for target in &selection.targets {
            if cancel.is_cancelled() {
                break;
            }
            match self
                .collect_bridge_container(ops, target, known_subnets, cancel)
                .await
            {
                Ok(Some(found)) => host_data.merge_bridge_container_results(found, target),
                Ok(None) => {}
                Err(e) => tracing::warn!(
                    ip = %target.ip_address.base.ip_address,
                    container = target.virtualization.container_name().unwrap_or_default(),
                    error = %e,
                    "Credential dispatch at a bridge container failed"
                ),
            }
        }

        selection.handled
    }

    /// Probe and collect one bridge container. `None` when no credential reached it, which the
    /// probe has already reported.
    async fn collect_bridge_container(
        &self,
        ops: &DiscoveryOps,
        target: &BridgeTarget,
        known_subnets: &[Subnet],
        cancel: &CancellationToken,
    ) -> Result<Option<HostData>, Error> {
        let ip = target.ip_address.base.ip_address;
        let mappings = mappings_at(&self.credential_mappings, ip);
        let trusted_ca = ops.config_store.get_trusted_ca().await;

        // The container scan saw only the ports the container declares, so the probe gate would
        // turn away a credential whose port the image never declared. Each integration probes its
        // own port here, and an address that does not answer is reported as unreachable for this
        // container.
        let probe_results = dispatch::probe_integrations(
            ip,
            &mappings,
            &target.open_ports,
            true,
            cancel,
            &self.service.utils,
            ops.config_store.get_accept_invalid_scan_certs().await?,
            trusted_ca.as_deref(),
            &ops.config_store,
        )
        .await?;
        ops.record_credential_issues(&probe_results.credential_issues)
            .await;
        if probe_results.client_responses.is_empty() {
            return Ok(None);
        }

        let virtualization_metadata = Some(target.virtualization.clone());
        let params = ServiceMatchBaselineParams {
            subnet: &target.subnet,
            ip_address: &target.ip_address,
            // Only what the probes answered on. The container scan already matched the
            // container's own ports, and matching them again would add each service twice.
            all_ports: &probe_results.additional_ports,
            endpoint_responses: &vec![],
            virtualization_metadata: &virtualization_metadata,
            virtualization_service_id: target.virtualization_service_id,
            client_responses: &probe_results.client_responses,
            managed_device: &None,
            // A bridge is not the daemon's broadcast domain.
            dns_sd: &None,
        };
        let Some(mut found) = ops
            .build_host_from_scan(
                params,
                None,
                self.host_naming_fallback,
                AttributeSource::PortScan,
            )
            .await?
        else {
            return Ok(None);
        };
        found.host.id = self.host_id;

        let mut open_ports = target.open_ports.clone();
        open_ports.extend(probe_results.additional_ports.iter());
        let execute_params = dispatch::ExecuteParams {
            ip,
            cancel,
            ops,
            utils: &self.service.utils,
            open_ports: &open_ports,
            endpoint_responses: &[],
            host_id: self.host_id,
            host_naming_fallback: self.host_naming_fallback,
            known_subnets,
            scanning_subnet: Some(&target.subnet),
            ip_address_id: Some(target.ip_address.id),
        };
        dispatch::execute_integrations(&mappings, &probe_results, &mut found, &execute_params)
            .await?;

        Ok(Some(found))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::bindings::r#impl::base::Binding;
    use crate::server::credentials::r#impl::mapping::IpOverride;
    use crate::server::credentials::r#impl::types::{
        CredentialAssignment, CredentialTypeDiscriminants,
    };
    use crate::server::hosts::r#impl::base::Host;
    use crate::server::ip_addresses::r#impl::base::IPAddressBase;
    use crate::server::ports::r#impl::base::{Port, PortBase};
    use crate::server::services::definitions::ssh::Ssh;
    use crate::server::services::r#impl::base::Service;
    use crate::server::services::r#impl::virtualization::DockerVirtualization;
    use crate::server::subnets::r#impl::base::{
        SubnetBase, SubnetCidr, SubnetCidrValue, SubnetTypeAttributed,
    };
    use crate::server::subnets::r#impl::types::SubnetType;

    const BRIDGE_CIDR: &str = "172.17.0.0/16";

    fn subnet(cidr: &str, subnet_type: SubnetType) -> Subnet {
        Subnet {
            id: Uuid::new_v4(),
            base: SubnetBase {
                cidr: SubnetCidr::new(
                    SubnetCidrValue(cidr.parse().unwrap()),
                    AttributeSource::DaemonSelfReport,
                ),
                subnet_type: SubnetTypeAttributed::unattributed(subnet_type),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn address(subnet: &Subnet, ip: &str) -> IPAddress {
        IPAddress {
            id: Uuid::new_v4(),
            base: IPAddressBase {
                subnet_id: subnet.id,
                ip_address: ip.parse().unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn container(name: &str, state: GuestState, ip: &IPAddress, port: &Port) -> Service {
        let mut service = Service::default();
        service.id = Uuid::new_v4();
        service.base.virtualization_metadata =
            Some(ServiceVirtualization::Docker(DockerVirtualization {
                container_name: Some(name.to_string()),
                container_id: None,
                compose_project: None,
                state,
            }));
        service.base.bindings = vec![Binding::new_serviceless(BindingType::Port {
            port_id: port.id,
            ip_address_id: Some(ip.id),
        })];
        service
    }

    fn ssh_override(ip: &str) -> CredentialMapping<CredentialQueryPayload> {
        let payload = CredentialTypeDiscriminants::SshPassword
            .to_credential_type()
            .to_query_payload();
        CredentialMapping {
            default_credential: Some(payload.clone()),
            default_credential_id: None,
            daemon_os: Default::default(),
            ip_overrides: vec![IpOverride {
                ip: ip.parse().unwrap(),
                credential: payload,
                credential_id: Uuid::new_v4(),
                host_id: None,
                mac_address: None,
            }],
        }
    }

    struct Scene {
        host_data: HostData,
        running_ip: IPAddress,
    }

    /// A runtime host with a LAN address and two bridge containers, each bound to port 8080:
    /// `web` running at .3 and `db` in `paused_state` at .4.
    fn scene(paused_state: GuestState) -> Scene {
        let lan = subnet("10.0.20.0/24", SubnetType::Lan);
        let bridge = subnet(BRIDGE_CIDR, SubnetType::DockerBridge);
        let lan_ip = address(&lan, "10.0.20.20");
        let running_ip = address(&bridge, "172.17.0.3");
        let paused_ip = address(&bridge, "172.17.0.4");
        let port = Port::new(PortBase::new_hostless(PortType::new_tcp(8080)));
        let services = vec![
            container("web", GuestState::Running, &running_ip, &port),
            container("db", paused_state, &paused_ip, &port),
        ];
        Scene {
            host_data: HostData::new(
                Host::default(),
                services,
                vec![port],
                vec![lan_ip, running_ip.clone(), paused_ip],
                vec![],
                vec![lan, bridge],
            ),
            running_ip,
        }
    }

    fn target_ips(selection: &BridgeSelection) -> Vec<IpAddr> {
        selection
            .targets
            .iter()
            .map(|t| t.ip_address.base.ip_address)
            .collect()
    }

    #[test]
    fn only_running_containers_with_an_override_are_targets() {
        let s = scene(GuestState::Paused);
        let mappings = vec![ssh_override("172.17.0.3"), ssh_override("172.17.0.4")];
        let selection = select_bridge_targets(&s.host_data, &mappings);
        assert_eq!(target_ips(&selection), vec![s.running_ip.base.ip_address]);
        assert_eq!(
            selection.targets[0].open_ports,
            vec![PortType::new_tcp(8080)]
        );
        // The paused container's override is handled (kept out of "not scanned") without a try.
        assert!(selection.handled.contains(&"172.17.0.4".parse().unwrap()));
    }

    #[test]
    fn a_restarting_container_is_not_a_target() {
        let s = scene(GuestState::Restarting);
        let selection = select_bridge_targets(&s.host_data, &[ssh_override("172.17.0.4")]);
        assert!(selection.targets.is_empty());
        assert_eq!(selection.handled.len(), 1);
    }

    /// A stopped container has no address in the scan at all; its override is still on a bridge
    /// this host owns, so it is handled silently rather than reported as not scanned.
    #[test]
    fn an_override_at_an_address_no_container_holds_is_handled_silently() {
        let s = scene(GuestState::Paused);
        let selection = select_bridge_targets(&s.host_data, &[ssh_override("172.17.0.9")]);
        assert!(selection.targets.is_empty());
        assert!(selection.handled.contains(&"172.17.0.9".parse().unwrap()));
    }

    #[test]
    fn overrides_off_the_bridges_are_left_to_the_network_phase() {
        let s = scene(GuestState::Paused);
        let selection = select_bridge_targets(&s.host_data, &[ssh_override("10.0.20.20")]);
        assert!(selection.targets.is_empty());
        assert!(selection.handled.is_empty());
    }

    #[test]
    fn mappings_at_keeps_only_that_addresss_overrides_and_no_default() {
        let mut mapping = ssh_override("172.17.0.3");
        mapping.ip_overrides.push(IpOverride {
            ip: "10.0.20.20".parse().unwrap(),
            ..mapping.ip_overrides[0].clone()
        });
        let at = mappings_at(
            &[mapping, ssh_override("10.0.20.30")],
            "172.17.0.3".parse().unwrap(),
        );
        assert_eq!(at.len(), 1);
        assert!(at[0].default_credential.is_none());
        assert_eq!(
            at[0].ip_overrides.iter().map(|o| o.ip).collect::<Vec<_>>(),
            vec!["172.17.0.3".parse::<IpAddr>().unwrap()]
        );
    }

    #[test]
    fn merge_keeps_services_and_assignments_and_drops_host_facts() {
        let mut s = scene(GuestState::Paused);
        let runtime_host_id = Uuid::new_v4();
        s.host_data.host.id = runtime_host_id;
        let target = select_bridge_targets(&s.host_data, &[ssh_override("172.17.0.3")])
            .targets
            .remove(0);

        // What an SSH run at the container returns: an SSH service on a new port 22, the
        // container's own hostname and an extra address, and the assignment it earned.
        let ssh_port = Port::new(PortBase::new_hostless(PortType::new_tcp(22)));
        let mut ssh = Service::default();
        ssh.id = Uuid::new_v4();
        ssh.base.service_definition = Box::new(Ssh);
        ssh.base.bindings = vec![Binding::new_serviceless(BindingType::Port {
            port_id: ssh_port.id,
            ip_address_id: Some(s.running_ip.id),
        })];
        let credential_id = Uuid::new_v4();
        let mut found_host = Host::default();
        found_host.base.credential_assignments = vec![CredentialAssignment {
            credential_id,
            ip_address_ids: Some(vec![s.running_ip.id]),
        }];
        let mut found = HostData::new(
            found_host,
            vec![ssh],
            vec![ssh_port],
            vec![s.running_ip.clone(), address(&target.subnet, "172.17.0.50")],
            vec![],
            vec![],
        );
        found.with_hostname("web-container".into(), AttributeSource::PortScan);

        let ips_before = s.host_data.ip_addresses.len();
        let hostname_before = s.host_data.host.base.hostname.clone();
        s.host_data.merge_bridge_container_results(found, &target);

        let merged = s.host_data.services.last().unwrap();
        assert_eq!(merged.base.host_id, runtime_host_id);
        assert_eq!(
            merged
                .base
                .virtualization_metadata
                .as_ref()
                .and_then(|v| v.container_name()),
            Some("web")
        );
        assert!(
            s.host_data
                .ports
                .iter()
                .any(|p| p.base.port_type == PortType::new_tcp(22))
        );
        assert_eq!(
            s.host_data.host.base.credential_assignments[0].credential_id,
            credential_id
        );
        assert_eq!(s.host_data.ip_addresses.len(), ips_before);
        assert_eq!(s.host_data.host.base.hostname, hostname_before);
    }

    #[test]
    fn merge_reuses_a_port_the_host_already_lists() {
        let mut s = scene(GuestState::Paused);
        let target = select_bridge_targets(&s.host_data, &[ssh_override("172.17.0.3")])
            .targets
            .remove(0);
        let existing_port_id = s.host_data.ports[0].id;
        let duplicate = Port::new(PortBase::new_hostless(PortType::new_tcp(8080)));
        let mut service = Service::default();
        service.id = Uuid::new_v4();
        service.base.service_definition = Box::new(Ssh);
        service.base.bindings = vec![Binding::new_serviceless(BindingType::Port {
            port_id: duplicate.id,
            ip_address_id: Some(s.running_ip.id),
        })];
        let found = HostData::new(
            Host::default(),
            vec![service],
            vec![duplicate],
            vec![],
            vec![],
            vec![],
        );

        let ports_before = s.host_data.ports.len();
        s.host_data.merge_bridge_container_results(found, &target);

        assert_eq!(s.host_data.ports.len(), ports_before);
        let merged = s.host_data.services.last().unwrap();
        assert_eq!(merged.base.bindings[0].port_id(), Some(existing_port_id));
    }
}
