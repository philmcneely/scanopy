//! Application probes run from inside a container's own network namespace.
//!
//! A container's unpublished port is on a bridge the daemon often has no route to. Docker Desktop's
//! bridges are inside its VM, rootless Podman's inside a user namespace, and a daemon in its own
//! bridge network is isolated from every other one. The container can always reach its own
//! listeners on loopback, so an exec running a relay inside it (`nc`, bash's `/dev/tcp`, or perl)
//! carries a probe's bytes to `127.0.0.1:<port>` and the replies back. The probe sees an ordinary
//! duplex stream through [`ProbeTunnel`] and does not change.

use std::collections::HashSet;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::time::Duration;

use async_trait::async_trait;
use bollard::Docker;
use bollard::container::LogOutput;
use futures::StreamExt;
use tokio_util::io::StreamReader;

use crate::daemon::utils::app_probe::{ProbeStream, ProbeTunnel};

/// Lists the relay tools the image carries, then the listening sockets. One exec answers both,
/// since a relay is only worth running for a port something listens on where it can reach.
pub(crate) const RELAY_SURVEY_SCRIPT: &str = "for t in timeout nc bash perl; do \
     command -v $t >/dev/null 2>&1 && echo \"relay-tool $t\"; \
   done; \
   cat /proc/net/tcp /proc/net/tcp6 2>/dev/null";

/// How long a relay process may live inside the container. Longer than any one probe's exchange
/// (a Modbus probe makes two round trips at [`SCAN_TIMEOUT`] each); short enough that a relay the
/// daemon abandoned is gone in seconds.
///
/// [`SCAN_TIMEOUT`]: crate::daemon::utils::scanner::SCAN_TIMEOUT
const RELAY_LIFETIME_SECS: u32 = 5;

/// Creating and starting an exec is two API round trips, slower than a TCP connect and slower
/// again through Docker Desktop's VM.
const RELAY_OPEN_TIMEOUT: Duration = Duration::from_secs(3);

/// Copies stdin to the socket and the socket to stdout until either side closes. Exits on the
/// alarm, so it bounds its own lifetime without `timeout`.
const PERL_RELAY: &str = "$SIG{ALRM}=sub{exit};alarm $ARGV[1];\
my $s=IO::Socket::INET->new(PeerAddr=>'127.0.0.1',PeerPort=>$ARGV[0]) or exit;\
my $sel=IO::Select->new(\\*STDIN,$s);\
while(my @r=$sel->can_read){for my $h (@r){\
my $n=sysread($h,my $b,65536);exit unless $n;\
syswrite(fileno($h)==fileno($s)?\\*STDOUT:$s,$b)}}";

/// Bash has no built-in relay, so the socket's half is a background `cat`, killed once stdin
/// closes so it can't outlive the exchange on a peer that keeps the connection open.
const BASH_RELAY: &str = "exec 3<>/dev/tcp/127.0.0.1/$0 || exit; cat <&3 & r=$!; cat >&3; kill $r";

/// A relay the container's image can run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RelayTool {
    Nc,
    Bash,
    Perl,
}

impl RelayTool {
    /// The first relay the survey found that can bound its own lifetime: `nc` and bash only under
    /// `timeout`, perl on its own alarm. `None` when there is none, or the exec never ran.
    fn choose(tools: &HashSet<&str>) -> Option<Self> {
        let bounded = tools.contains("timeout");
        if bounded && tools.contains("nc") {
            Some(Self::Nc)
        } else if bounded && tools.contains("bash") {
            Some(Self::Bash)
        } else if tools.contains("perl") {
            Some(Self::Perl)
        } else {
            None
        }
    }

