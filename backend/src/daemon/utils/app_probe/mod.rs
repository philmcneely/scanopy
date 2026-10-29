//! The application-probe stage: unicast an address on a known port, speak the protocol, and
//! report what answered.
//!
//! Before this module a non-credentialed probe was reached through a hardcoded `match port` inside
//! `scan_udp_ports`, every probe returned `Result<Option<u16>, Error>` — a port number — and the
//! response body was discarded. BACnet read an I-Am carrying a vendor ID and device instance,
//! checked that byte 0 was `0x81`, and dropped it. Adding a probe meant editing that function, and
//! DHCP's "only on the gateway" rule lived inside one of its arms as an `if is_gateway`.
//!
//! **A probe is a method on the [`ServiceDefinition`] it belongs to.** That gives one registry
//! rather than two and makes the invariants that matter structural: a probe cannot exist without a
//! service definition, because a definition is the only thing that can hand one out; and the port
//! that gets probed is the port that gets scanned, because both are read from
//! [`ClientProbe::ports`]: the runner probes those ports, and a definition's
//! [`Pattern::ClientResponse`](crate::server::services::r#impl::patterns::Pattern::ClientResponse)
//! puts them in its scan. What stays outside is HTTP endpoint
//! probing, which is already declarative, and the credentialed integrations, which have their own
//! registry keyed on credential type — a split that means something, unlike the one this replaces.
//!
//! The probe owns its transport, because they differ: DNS and NTP use library clients (hickory,
//! sntpc), BACnet and EtherNet/IP bind a UDP socket, and Modbus opens a TCP stream and keeps it for
//! a second exchange.

pub mod amqp;
pub mod apple_lockdown;
pub mod bacula;
pub mod beszel_agent;
pub mod cassandra;
pub mod checkmk;
pub mod dns_tcp;
pub mod docker_ping;
pub mod docker_swarm;
pub mod epics_ca;
pub mod ethernet_ip;
pub mod ftp;
pub mod google_wifi;
pub mod h323;
pub mod ike;
pub mod ipp;
pub mod kafka;
pub mod kerberos;
pub mod ldap;
#[cfg(test)]
mod live_servers;
pub mod llmnr;
pub mod lpd;
pub mod mgcp;
#[cfg(test)]
mod middlebox;
pub mod modbus;
pub mod mongodb;
pub mod mqtt;
pub mod mssql;
pub mod mysql;
pub mod nfs;
pub mod nut;
pub mod opcua;
pub mod openvpn;
pub mod oracle;
pub mod postgresql;
pub mod rdp;
pub mod redis;
pub mod rpcbind;
pub mod rtsp;
pub mod sip;
pub mod smb;
pub mod smux;
pub mod snmp_engine;
pub mod spotify_connect;
pub mod ssh;
pub mod telnet;
pub mod tftp;
pub mod tls;
pub mod udp;
pub mod unbound_control;
pub mod vnc;
pub mod zabbix;
pub mod zmtp;

use anyhow::Error;
use async_trait::async_trait;
use cidr::IpCidr;
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

use crate::daemon::discovery::service::ops::HostData;
use crate::daemon::discovery::types::base::DiscoveryCriticalError;
use crate::daemon::utils::scanner::{SCAN_TIMEOUT, ScanConcurrencyController, batch_scan};
use crate::server::hosts::r#impl::attributes::{
    HostFirmwareRevisionAttributed, HostManufacturerAttributed, HostModelAttributed,
    HostOsAttributed, HostSerialNumberAttributed,
};
use crate::server::hosts::r#impl::os::recog::{RecogDatabase, RecogMatch};
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::definitions::ServiceDefinitionRegistry;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::ClientProbe;
use crate::server::shared::attribution::{AttributeSource, AttributeValue, Attributed};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::TcpStream;

/// Everything a probe is allowed to know about its target.
///
/// Owned rather than borrowed: [`batch_scan`] requires the futures it drives to be `'static`, so a
/// context holding references could not be captured into one.
#[derive(Debug, Clone)]
pub struct ProbeContext {
    pub ip: IpAddr,
    /// The subnet the address was found on. DHCP needs it to compute a broadcast address.
    pub subnet_cidr: IpCidr,
    /// Whether this address is in the daemon's routing table.
    pub is_gateway: bool,
    pub cancel: CancellationToken,
    /// Reports FD exhaustion so the scan degrades instead of failing.
    ///
    /// `scan_endpoints` does not feed this and does not need to — reqwest pools its connections.
    /// A raw-byte probe opens its own socket per target, so it is in the same position as
    /// `scan_tcp_ports`, which is the one stage that does feed it today.
    pub scan_controller: Arc<ScanConcurrencyController>,
    /// The scan's `probe_raw_socket_ports`. Off, no probe on 9100-9107 runs: a raw print port
    /// prints whatever it receives, and Bacula's greeting on 9101 came out of a Kyocera (GH #801).
    pub probe_raw_socket_ports: bool,
    /// Where to reach a probe's port when it isn't the port itself: a container's port published
    /// on another host port. Empty for a network scan, which probes each port where it is.
    pub port_map: HashMap<u16, u16>,
    /// Opens the probe's connections instead of a TCP connect to [`Self::target`]: a relay into a
    /// container's own network namespace, for a port the daemon has no route to. `None` for a
    /// network scan.
    pub tunnel: Option<Arc<dyn ProbeTunnel>>,
}

