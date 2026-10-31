//! SSH detection by its identification string.
//!
//! RFC 4253 §4.2 requires the server to send `SSH-protocolversion-softwareversion` before anything
//! else happens, so this needs no request at all. That also makes it one of the cleanest separations
//! available between a real service and a listener that merely completes a handshake: a middlebox
//! answering on behalf of an empty address has nothing to send.

use std::time::Duration;

use anyhow::Error;
use async_trait::async_trait;
use tokio::time::Instant;

use crate::daemon::utils::app_probe::{
    AppProbe, AppProbeOutcome, DeviceIdentity, ProbeContext, read_greeting,
};
use crate::daemon::utils::scanner::SCAN_TIMEOUT;
use crate::server::hosts::r#impl::os::recog::RecogDatabase;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::r#impl::patterns::ClientProbe;
use crate::server::shared::attribution::AttributeSource;

/// The identification string every SSH server opens with.
const SSH_BANNER: &[u8] = b"SSH-";

pub struct SshProbe;

#[async_trait]
impl AppProbe for SshProbe {
    fn client_probe(&self) -> ClientProbe {
        ClientProbe::Ssh
    }

    /// Retried when the server closed the connection without a banner, after a pause.
    ///
    /// OpenSSH's default `MaxStartups 10:30:100` starts refusing new unauthenticated connections
    /// at random once 10 are pending, and a refused client sees the connection close before the
    /// banner. A scan that reaches one sshd on many addresses at once (a VM with secondary IPs,
    /// a NAT fanning in) gets most of its connections refused: on a lab VM answering on 40
    /// addresses, 26 read no banner. A refused connection closes at once, so a quick empty read
    /// is retried. A listener that accepts and says nothing uses up the read timeout and is not.
    async fn run(&self, ctx: &ProbeContext, port: PortType) -> Result<AppProbeOutcome, Error> {
        for pause in std::iter::once(Duration::ZERO).chain(RETRY_PAUSES) {
            tokio::time::sleep(pause).await;
            let started = Instant::now();
            let greeting = read_greeting(ctx, port, 512).await;
            if !greeting.is_empty() || started.elapsed() >= SCAN_TIMEOUT {
                return Ok(parse_banner(&greeting));
            }
        }
        Ok(AppProbeOutcome::NoAnswer)
    }
}

/// Pauses before each retry of a connection closed without a banner. By then the probes that
/// held sshd's unauthenticated slots have read their banners and let go of them.
const RETRY_PAUSES: [Duration; 2] = [Duration::from_millis(500), Duration::from_millis(1500)];