    /// The exec's argv. No shell in between, so nothing here needs quoting.
    fn command(self, port: u16) -> Vec<String> {
        let port = port.to_string();
        let lifetime = RELAY_LIFETIME_SECS.to_string();
        let argv: Vec<&str> = match self {
            Self::Nc => vec!["timeout", &lifetime, "nc", "127.0.0.1", &port],
            Self::Bash => vec!["timeout", &lifetime, "bash", "-c", BASH_RELAY, &port],
            Self::Perl => vec![
                "perl",
                "-MIO::Socket::INET",
                "-MIO::Select",
                "-e",
                PERL_RELAY,
                &port,
                &lifetime,
            ],
        };
        argv.into_iter().map(String::from).collect()
    }
}

/// What the survey exec found: the relay to use, and the ports listening where it can reach them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RelaySurvey {
    pub tool: RelayTool,
    pub loopback_ports: HashSet<u16>,
}

impl RelaySurvey {
    /// `None` when the image has no usable relay.
    pub(crate) fn parse(output: &str) -> Option<Self> {
        let tools: HashSet<&str> = output
            .lines()
            .filter_map(|l| l.strip_prefix("relay-tool "))
            .map(str::trim)
            .collect();
        let tool = RelayTool::choose(&tools)?;
        let loopback_ports = listening_sockets(output)
            .filter(|(address, _)| reachable_on_loopback(address))
            .map(|(_, port)| port)
            .collect();
        Some(Self {
            tool,
            loopback_ports,
        })
    }
}

/// The `(local address, port)` of every listening socket in `/proc/net/tcp`-format text. Other
/// lines, headers included, are skipped.
pub(crate) fn listening_sockets(procfs: &str) -> impl Iterator<Item = (&str, u16)> {
    const TCP_LISTEN: &str = "0A";

    procfs.lines().filter_map(|line| {
        let mut fields = line.split_whitespace();
        // sl, local_address, rem_address, st
        let _sl = fields.next()?;
        let local = fields.next()?;
        let _rem = fields.next()?;
        if fields.next()? != TCP_LISTEN {
            return None;
        }
        let (address, port) = local.rsplit_once(':')?;
        Some((address, u16::from_str_radix(port, 16).ok()?))
    })
}

/// Whether a listener bound to this procfs address accepts a connection to `127.0.0.1`.
///
/// The kernel prints each 32-bit word of the address in host byte order, which is little-endian on
/// every architecture the daemon ships for. `::` counts because Linux binds it dual-stack unless
/// the socket asks otherwise; `::1` does not, since the relay connects over IPv4.
fn reachable_on_loopback(address: &str) -> bool {
    let words: Option<Vec<[u8; 4]>> = (0..address.len() / 8)
        .map(|i| {
            let word = address.get(i * 8..i * 8 + 8)?;
            u32::from_str_radix(word, 16).ok().map(u32::to_le_bytes)
        })
        .collect();
    let ip = match words.as_deref() {
        Some([v4]) => IpAddr::V4(Ipv4Addr::from(*v4)),
        Some([a, b, c, d]) => {
            let mut octets = [0u8; 16];
            for (i, word) in [a, b, c, d].into_iter().enumerate() {
                octets[i * 4..i * 4 + 4].copy_from_slice(word);
            }
            IpAddr::V6(Ipv6Addr::from(octets))
        }
        _ => return false,
    };
    match ip {
        IpAddr::V4(v4) => v4.is_unspecified() || v4.is_loopback(),
        IpAddr::V6(v6) => {
            v6.is_unspecified() || v6.to_ipv4_mapped().is_some_and(|v4| v4.is_loopback())
        }
    }
}

/// Opens each probe connection as an exec running a relay inside the container.
pub(crate) struct ExecRelay {
    pub client: Docker,
    pub container: String,
    pub tool: RelayTool,
}

impl std::fmt::Debug for ExecRelay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExecRelay")
            .field("container", &self.container)
            .field("tool", &self.tool)
            .finish()
    }
}