/// A connection a TCP probe talks over: a socket, or a relay standing in for one.
pub trait ProbeStream: AsyncRead + AsyncWrite + Unpin + Send {}

impl<T: AsyncRead + AsyncWrite + Unpin + Send> ProbeStream for T {}

/// Reaches a target's ports some other way than connecting to its address.
///
/// A container's unpublished port lives on a bridge the daemon often cannot route to: Docker
/// Desktop's sits inside its VM, rootless Podman's inside a user namespace. The container itself can
/// always reach its own ports on loopback, so a relay run inside it carries the probe's bytes there.
#[async_trait]
pub trait ProbeTunnel: Send + Sync + std::fmt::Debug {
    /// A connection to `port` on the far side. Opening one succeeding says nothing about whether
    /// anything is listening: a relay that cannot connect closes its end, which the probe reads as
    /// silence, the same as a socket that is accepted and then closed.
    async fn open(&self, port: u16) -> std::io::Result<Box<dyn ProbeStream>>;
}

impl ProbeContext {
    /// The address a probe for `port` connects to.
    pub fn target(&self, port: u16) -> SocketAddr {
        SocketAddr::new(self.ip, *self.port_map.get(&port).unwrap_or(&port))
    }

    /// Open a connection to `port` through the tunnel when there is one, or to [`Self::target`].
    ///
    /// `None` covers every way that fails, after reporting a connect error to the scan controller.
    /// The tunnel bounds its own setup, which takes longer than a TCP connect.
    // The one place a probe's socket is opened; everything else comes through here.
    #[allow(clippy::disallowed_methods)]
    pub async fn connect(&self, port: u16) -> Option<Box<dyn ProbeStream>> {
        let opened = match &self.tunnel {
            Some(tunnel) => tunnel.open(port).await,
            None => match tokio::time::timeout(SCAN_TIMEOUT, TcpStream::connect(self.target(port)))
                .await
            {
                Ok(connected) => connected.map(|stream| Box::new(stream) as Box<dyn ProbeStream>),
                Err(_) => return None,
            },
        };
        match opened {
            Ok(stream) => Some(stream),
            Err(e) => {
                self.note_connect_error(&e);
                None
            }
        }
    }

    /// Report a connect failure, so an FD-exhaustion error degrades the whole scan's concurrency
    /// rather than being read as "nothing is listening there".
    pub fn note_connect_error(&self, error: &std::io::Error) {
        self.scan_controller.check_and_handle_error(error);
    }
}

/// What a probe established, beyond "it did not work".
///
/// Two variants rather than more because a probe has exactly two things to say that the caller can
/// act on. A transport failure is an `Err` from [`AppProbe::run`] instead, so the stage can keep
/// applying the same critical-error classification the old dispatch did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppProbeOutcome {
    /// The protocol answered on this port.
    ///
    /// `identity` is `None` both for protocols that carry no identity at all (DNS, NTP, DHCP,
    /// BACnet, OPC UA) and for a device that answered detection but declined identification — a
    /// Modbus device without `0x2B` support. Those two cases need no distinction downstream: the
    /// service matches on the probe having answered, and the host simply has no `model`.
    Answered { identity: Option<DeviceIdentity> },
    /// Nothing answered, or what answered was not this protocol.
    NoAnswer,
}

