//! VNC detection by the server's greeting.
//!
//! RFB (RFC 6143) opens with the server sending its protocol version, twelve bytes of the form
//! `RFB xxx.yyy\n`, before the client says anything. macOS Screen Sharing sends `RFB 003.889`;
//! RealVNC, TigerVNC and x11vnc send `RFB 003.008` or similar. The `RFB ` prefix and the version's
//! shape are the protocol's own framing.

use anyhow::Error;
use async_trait::async_trait;

use crate::daemon::utils::app_probe::{
    AppProbe, AppProbeOutcome, ProbeContext, presence, read_greeting,
};
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::r#impl::patterns::ClientProbe;

/// The length of an RFB `ProtocolVersion` message.
const VERSION_LEN: usize = 12;

pub struct VncProbe;

#[async_trait]
impl AppProbe for VncProbe {
    fn client_probe(&self) -> ClientProbe {
        ClientProbe::Vnc
    }

    async fn run(&self, ctx: &ProbeContext, port: PortType) -> Result<AppProbeOutcome, Error> {
        Ok(parse_version(&read_greeting(ctx, port, VERSION_LEN).await))
    }
}

/// Whether the greeting is an RFB `ProtocolVersion`: `RFB `, three digits, `.`, three digits, `\n`.
fn parse_version(bytes: &[u8]) -> AppProbeOutcome {
    let is_digits = |range: std::ops::Range<usize>| {
        bytes
            .get(range)
            .is_some_and(|digits| digits.iter().all(u8::is_ascii_digit))
    };
    presence(
        bytes.starts_with(b"RFB ")
            && is_digits(4..7)
            && bytes.get(7) == Some(&b'.')
            && is_digits(8..11)
            && bytes.get(11) == Some(&b'\n'),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_rfb_version_greeting_is_vnc() {
        for greeting in [&b"RFB 003.889\n"[..], b"RFB 003.008\n"] {
            assert_eq!(
                parse_version(greeting),
                AppProbeOutcome::Answered { identity: None }
            );
        }
    }

    #[test]
    fn other_greetings_are_not_vnc() {
        for greeting in [
            &b""[..],
            b"SSH-2.0-OpenSSH_9.6\r\n",
            b"HTTP/1.1 400 Bad Request\r\n",
            b"RFB 3.8\n",
            b"RFB 003.008",
        ] {
            assert_eq!(parse_version(greeting), AppProbeOutcome::NoAnswer);
        }
    }
}