#[async_trait]
impl ProbeTunnel for ExecRelay {
    async fn open(&self, port: u16) -> std::io::Result<Box<dyn ProbeStream>> {
        let start = async {
            let exec = self
                .client
                .create_exec(
                    &self.container,
                    bollard::exec::CreateExecOptions {
                        cmd: Some(self.tool.command(port)),
                        attach_stdin: Some(true),
                        attach_stdout: Some(true),
                        // A relay's complaints ("connection refused") are not the peer's bytes.
                        attach_stderr: Some(false),
                        ..Default::default()
                    },
                )
                .await
                .map_err(std::io::Error::other)?;
            self.client
                .start_exec(&exec.id, None)
                .await
                .map_err(std::io::Error::other)
        };
        let started = tokio::time::timeout(RELAY_OPEN_TIMEOUT, start)
            .await
            .map_err(|_| std::io::Error::from(std::io::ErrorKind::TimedOut))??;

        let bollard::exec::StartExecResults::Attached { output, input } = started else {
            return Err(std::io::Error::other("the exec started detached"));
        };
        let replies = output.filter_map(|frame| {
            futures::future::ready(match frame {
                Ok(LogOutput::StdOut { message }) => Some(Ok(message)),
                Ok(_) => None,
                Err(e) => Some(Err(std::io::Error::other(e))),
            })
        });
        Ok(Box::new(tokio::io::join(StreamReader::new(replies), input)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `/proc/net/tcp` and `tcp6` rows as a container prints them, one listener per bind shape.
    const PROCFS: &str = "\
relay-tool timeout
relay-tool bash
relay-tool perl
  sl  local_address rem_address   st tx_queue rx_queue tr tm->when retrnsmt   uid  timeout inode
   0: 00000000:6989 00000000:0000 0A 00000000:00000000 00:00000000 00000000   999        0 1 1 0000000000000000 100 0 0 10 0
   1: 0100007F:1F90 00000000:0000 0A 00000000:00000000 00:00000000 00000000     0        0 2 1 0000000000000000 100 0 0 10 0
   2: 0200140A:0050 00000000:0000 0A 00000000:00000000 00:00000000 00000000     0        0 3 1 0000000000000000 100 0 0 10 0
   3: 0100007F:A1B2 0100007F:6989 01 00000000:00000000 00:00000000 00000000     0        0 4 1 0000000000000000 100 0 0 10 0
  sl  local_address                         remote_address                        st tx_queue rx_queue tr tm->when retrnsmt   uid  timeout inode
   0: 00000000000000000000000000000000:1538 00000000000000000000000000000000:0000 0A 00000000:00000000 00:00000000 00000000     0        0 5 1 0000000000000000 100 0 0 10 0
   1: 00000000000000000000000001000000:18EB 00000000000000000000000000000000:0000 0A 00000000:00000000 00:00000000 00000000     0        0 6 1 0000000000000000 100 0 0 10 0
   2: 0000000000000000FFFF00000100007F:2382 00000000000000000000000000000000:0000 0A 00000000:00000000 00:00000000 00000000     0        0 7 1 0000000000000000 100 0 0 10 0
";

    #[test]
    fn only_listeners_reachable_over_ipv4_loopback_are_relayed() {
        let survey = RelaySurvey::parse(PROCFS).expect("bash under timeout is usable");
        assert_eq!(
            survey.loopback_ports,
            HashSet::from([
                27017, // 0.0.0.0
                8080,  // 127.0.0.1
                5432,  // ::, dual-stack
                9090,  // ::ffff:127.0.0.1
            ]),
            "a listener on the container's own address (10.20.0.2:80) or on ::1 (6379) is not \
             reachable at 127.0.0.1, and an established connection is not a listener"
        );
    }

    #[test]
    fn a_relay_is_chosen_only_when_it_can_bound_its_own_lifetime() {
        let pick = |tools: &[&str]| RelayTool::choose(&tools.iter().copied().collect());
        assert_eq!(
            pick(&["timeout", "nc", "bash", "perl"]),
            Some(RelayTool::Nc)
        );
        assert_eq!(pick(&["timeout", "bash"]), Some(RelayTool::Bash));
        assert_eq!(pick(&["nc", "bash", "perl"]), Some(RelayTool::Perl));
        assert_eq!(pick(&["nc", "bash"]), None, "no timeout and no perl");
        assert_eq!(pick(&[]), None, "distroless, or the exec was refused");
    }

    #[test]
    fn an_image_without_a_relay_yields_no_survey() {
        let procfs_only = PROCFS
            .lines()
            .filter(|l| !l.starts_with("relay-tool"))
            .collect::<Vec<_>>()
            .join("\n");
        assert_eq!(RelaySurvey::parse(&procfs_only), None);
    }

    /// Every TCP probe whose port the container listens on, run through a real relay into it.
    ///
    /// Needs a local Docker and a running container, named in `SCANOPY_LIVE_RELAY_CONTAINER`:
    ///
    /// ```text
    /// docker run -d --name relay-mongo mongo:8
    /// SCANOPY_LIVE_RELAY_CONTAINER=relay-mongo cargo test --lib -- --ignored relay_reaches
    /// ```
    #[tokio::test(flavor = "multi_thread")]
    #[ignore = "needs a local Docker and a container named in SCANOPY_LIVE_RELAY_CONTAINER"]
    async fn a_live_relay_reaches_every_probe_the_container_listens_for() {
        use crate::daemon::utils::app_probe::{ProbeContext, probe_runs};
        use std::sync::Arc;

        let container = std::env::var("SCANOPY_LIVE_RELAY_CONTAINER").expect("name a container");
        let client = Docker::connect_with_local_defaults().expect("a local Docker");
        let survey = client
            .create_exec(
                &container,
                bollard::exec::CreateExecOptions {
                    cmd: Some(vec!["sh", "-c", RELAY_SURVEY_SCRIPT]),
                    attach_stdout: Some(true),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        let bollard::exec::StartExecResults::Attached { output, .. } =
            client.start_exec(&survey.id, None).await.unwrap()
        else {
            panic!("detached");
        };
        let text: String = output
            .filter_map(|f| async move { f.ok().map(|m| m.to_string()) })
            .collect::<Vec<_>>()
            .await
            .concat();
        let survey = RelaySurvey::parse(&text).expect("the image carries a relay");

        let ctx = ProbeContext {
            // Unrouted: anything answering came through the relay.
            ip: "192.0.2.1".parse().unwrap(),
            subnet_cidr: "192.0.2.0/24".parse().unwrap(),
            is_gateway: false,
            cancel: tokio_util::sync::CancellationToken::new(),
            scan_controller: crate::daemon::utils::scanner::ScanConcurrencyController::new(4),
            probe_raw_socket_ports: true,
            port_map: Default::default(),
            tunnel: Some(Arc::new(ExecRelay {
                client,
                container,
                tool: survey.tool,
            })),
        };
        let mut missed = Vec::new();
        let mut recognised = Vec::new();
        for (probe, port) in probe_runs()
            .into_iter()
            .filter(|(_, port)| port.is_tcp() && survey.loopback_ports.contains(&port.number()))
        {
            match probe.run(&ctx, port).await {
                Ok(crate::daemon::utils::app_probe::AppProbeOutcome::Answered { .. }) => {
                    recognised.push(probe.client_probe())
                }
                _ => missed.push(probe.client_probe()),
            }
        }
        eprintln!("relay {:?}; recognised {recognised:?}", survey.tool);
        assert!(!recognised.is_empty(), "nothing answered through the relay");
        // A port can carry probes for several protocols, so only the count of answers is checked.
        eprintln!("not this protocol: {missed:?}");
    }

    /// The port reaches the relay as an argument and nothing else does: a port that ended up in
    /// the middle of a script would be a quoting bug waiting for a non-numeric value.
    #[test]
    fn every_relay_takes_the_port_as_its_own_argument() {
        for tool in [RelayTool::Nc, RelayTool::Bash, RelayTool::Perl] {
            let argv = tool.command(27017);
            assert!(argv.iter().any(|a| a == "27017"), "{tool:?}: {argv:?}");
            assert!(
                !argv.iter().any(|a| a != "27017" && a.contains("27017")),
                "{tool:?}"
            );
        }
    }
}