/// Whether the opening bytes are an SSH identification string, and what it names, if anything.
///
/// The version that follows is not checked: `SSH-1.99` and `SSH-2.0` are both real, and a server
/// free-texting its software name after them is expected. The prefix is what identifies the
/// protocol.
///
/// The software version after it is matched against Recog: a distribution's OpenSSH package
/// appends its own tag (`OpenSSH_9.6p1 Ubuntu-3ubuntu13.5`), some servers name their OS outright
/// (`OpenSSH_for_Windows`, `ROSSSH`), and embedded ones name the device. A bare `OpenSSH_9.6`
/// names nothing about the host, and the probe still answers.
fn parse_banner(bytes: &[u8]) -> AppProbeOutcome {
    if !bytes.starts_with(SSH_BANNER) {
        return AppProbeOutcome::NoAnswer;
    }
    let line = String::from_utf8_lossy(bytes);
    let line = line.lines().next().unwrap_or_default();
    let identity = line.splitn(3, '-').nth(2).and_then(|software| {
        DeviceIdentity::matched(
            RecogDatabase::SshBanner,
            software,
            AttributeSource::SshBannerMatch,
        )
    });
    AppProbeOutcome::Answered { identity }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use tokio::io::AsyncWriteExt;
    use tokio::net::TcpListener;
    use tokio_util::sync::CancellationToken;

    use crate::daemon::utils::scanner::ScanConcurrencyController;

    /// A context that reaches 22 at `listener_port` on loopback.
    fn loopback(listener_port: u16) -> ProbeContext {
        ProbeContext {
            ip: "127.0.0.1".parse().unwrap(),
            subnet_cidr: "127.0.0.0/8".parse().unwrap(),
            is_gateway: false,
            cancel: CancellationToken::new(),
            scan_controller: ScanConcurrencyController::new(1),
            probe_raw_socket_ports: false,
            port_map: HashMap::from([(22, listener_port)]),
            tunnel: None,
        }
    }

    /// Accepts connections on loopback, handing each to `serve` with its index, and counts them.
    async fn listen(
        serve: fn(usize, tokio::net::TcpStream) -> tokio::task::JoinHandle<()>,
    ) -> (u16, Arc<AtomicUsize>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let accepted = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&accepted);
        tokio::spawn(async move {
            while let Ok((stream, _)) = listener.accept().await {
                serve(counter.fetch_add(1, Ordering::SeqCst), stream);
            }
        });
        (port, accepted)
    }

    #[tokio::test]
    async fn a_connection_refused_before_the_banner_is_retried() {
        // MaxStartups' shape: the first connection closes at once, the next one gets the banner.
        let (port, accepted) = listen(|index, mut stream| {
            tokio::spawn(async move {
                if index > 0 {
                    let _ = stream.write_all(b"SSH-2.0-OpenSSH_9.6\r\n").await;
                }
            })
        })
        .await;

        let outcome = SshProbe.run(&loopback(port), PortType::Ssh).await.unwrap();

        assert_eq!(outcome, AppProbeOutcome::Answered { identity: None });
        assert_eq!(accepted.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn a_listener_that_stays_silent_is_not_retried() {
        let (port, accepted) = listen(|_, stream| {
            tokio::spawn(async move {
                tokio::time::sleep(SCAN_TIMEOUT * 4).await;
                drop(stream);
            })
        })
        .await;

        let outcome = SshProbe.run(&loopback(port), PortType::Ssh).await.unwrap();

        assert_eq!(outcome, AppProbeOutcome::NoAnswer);
        assert_eq!(accepted.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn an_identification_string_is_ssh() {
        for banner in [
            &b"SSH-2.0-OpenSSH_9.6\r\n"[..],
            &b"SSH-1.99-Cisco-1.25\r\n"[..],
            &b"SSH-2.0-dropbear_2022.83\r\n"[..],
        ] {
            assert!(matches!(
                parse_banner(banner),
                AppProbeOutcome::Answered { .. }
            ));
        }
    }

    #[test]
    fn a_distribution_banner_names_the_os_and_a_bare_one_names_none() {
        let AppProbeOutcome::Answered { identity } =
            parse_banner(b"SSH-2.0-OpenSSH_6.6.1p1 Ubuntu-2ubuntu2.13\r\n")
        else {
            panic!("an identification string answers");
        };
        let (matched, source) = *identity
            .and_then(|i| i.matched)
            .expect("the Ubuntu tag names an OS");
        let os = matched.os.expect("the Ubuntu tag names an OS");
        assert_eq!(os.name.as_deref(), Some("Ubuntu"));
        assert_eq!(source, AttributeSource::SshBannerMatch);

        assert_eq!(
            parse_banner(b"SSH-2.0-OpenSSH_9.6\r\n"),
            AppProbeOutcome::Answered { identity: None }
        );
    }

    #[test]
    fn silence_or_another_protocol_is_not_ssh() {
        for banner in [
            &b""[..],
            &b"220 ProFTPD Server ready\r\n"[..],
            &b"HTTP/1.1 400 Bad Request\r\n"[..],
            &b"\0\0\0\0"[..],
        ] {
            assert_eq!(parse_banner(banner), AppProbeOutcome::NoAnswer);
        }
    }
}
