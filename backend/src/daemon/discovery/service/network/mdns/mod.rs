//! mDNS / DNS-SD browsing.
//!
//! ## How this differs from the reverse-DNS lookup already in the scan
//!
//! [`super::dns::NetworkScan::get_hostname_for_ip`] resolves a name through `getnameinfo` — a
//! **unicast PTR query to whatever resolver the daemon's host is configured with**. It answers
//! only when some server holds a `PTR` record for the address: a router that registers its DHCP
//! leases, an internal zone somebody maintains, or public reverse DNS. Nothing anywhere holds a
//! PTR record for a Chromecast.
//!
//! mDNS is a **multicast query to `224.0.0.251:5353` that the device itself answers**. No server,
//! no zone, no registration. RFC 6762 makes `.local` explicitly not a unicast-DNS zone, and the
//! container has no `nss-mdns` module, so the existing lookup cannot reach this even in principle
//! (GH #587).
//!
//! Beyond names, DNS-SD advertises *services* — `_googlecast._tcp`, `_airplay._tcp`, `_hap._tcp` —
//! with TXT records carrying model and vendor. That is match evidence for devices with no
//! scannable TCP port at all, which is a larger prize than the naming the issue asked for.
//!
//! ## What bounds it
//!
//! **mDNS does not cross a router.** `224.0.0.251` is link-local with TTL 1, so a daemon only ever
//! sees its own broadcast domains. This enriches the subnets a daemon sits on and contributes
//! nothing to a routed subnet it scans remotely. Multicast also requires the container to be on
//! host networking; default bridge networking does not forward link-local multicast, the same
//! constraint ARP already carries.

pub mod types;
pub mod wire;

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};
use std::time::Duration;

use anyhow::{Error, Result};
use hickory_resolver::proto::rr::Name;
use socket2::{Domain, Protocol, Socket, Type};
use tokio::net::UdpSocket;

pub use types::DnsSdHost;
use wire::{Accumulator, SERVICE_ENUMERATION};

use crate::server::services::definitions::ServiceDefinitionRegistry;

/// The mDNS group and port (RFC 6762).
const MDNS_GROUP: Ipv4Addr = Ipv4Addr::new(224, 0, 0, 251);
const MDNS_PORT: u16 = 5353;

/// How long to listen after each burst. Responders answer within milliseconds, but stagger their
/// replies deliberately to avoid colliding on the group.
const COLLECT_WINDOW: Duration = Duration::from_millis(1500);

/// Receive buffer. mDNS messages are supposed to fit a link MTU; this is comfortably above it.
const RECV_BUFFER: usize = 9000;

/// Service types asked for in the first burst.
///
/// The enumeration query discovers what a link actually offers, and a second burst asks about
/// whatever it named that is not already here — so this list is a latency optimisation, not the
/// limit of what a browse can find. These are the types worth having an answer for in the first
/// round trip because they identify device populations the port scan cannot see at all.
const WELL_KNOWN_SERVICE_TYPES: [&str; 12] = [
    "_googlecast._tcp.local.",
    "_airplay._tcp.local.",
    "_raop._tcp.local.",
    "_companion-link._tcp.local.",
    "_device-info._tcp.local.",
    "_hap._tcp.local.",
    "_ipp._tcp.local.",
    "_pdl-datastream._tcp.local.",
    "_printer._tcp.local.",
    "_sonos._tcp.local.",
    "_hue._tcp.local.",
    "_home-assistant._tcp.local.",
];

/// Browse every broadcast domain in `interface_addresses`, returning what answered, by address.
///
/// One multicast burst per interface per round rather than anything per host, so the cost is
/// independent of how large the subnets being scanned are.
///
/// Failure to browse one interface is logged and skipped: a daemon with several NICs should still
/// get the segments it can reach, and the whole feature is an enrichment that discovery must run
/// without.
pub async fn browse(interface_addresses: &[Ipv4Addr]) -> HashMap<IpAddr, DnsSdHost> {
    let mut accumulator = Accumulator::default();

    let well_known = first_burst_names();

    for address in interface_addresses {
        if let Err(e) = browse_interface(*address, &well_known, &mut accumulator).await {
            tracing::debug!(interface = %address, error = %e, "mDNS browse failed on interface");
        }
    }

    // A second burst for types the link advertised that we did not think to ask about. This is
    // what lets a browse identify a device nobody wrote a service type for in advance.
    let follow_up = accumulator.unasked_types(&well_known);
    if !follow_up.is_empty() {
        tracing::debug!(
            types = follow_up.len(),
            "Asking about mDNS service types the link advertised"
        );
        for address in interface_addresses {
            if let Err(e) = browse_interface(*address, &follow_up, &mut accumulator).await {
                tracing::debug!(interface = %address, error = %e, "mDNS follow-up failed");
            }
        }
    }

    let hosts = accumulator.resolve();
    for (address, host) in &hosts {
        tracing::debug!(
            %address,
            services = ?host.services.keys().collect::<Vec<_>>(),
            "mDNS services the host advertised"
        );
    }
    tracing::info!(
        hosts = hosts.len(),
        interfaces = interface_addresses.len(),
        "mDNS browse complete"
    );
    hosts
}