/// The hardware identity a probe read off a device.
///
/// No `Default`, following [`ControllerIdentity`](crate::daemon::discovery::integration::controller::ControllerIdentity):
/// a new probe cannot `..Default::default()` past a field, so "we forgot to record the model" is a
/// compile error rather than a silently empty column.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceIdentity {
    /// Not a plain string, because the four fields do not all come from the same place. A probe
    /// that reads a vendor *name* off the device and one that synthesises it from a numeric ID are
    /// making different claims, and only the value's own source can say which — see
    /// `ethernet_ip.rs`, whose `manufacturer` is our construction rather than the device's word.
    pub manufacturer: Option<HostManufacturerAttributed>,
    pub model: Option<HostModelAttributed>,
    pub serial_number: Option<HostSerialNumberAttributed>,
    /// Firmware or product revision, as the device words it — Modbus MajorMinorRevision,
    /// EtherNet/IP `<major>.<minor>`.
    pub firmware_revision: Option<HostFirmwareRevisionAttributed>,
    /// The operating system, when the protocol names it: the distribution an SSH banner carries,
    /// or the Windows build in an SMB challenge.
    /// Boxed: an OS reading is several times the size of the other fields, and inline it makes
    /// every enum carrying an identity (`AppProbeOutcome`, `ModbusReply`) needlessly large.
    pub os: Option<Box<HostOsAttributed>>,
    /// What a Recog fingerprint of a string the device sent names (an SSH, FTP or Telnet banner,
    /// an RTSP `Server` header): OS, maker, model and device class, all inferred, so they combine
    /// with every other match through [`HostData::offer_match`] rather than apply by rank.
    /// Boxed for the reason `os` is.
    pub matched: Option<Box<(RecogMatch, AttributeSource)>>,
}

impl DeviceIdentity {
    /// The identity a fingerprint of `input` names, if it names anything.
    pub fn matched(
        database: RecogDatabase,
        input: &str,
        source: AttributeSource,
    ) -> Option<DeviceIdentity> {
        // The software alone (`OpenSSH`, `ProFTPD`) says nothing about the host: the protocol's
        // own definition already names the service.
        let matched = database.identify(input).filter(RecogMatch::names_host)?;
        Some(DeviceIdentity {
            manufacturer: None,
            model: None,
            serial_number: None,
            firmware_revision: None,
            os: None,
            matched: Some(Box::new((matched, source))),
        })
    }

    /// Fold this identity into the host currently being scanned.
    ///
    /// Each value carries the source that produced it, so the ordering against a credentialed
    /// read is stated rather than resting on the app-probe stage running last. An industrial
    /// protocol is the device's own, which outranks a generic MIB — so a Modbus vendor name does
    /// displace SNMP's, deliberately. What does not is a manufacturer we synthesised from a vendor
    /// ID: that is an inference, and it says so.
    pub fn enrich(&self, host_data: &mut HostData) {
        let Self {
            manufacturer,
            model,
            serial_number,
            firmware_revision,
            os,
            matched,
        } = self.clone();

        if let Some(manufacturer) = manufacturer {
            Attributed::apply(&mut host_data.host.base.manufacturer, manufacturer);
        }
        if let Some(model) = model {
            Attributed::apply(&mut host_data.host.base.model, model);
        }
        if let Some(serial_number) = serial_number {
            Attributed::apply(&mut host_data.host.base.serial_number, serial_number);
        }
        if let Some(firmware_revision) = firmware_revision {
            Attributed::apply(
                &mut host_data.host.base.firmware_revision,
                firmware_revision,
            );
        }
        if let Some(os) = os {
            host_data.offer_os(*os);
        }
        if let Some(matched) = matched {
            let (matched, source) = *matched;
            host_data.offer_match(matched, source);
        }
    }
}

/// Wrap one identity string with the source that produced it, dropping it if the device sent
/// nothing usable.
///
/// Devices routinely pad a fixed-width identity field with spaces or return it empty, and an empty
/// string is absence rather than a value. Trimming here rather than at each parse site keeps the
/// two probes from disagreeing about it; the applier then refuses whatever is still blank.
pub fn identity_field<T: AttributeValue + From<String>>(
    value: Option<String>,
    source: AttributeSource,
) -> Option<Attributed<T>> {
    let value = value?.trim().to_string();
    let carrier = Attributed::new(T::from(value), source);
    (!carrier.is_blank()).then_some(carrier)
}

/// A non-credentialed application probe.
#[async_trait]
pub trait AppProbe: Send + Sync {
    /// The evidence a successful probe contributes to service matching, which a definition reads
    /// with [`Pattern::ClientResponse`](crate::server::services::r#impl::patterns::Pattern::ClientResponse).
    /// Its [`ClientProbe::ports`] are the ports the probe runs on: [`probe_runs`] pairs the probe
    /// with each, and `run` receives the one it is speaking on.
    fn client_probe(&self) -> ClientProbe;

    /// Whether this probe applies to this target at all. DHCP overrides it.
    fn applies(&self, _ctx: &ProbeContext) -> bool {
        true
    }

    async fn run(&self, ctx: &ProbeContext, port: PortType) -> Result<AppProbeOutcome, Error>;
}

