//! LPD detection with a short queue-state request.
//!
//! RFC 1179 gives the line printer daemon no greeting, so the client has to speak first. The one
//! command that reads without printing is "send queue state" (§5.3): `0x03`, a queue name, `LF`.
//! The daemon answers with a plain-text status of that queue and closes the connection.
//!
//! The command needs a queue name. HP Jetdirect print servers accept the fixed names `raw`, `text`,
//! `auto` and `binps`, and read any other name as `raw1`, so `raw` reaches a queue on every
//! Jetdirect. A daemon that has no queue by that name still answers in text that it has none,
//! which is the protocol answering.
//!
//! Nothing is printed: the request only reads the queue's state.

use anyhow::Error;
use async_trait::async_trait;

use crate::daemon::utils::app_probe::{
    AppProbe, AppProbeOutcome, ProbeContext, presence, request_response,
};
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::r#impl::patterns::ClientProbe;

/// "Send queue state (short)" for the `raw` queue.
const QUEUE_STATE: &[u8] = b"\x03raw\n";

/// A queue status is a few lines of text.
const MAX_REPLY: usize = 1024;

pub struct LpdProbe;

#[async_trait]
impl AppProbe for LpdProbe {
    fn client_probe(&self) -> ClientProbe {
        ClientProbe::Lpd
    }

    async fn run(&self, ctx: &ProbeContext, port: PortType) -> Result<AppProbeOutcome, Error> {
        Ok(parse_reply(
            &request_response(ctx, port, QUEUE_STATE, MAX_REPLY).await,
        ))
    }
}

/// Whether the reply is the plain-text status RFC 1179 describes.
///
/// The status has no fixed wording, so the check is its shape: something came back, and all of it
/// is printable text. A binary protocol that happened to answer the request does not pass, nor does
/// another text protocol's greeting that opens with a status code.
fn parse_reply(bytes: &[u8]) -> AppProbeOutcome {
    let text = String::from_utf8_lossy(bytes);
    presence(
        !text.trim().is_empty()
            && bytes
                .iter()
                .all(|b| b.is_ascii_graphic() || b.is_ascii_whitespace())
            && !looks_like_another_protocol(&text),
    )
}

/// Text protocols that answer anything sent to them with their own status line: HTTP, SSH, and the
/// `220 `/`500-` reply codes of FTP, SMTP and their kin.
fn looks_like_another_protocol(text: &str) -> bool {
    let bytes = text.as_bytes();
    text.starts_with("HTTP/")
        || text.starts_with("SSH-")
        || (bytes.len() >= 4
            && bytes[..3].iter().all(u8::is_ascii_digit)
            && matches!(bytes[3], b' ' | b'-'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_queue_status_is_lpd() {
        for reply in [
            // An HP LaserJet M110w's Jetdirect, idle: no newline.
            &b"queue empty"[..],
            &b"Printer: raw is ready\nno entries\n"[..],
            // A daemon with no queue by that name still answers in text.
            &b"Unknown printer raw\n"[..],
            &b"no entries\n"[..],
        ] {
            assert_eq!(
                parse_reply(reply),
                AppProbeOutcome::Answered { identity: None },
                "{}",
                String::from_utf8_lossy(reply)
            );
        }
    }

    #[test]
    fn silence_binary_or_another_protocol_is_not_lpd() {
        for reply in [
            &b""[..],
            &b"\n"[..],
            &b"\0\x01\x02"[..],
            &b"HTTP/1.1 400 Bad Request\r\n\r\n"[..],
            &b"SSH-2.0-OpenSSH_9.6\r\n"[..],
            &b"220 ProFTPD Server ready\r\n"[..],
            &b"500-Syntax error\r\n"[..],
        ] {
            assert_eq!(
                parse_reply(reply),
                AppProbeOutcome::NoAnswer,
                "{}",
                String::from_utf8_lossy(reply)
            );
        }
    }
}