/// When, after the first, the query goes out again within the same window.
///
/// Responders drop queries: a Bose amp answered a lone `_spotify-connect._tcp` query 9 times in 10
/// and every query repeated half a second later, and the browse missed its Spotify Connect
/// advertisement in two of three live scans. RFC 6762 §5.2 expects a querier to repeat itself for
/// the same reason.
const REQUERY_AFTER: Duration = Duration::from_millis(500);

/// The first burst: the well-known types, every type a service definition matches on, and the
/// enumeration query.
///
/// A type only the follow-up burst asks about depends on some device's enumeration reply naming
/// it. A Bose amp's `_spotify-connect._tcp` went unasked that way in one of four live scans, so its
/// Spotify Connect service and identity were lost for that scan. Every type a definition reads is
/// asked for directly instead.
fn first_burst_names() -> Vec<Name> {
    let definition_types: Vec<String> = ServiceDefinitionRegistry::all_service_definitions()
        .iter()
        .flat_map(|definition| {
            definition
                .discovery_pattern()
                .dns_sd_types()
                .into_iter()
                .map(|service_type| format!("{}.local.", service_type.to_ascii_lowercase()))
                .collect::<Vec<_>>()
        })
        .collect();
    let mut names: Vec<String> = WELL_KNOWN_SERVICE_TYPES
        .iter()
        .map(|name| name.to_string())
        .chain(definition_types)
        .chain(std::iter::once(SERVICE_ENUMERATION.to_string()))
        .collect();
    names.sort();
    names.dedup();
    names
        .iter()
        .filter_map(|name| Name::from_ascii(name).ok())
        .collect()
}

/// Send a query out `interface_address`, again after [`REQUERY_AFTER`], and fold everything that
/// answers within the window into `accumulator`.
async fn browse_interface(
    interface_address: Ipv4Addr,
    names: &[Name],
    accumulator: &mut Accumulator,
) -> Result<()> {
    if names.is_empty() {
        return Ok(());
    }

    let socket = open_socket(interface_address)?;
    let query = wire::build_query(names)?;
    let group = SocketAddr::V4(SocketAddrV4::new(MDNS_GROUP, MDNS_PORT));
    socket.send_to(&query, group).await?;

    let start = tokio::time::Instant::now();
    let deadline = start + COLLECT_WINDOW;
    let mut requery_at = Some(start + REQUERY_AFTER);
    let mut buffer = vec![0u8; RECV_BUFFER];
    loop {
        let now = tokio::time::Instant::now();
        if let Some(at) = requery_at
            && now >= at
        {
            requery_at = None;
            if let Err(e) = socket.send_to(&query, group).await {
                tracing::trace!(error = %e, "mDNS re-query failed");
            }
        }
        // Wake for the re-query as well as for the end of the window.
        let wake = requery_at.map_or(deadline, |at| at.min(deadline));
        let remaining = wake.saturating_duration_since(now);
        if deadline.saturating_duration_since(now).is_zero() {
            break;
        }
        match tokio::time::timeout(remaining, socket.recv_from(&mut buffer)).await {
            Ok(Ok((len, _from))) => accumulator.absorb(&buffer[..len]),
            // The re-query is due or the window elapsed; the top of the loop tells which.
            Err(_) => continue,
            // A receive error on a multicast socket is per-packet, not fatal; keep listening
            // until the window closes rather than abandoning the whole interface.
            Ok(Err(e)) => {
                tracing::trace!(error = %e, "mDNS receive error");
            }
        }
    }

    Ok(())
}

/// A socket joined to the mDNS group and bound to send from `interface_address`.
///
/// `set_multicast_if_v4` is the part that matters and the reason `socket2` is used here: it
/// decides which interface the query actually leaves by. Without it a multi-homed daemon browses
/// whichever segment the routing table happens to prefer, and silently misses the rest.
///
/// Address and port reuse are set because 5353 is a well-known port that a system mDNS responder
/// (avahi, mDNSResponder) is very likely already bound to — without them, binding fails on any
/// host running one, which is most of them.
fn open_socket(interface_address: Ipv4Addr) -> Result<UdpSocket> {
    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;
    socket.set_reuse_address(true)?;
    #[cfg(not(target_family = "windows"))]
    socket.set_reuse_port(true)?;
    socket.set_nonblocking(true)?;

    socket.bind(&SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, MDNS_PORT).into())?;
    socket.set_multicast_if_v4(&interface_address)?;
    // TTL 1 keeps the query on the link, which is the whole contract of mDNS — a query that
    // escaped onto a routed segment would be both useless and rude.
    socket.set_multicast_ttl_v4(1)?;
    socket.join_multicast_v4(&MDNS_GROUP, &interface_address)?;

    UdpSocket::from_std(socket.into()).map_err(Error::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every service type a definition matches on is asked for in the first burst, so none waits
    /// on another device's enumeration reply.
    #[test]
    fn the_first_burst_asks_for_every_type_a_definition_reads() {
        let asked: Vec<String> = first_burst_names()
            .iter()
            .map(|name| name.to_ascii().to_ascii_lowercase())
            .collect();
        for definition in ServiceDefinitionRegistry::all_service_definitions() {
            for service_type in definition.discovery_pattern().dns_sd_types() {
                let wanted = format!("{}.local.", service_type.to_ascii_lowercase());
                assert!(
                    asked.contains(&wanted),
                    "{} reads {service_type}, which the first burst does not ask for",
                    definition.name()
                );
            }
        }
        assert!(asked.contains(&SERVICE_ENUMERATION.to_ascii_lowercase()));
    }
}