/// What one answered probe contributes back to the scan.
#[derive(Debug, Clone)]
pub struct AppProbeResult {
    pub port: PortType,
    pub client_probe: Option<ClientProbe>,
    pub identity: Option<DeviceIdentity>,
}

/// Connect to `port` on the context's address and read whatever the peer sends first.
///
/// For the protocols where the *server* opens the conversation: SSH sends its version string, FTP
/// and SMTP send a `220`, a Check_MK agent dumps its section list. `None` covers every way that can
/// fail to produce evidence — no connect, no bytes, a closed connection — because a probe only ever
/// distinguishes "the protocol answered" from "it did not".
///
/// A connect error is reported to the scan controller before being swallowed, so FD exhaustion
/// degrades the scan's concurrency rather than being read as "nothing is listening there".
pub(crate) async fn read_greeting(ctx: &ProbeContext, port: PortType, buf_len: usize) -> Vec<u8> {
    let Some(mut stream) = ctx.connect(port.number()).await else {
        return Vec::new();
    };

    // A listener that accepts and then says nothing is the middlebox shape, and it is why this
    // returns empty rather than erroring: silence is a legitimate answer meaning "not this".
    read_reply(&mut stream, buf_len, SCAN_TIMEOUT).await
}

/// Connect to `port`, send `request`, and read the reply.
///
/// For the protocols where the *client* opens the conversation: RTSP `OPTIONS`, a Redis `PING`, an
/// LDAP search, an SMB negotiate. Same `None`-shaped contract as [`read_greeting`].
pub(crate) async fn request_response(
    ctx: &ProbeContext,
    port: PortType,
    request: &[u8],
    buf_len: usize,
) -> Vec<u8> {
    let Some(mut stream) = ctx.connect(port.number()).await else {
        return Vec::new();
    };

    if tokio::time::timeout(SCAN_TIMEOUT, stream.write_all(request))
        .await
        .is_err()
    {
        return Vec::new();
    }

    // The connect and write bind to the tight `SCAN_TIMEOUT` (network-bound), but the peer has to
    // compute the reply, so the first response byte is allowed `RESPONSE_READ_TIMEOUT`. A Google
    // Nest root's `/api/v1/status` takes ~1.15s to answer where the leaf takes ~0.12s; under the
    // old 800ms read the root's reply was lost and its identity never collected.
    let reply = read_reply(&mut stream, buf_len, RESPONSE_READ_TIMEOUT).await;
    if reply.is_empty() {
        // Connect and write succeeded (a connect failure returned above), so this is a port that
        // accepted us and then sent no first byte within the window: a slow or silent endpoint,
        // not an absent service. Logged so slow endpoints are visible rather than silently read as
        // "nothing here" — grep this to find probes whose `RESPONSE_READ_TIMEOUT` is too tight.
        tracing::debug!(
            ip = %ctx.ip,
            port = %port.number(),
            timeout_ms = RESPONSE_READ_TIMEOUT.as_millis(),
            "request_response: connected and sent request but received no reply in time"
        );
    }
    reply
}

/// Read what the peer sends, up to `buf_len` bytes: the first segment within `first_read_timeout`,
/// then any further segments that follow within [`REPLY_CONTINUATION`] of each other.
///
/// `first_read_timeout` is how long the peer may take to send its first byte. For a greeting it is
/// the tight [`SCAN_TIMEOUT`] (a banner is pushed immediately or not at all); for a request the
/// peer has to compute the reply, so [`request_response`] allows the longer [`RESPONSE_READ_TIMEOUT`].
///
/// TCP carries a stream, not messages, and one `read` returns whatever segment arrived first. A
/// Google Nest router's DNS server writes its 2-byte length prefix and the message as two
/// segments, and a Brother print server writes a screen clear and then its Telnet negotiation, so
/// a single read handed DnsTcp the prefix alone and Telnet the escape codes alone.
async fn read_reply(
    stream: &mut (impl AsyncRead + Unpin),
    buf_len: usize,
    first_read_timeout: std::time::Duration,
) -> Vec<u8> {
    let mut buf = vec![0u8; buf_len];
    let mut filled = match tokio::time::timeout(first_read_timeout, stream.read(&mut buf)).await {
        Ok(Ok(read)) => read,
        _ => return Vec::new(),
    };
    while filled > 0 && filled < buf_len {
        match tokio::time::timeout(REPLY_CONTINUATION, stream.read(&mut buf[filled..])).await {
            Ok(Ok(read)) if read > 0 => filled += read,
            _ => break,
        }
    }
    buf.truncate(filled);
    buf
}

/// How long [`read_reply`] waits for the next segment of a reply that has started arriving.
const REPLY_CONTINUATION: std::time::Duration = std::time::Duration::from_millis(100);

/// How long [`request_response`] waits for the first byte of a reply it asked for. Longer than
/// [`SCAN_TIMEOUT`] because that bounds the network (connect, write), while this bounds the peer's
/// time to *compute* the reply, which varies by device: an embedded web server that aggregates
/// state can take over a second to answer where the connect took milliseconds. Only ports that
/// already passed the connect scan reach here, so the cost of the longer wait falls on open ports
/// that are slow or silent, not on every probed address.
pub(crate) const RESPONSE_READ_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(2);

/// Connect to `port`, send `request`, and read until `want` bytes have arrived.
///
/// Sibling of [`request_response`], for a reply of known fixed size that a peer is entitled to send
/// in pieces. ZMTP is the case that needs it: RFC 23 lets a peer write the 10-octet signature, wait
/// to read ours, and only then send the remaining 54 — so a single `read` sees a tenth of the
/// greeting and a probe built on it reports "not this protocol" against a real server.
///
/// Returns what arrived when the peer stops early or the timeout expires, which the caller checks
/// against the length it needs. The whole read is bounded by one [`SCAN_TIMEOUT`], not one per
/// segment, so a peer that dribbles bytes cannot hold the scan open.
pub(crate) async fn request_exact(
    ctx: &ProbeContext,
    port: PortType,
    request: &[u8],
    want: usize,
) -> Vec<u8> {
    let Some(mut stream) = ctx.connect(port.number()).await else {
        return Vec::new();
    };

    if tokio::time::timeout(SCAN_TIMEOUT, stream.write_all(request))
        .await
        .is_err()
    {
        return Vec::new();
    }

    let mut received = Vec::with_capacity(want);
    let read_all = async {
        let mut buf = vec![0u8; want];
        while received.len() < want {
            match stream.read(&mut buf).await {
                // The peer closed. What arrived is all there is.
                Ok(0) | Err(_) => break,
                Ok(n) => received.extend_from_slice(&buf[..n]),
            }
        }
    };
    let _ = tokio::time::timeout(SCAN_TIMEOUT, read_all).await;
    received.truncate(want);
    received
}

/// Send one datagram to `port` and read one back.
///
/// The UDP sibling of [`request_response`]. Only the source *address* is checked, deliberately not
/// the source port: TFTP answers a request to 69 from a freshly allocated ephemeral port, which is
/// the protocol working correctly rather than a stray packet.
pub(crate) async fn udp_request_response(
    ctx: &ProbeContext,
    port: PortType,
    request: &[u8],
    buf_len: usize,
) -> Vec<u8> {
    let target = ctx.target(port.number());
    let bind = if ctx.ip.is_ipv6() {
        "[::]:0"
    } else {
        "0.0.0.0:0"
    };
    let Ok(socket) = tokio::net::UdpSocket::bind(bind).await else {
        return Vec::new();
    };
    if socket.send_to(request, target).await.is_err() {
        return Vec::new();
    }

    let mut buf = vec![0u8; buf_len];
    match tokio::time::timeout(SCAN_TIMEOUT, socket.recv_from(&mut buf)).await {
        Ok(Ok((read, from))) if from.ip() == ctx.ip => {
            buf.truncate(read);
            buf
        }
        _ => Vec::new(),
    }
}

/// `Answered` with no identity when `detected`, `NoAnswer` otherwise.
///
/// Most probes establish presence and nothing else; this saves each of them writing the same
/// two-arm match.
pub(crate) fn presence(detected: bool) -> AppProbeOutcome {
    if detected {
        AppProbeOutcome::Answered { identity: None }
    } else {
        AppProbeOutcome::NoAnswer
    }
}

/// The probe registry *is* the service-definition registry.
pub fn all_app_probes() -> Vec<Box<dyn AppProbe>> {
    ServiceDefinitionRegistry::all_service_definitions()
        .iter()
        .flat_map(|d| d.app_probes())
        .collect()
}

/// Every probe paired with each port it runs on, from its [`ClientProbe::ports`].
pub fn probe_runs() -> Vec<(Arc<dyn AppProbe>, PortType)> {
    all_app_probes()
        .into_iter()
        .flat_map(|probe| {
            let probe: Arc<dyn AppProbe> = Arc::from(probe);
            probe
                .client_probe()
                .ports()
                .into_iter()
                .map(move |port| (Arc::clone(&probe), port))
        })
        .collect()
}

/// The probe runs [`scan_app_probes`] makes against one address. Every run passes through here, so
/// the raw-socket gate covers each one, TCP or UDP, without the probe having to know about it.
///
/// TCP probes gate on the connect scan having found the port open; UDP has no connect scan to gate
/// on, so its probes run whenever `udp` is set — which is what `scan_udp_ports` did for the same four probes.
fn select_probes(
    ctx: &ProbeContext,
    open_ports: &[PortType],
    udp: bool,
) -> Vec<(Arc<dyn AppProbe>, PortType)> {
    probe_runs()
        .into_iter()
        .filter(|(_, port)| ctx.probe_raw_socket_ports || !port.is_raw_socket())
        .filter(|(probe, _)| probe.applies(ctx))
        .filter(|(_, port)| {
            if port.is_udp() {
                udp
            } else {
                open_ports.contains(port)
            }
        })
        .collect()
}

/// Run every applicable probe against one address.
pub async fn scan_app_probes(
    ctx: &ProbeContext,
    open_ports: &[PortType],
    batch_size: usize,
    scan_rate_pps: u32,
    udp: bool,
) -> Vec<AppProbeResult> {
    let probes = select_probes(ctx, open_ports, udp);

    if probes.is_empty() {
        return Vec::new();
    }

    // UDP is slower and less reliable, capped the same way `scan_udp_ports` capped it.
    let probe_batch_size = std::cmp::min(batch_size, 10);
    let total = probes.len();
    let ip = ctx.ip;
    let ctx = ctx.clone();

    let results = batch_scan(
        probes,
        probe_batch_size,
        scan_rate_pps,
        ctx.cancel.clone(),
        move |(probe, port)| {
            let ctx = ctx.clone();
            async move {
                match probe.run(&ctx, port).await {
                    Ok(AppProbeOutcome::Answered { identity }) => {
                        tracing::trace!(ip = %ctx.ip, port = %port.number(), "Application probe answered");
                        Some(AppProbeResult {
                            port,
                            client_probe: Some(probe.client_probe()),
                            identity,
                        })
                    }
                    Ok(AppProbeOutcome::NoAnswer) => {
                        tracing::debug!(
                            ip = %ctx.ip,
                            port = %port.number(),
                            client_probe = ?probe.client_probe(),
                            "Application probe got no answer"
                        );
                        None
                    }
                    Err(e) => {
                        tracing::debug!(
                            ip = %ctx.ip,
                            port = %port.number(),
                            client_probe = ?probe.client_probe(),
                            error = %e,
                            "Application probe failed"
                        );
                        if DiscoveryCriticalError::is_critical_error(e.to_string()) {
                            tracing::error!(
                                "Critical error running application probe {}:{}: {}",
                                ctx.ip,
                                port.number(),
                                e
                            );
                        }
                        None
                    }
                }
            }
        },
    )
    .await;

    tracing::debug!(
        ip = %ip,
        probes_run = total,
        answered = results.len(),
        "Application probes complete"
    );

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::services::r#impl::patterns::ClientProbe;

    /// A probe reaches a mapped port on the port it's published on, and any other on its own.
    #[test]
    fn a_probe_reaches_a_mapped_port_on_its_published_port() {
        let ctx = ProbeContext {
            ip: "192.168.4.58".parse().unwrap(),
            subnet_cidr: "192.168.4.0/24".parse().unwrap(),
            is_gateway: false,
            cancel: CancellationToken::new(),
            scan_controller: crate::daemon::utils::scanner::ScanConcurrencyController::new(1),
            probe_raw_socket_ports: false,
            port_map: HashMap::from([(5432, 15432)]),
            tunnel: None,
        };
        assert_eq!(ctx.target(5432).port(), 15432);
        assert_eq!(ctx.target(6379).port(), 6379);
        assert_eq!(ctx.target(5432).ip(), ctx.ip);
    }

    /// Stands in for a container relay: each connection is an in-memory pipe to a peer that reads
    /// one request and answers with `reply` prefixed by the port it was opened for.
    #[derive(Debug)]
    struct PipeTunnel {
        reply: Option<&'static [u8]>,
    }

    #[async_trait]
    impl ProbeTunnel for PipeTunnel {
        async fn open(&self, port: u16) -> std::io::Result<Box<dyn ProbeStream>> {
            let Some(reply) = self.reply else {
                return Err(std::io::Error::other("exec refused"));
            };
            let (ours, mut theirs) = tokio::io::duplex(1024);
            tokio::spawn(async move {
                let mut request = [0u8; 64];
                if theirs.read(&mut request).await.is_ok() {
                    let _ = theirs.write_all(&port.to_be_bytes()).await;
                    let _ = theirs.write_all(reply).await;
                }
            });
            Ok(Box::new(ours))
        }
    }

    fn tunnelled(reply: Option<&'static [u8]>) -> ProbeContext {
        ProbeContext {
            // TEST-NET-1: never routed, so a probe that dialled it instead would read nothing.
            ip: "192.0.2.1".parse().unwrap(),
            subnet_cidr: "192.0.2.0/24".parse().unwrap(),
            is_gateway: false,
            cancel: CancellationToken::new(),
            scan_controller: crate::daemon::utils::scanner::ScanConcurrencyController::new(1),
            probe_raw_socket_ports: false,
            // Ignored through a tunnel, which reaches the container's own port.
            port_map: HashMap::from([(27017, 27018)]),
            tunnel: Some(Arc::new(PipeTunnel { reply })),
        }
    }

    #[tokio::test]
    async fn a_tunnelled_probe_talks_through_the_tunnel_to_the_containers_own_port() {
        let ctx = tunnelled(Some(b"pong"));
        let reply = request_response(&ctx, PortType::MongoDB, b"ping", 64).await;
        assert_eq!(reply, [&27017u16.to_be_bytes()[..], b"pong"].concat());

        let exact = request_exact(&ctx, PortType::MongoDB, b"ping", 6).await;
        assert_eq!(exact, reply);
    }

    /// A peer that answers in two segments with a pause between them, as a Google Nest router's
    /// DNS server sends its length prefix and then the message.
    #[derive(Debug)]
    struct SegmentedTunnel {
        first: &'static [u8],
        rest: &'static [u8],
    }

    #[async_trait]
    impl ProbeTunnel for SegmentedTunnel {
        async fn open(&self, _port: u16) -> std::io::Result<Box<dyn ProbeStream>> {
            let (first, rest) = (self.first, self.rest);
            let (ours, mut theirs) = tokio::io::duplex(1024);
            tokio::spawn(async move {
                let mut request = [0u8; 64];
                if theirs.read(&mut request).await.is_ok() {
                    let _ = theirs.write_all(first).await;
                    let _ = theirs.flush().await;
                    tokio::time::sleep(std::time::Duration::from_millis(30)).await;
                    let _ = theirs.write_all(rest).await;
                }
            });
            Ok(Box::new(ours))
        }
    }

    /// The router's reply to DnsTcp's root NS query: the prefix, then a SERVFAIL response echoing
    /// the transaction id. A single read saw only the prefix and reported no answer.
    #[tokio::test]
    async fn a_reply_split_across_segments_is_read_whole() {
        use crate::daemon::utils::app_probe::dns_tcp::DnsTcpProbe;

        let ctx = ProbeContext {
            tunnel: Some(Arc::new(SegmentedTunnel {
                first: &[0x00, 0x11],
                rest: &[
                    0x5C, 0xA1, 0x80, 0x82, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x02, 0x00, 0x01,
                ],
            })),
            ..tunnelled(None)
        };
        let reply = request_response(&ctx, PortType::DnsTcp, b"query", 1024).await;
        assert_eq!(reply.len(), 19, "prefix and message both read");
        assert_eq!(
            DnsTcpProbe
                .run(&ctx, PortType::DnsTcp)
                .await
                .expect("probe runs"),
            AppProbeOutcome::Answered { identity: None }
        );
    }

    /// A Brother HL-2270DW's Telnet greeting: a screen clear, then the option negotiation in a
    /// second segment. A single read saw only the escape codes.
    #[tokio::test]
    async fn a_greeting_split_across_segments_is_read_whole() {
        use crate::daemon::utils::app_probe::telnet::TelnetProbe;

        let ctx = ProbeContext {
            tunnel: Some(Arc::new(GreetingTunnel {
                first: b"\x1b[2J\x1b[1;1f",
                rest: &[0xFF, 0xFB, 0x01, 0xFF, 0xFB, 0x03, 0xFF, 0xFD, 0x03],
            })),
            ..tunnelled(None)
        };
        assert_eq!(
            TelnetProbe
                .run(&ctx, PortType::Telnet)
                .await
                .expect("probe runs"),
            AppProbeOutcome::Answered { identity: None }
        );
    }

    /// A server that speaks first, in two segments with a pause between.
    #[derive(Debug)]
    struct GreetingTunnel {
        first: &'static [u8],
        rest: &'static [u8],
    }

    #[async_trait]
    impl ProbeTunnel for GreetingTunnel {
        async fn open(&self, _port: u16) -> std::io::Result<Box<dyn ProbeStream>> {
            let (first, rest) = (self.first, self.rest);
            let (ours, mut theirs) = tokio::io::duplex(1024);
            tokio::spawn(async move {
                let _ = theirs.write_all(first).await;
                let _ = theirs.flush().await;
                tokio::time::sleep(std::time::Duration::from_millis(30)).await;
                let _ = theirs.write_all(rest).await;
                // Hold the connection open, as a real server does, until the probe hangs up.
                let mut drain = [0u8; 64];
                let _ = theirs.read(&mut drain).await;
            });
            Ok(Box::new(ours))
        }
    }

    #[tokio::test]
    async fn a_tunnel_that_cannot_open_is_no_answer() {
        let ctx = tunnelled(None);
        assert!(
            request_response(&ctx, PortType::MongoDB, b"ping", 64)
                .await
                .is_empty()
        );
        assert!(ctx.connect(27017).await.is_none());
    }

    /// Every probe is reachable only through a service definition, so this also asserts that each
    /// one has a definition behind it.
    #[test]
    fn every_registered_probe_declares_a_port_and_is_uniquely_ported() {
        let probes = all_app_probes();
        assert!(
            !probes.is_empty(),
            "the registry should expose the migrated UDP probes at minimum"
        );
        for probe in &probes {
            assert!(
                !probe.client_probe().ports().is_empty(),
                "{:?} names no port, so its probe never runs",
                probe.client_probe()
            );
        }

        let mut ports: Vec<_> = probe_runs().into_iter().map(|(_, port)| port).collect();
        let before = ports.len();
        ports.sort_by_key(|p| (p.number(), p.protocol()));
        ports.dedup();
        assert_eq!(
            before,
            ports.len(),
            "two probes claiming one port would race for the same evidence slot"
        );
    }

    /// A gateway address with every registered probe's port open, so nothing but the raw-socket
    /// gate stands between a probe and selection.
    fn selection_for(probe_raw_socket_ports: bool) -> Vec<(Arc<dyn AppProbe>, PortType)> {
        let ctx = ProbeContext {
            ip: IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 1, 1)),
            subnet_cidr: "192.168.1.0/24".parse().unwrap(),
            is_gateway: true,
            cancel: CancellationToken::new(),
            scan_controller: ScanConcurrencyController::new(16),
            probe_raw_socket_ports,
            port_map: Default::default(),
            tunnel: None,
        };
        let open_ports: Vec<PortType> = probe_runs().into_iter().map(|(_, port)| port).collect();
        select_probes(&ctx, &open_ports, true)
    }

    /// GH #801: with the setting off, nothing past a connect may reach 9100-9107. Derived from the
    /// registry, so a probe added on any raw-socket port fails here without this test changing.
    #[test]
    fn raw_socket_probes_do_not_run_with_the_setting_off() {
        let offending: Vec<u16> = selection_for(false)
            .into_iter()
            .map(|(_, port)| port)
            .filter(PortType::is_raw_socket)
            .map(|p| p.number())
            .collect();
        assert!(
            offending.is_empty(),
            "probes selected on raw-socket ports with probe_raw_socket_ports off: {offending:?}"
        );
    }

    #[test]
    fn raw_socket_probes_run_with_the_setting_on() {
        let raw_socket_ports = |runs: Vec<(Arc<dyn AppProbe>, PortType)>| -> Vec<PortType> {
            runs.into_iter()
                .map(|(_, port)| port)
                .filter(PortType::is_raw_socket)
                .collect()
        };
        let registered = raw_socket_ports(probe_runs());
        assert!(
            !registered.is_empty(),
            "no registered probe targets a raw-socket port, so the gate is untested"
        );
        assert_eq!(raw_socket_ports(selection_for(true)), registered);
    }

    /// Devices routinely pad a fixed-width identity field with spaces or return it empty, and an
    /// empty string is absence rather than a value — it must not occupy a rung where it would
    /// block a real reading. `identity_field` is where both probes get that, so neither can
    /// forget it.
    #[test]
    fn blank_identity_fields_are_absence_not_values() {
        let source = AttributeSource::Probe(ClientProbe::ModbusTcp);

        let blank: Option<HostManufacturerAttributed> =
            identity_field(Some("   ".to_string()), source);
        assert_eq!(blank, None);

        let empty: Option<HostModelAttributed> = identity_field(Some(String::new()), source);
        assert_eq!(empty, None);

        let padded: Option<HostSerialNumberAttributed> =
            identity_field(Some("  FOC1234X5YZ  ".to_string()), source);
        assert_eq!(
            padded.map(|c| c.value().0.clone()),
            Some("FOC1234X5YZ".to_string())
        );
    }
}
