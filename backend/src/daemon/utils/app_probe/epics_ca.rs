//! EPICS Channel Access detection with `CA_PROTO_VERSION`.
//!
//! Every Channel Access message opens with the same 16-byte big-endian header: command, payload
//! size, data type, data count and two parameters. A client opens a circuit by sending a
//! `CA_PROTO_VERSION` header carrying its minor protocol version, and the server answers with its
//! own. The exchange creates no channel and touches no process variable, which is why it is safe
//! to run by default.
//!
//! Detection only. The version reply carries the protocol's minor version, nothing about the IOC.

use anyhow::Error;
use async_trait::async_trait;

use crate::daemon::utils::app_probe::{
    AppProbe, AppProbeOutcome, ProbeContext, presence, request_response,
};
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::r#impl::patterns::ClientProbe;

/// The Channel Access server port. Searches go to UDP 5064 and circuits to TCP 5064.
const CA_SERVER_PORT: u16 = 5064;

const HEADER_LEN: usize = 16;

/// `CA_PROTO_VERSION`, the first message on a circuit in either direction.
const CA_PROTO_VERSION: u16 = 0;

/// The minor version this probe announces. 13 is the current protocol (EPICS 3.14 onward); a
/// server answers with its own, whatever the client sends.
const CA_MINOR_VERSION: u16 = 13;

pub struct EpicsCaProbe;

#[async_trait]
impl AppProbe for EpicsCaProbe {
    fn port(&self) -> PortType {
        PortType::new_tcp(CA_SERVER_PORT)
    }

    fn client_probe(&self) -> Option<ClientProbe> {
        Some(ClientProbe::EpicsCa)
    }

    async fn run(&self, ctx: &ProbeContext) -> Result<AppProbeOutcome, Error> {
        let reply = request_response(ctx, self.port(), &version_request(), 256).await;
        Ok(presence(is_version_reply(&reply)))
    }
}

/// A `CA_PROTO_VERSION` header: no payload, priority 0 in the data type field, the minor version
/// in the count field, both parameters zero.
fn version_request() -> [u8; HEADER_LEN] {
    let mut msg = [0u8; HEADER_LEN];
    msg[0..2].copy_from_slice(&CA_PROTO_VERSION.to_be_bytes());
    msg[6..8].copy_from_slice(&CA_MINOR_VERSION.to_be_bytes());
    msg
}

/// Whether the reply opens with a `CA_PROTO_VERSION` header.
///
/// The command alone is two zero bytes, which plenty of binary protocols start with. A version
/// header also has no payload and a nonzero minor version in its count field, and that
/// combination is what makes it Channel Access rather than a coincidence.
fn is_version_reply(reply: &[u8]) -> bool {
    if reply.len() < HEADER_LEN {
        return false;
    }
    let field = |at: usize| u16::from_be_bytes([reply[at], reply[at + 1]]);
    let (command, payload_size, minor_version) = (field(0), field(2), field(6));
    command == CA_PROTO_VERSION && payload_size == 0 && minor_version > 0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn version_header(minor_version: u16) -> Vec<u8> {
        let mut msg = vec![0u8; HEADER_LEN];
        msg[6..8].copy_from_slice(&minor_version.to_be_bytes());
        msg
    }

    #[test]
    fn a_version_reply_is_a_channel_access_server() {
        assert!(is_version_reply(&version_header(13)));
        assert!(is_version_reply(&version_header(11)));
    }

    /// A server may follow the version with more messages in the same read.
    #[test]
    fn trailing_messages_after_the_version_still_count() {
        let mut reply = version_header(13);
        reply.extend_from_slice(&[0u8, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert!(is_version_reply(&reply));
    }

    #[test]
    fn another_protocol_on_the_port_is_not_channel_access() {
        assert!(!is_version_reply(b"HTTP/1.1 400 Bad Request\r\n\r\n"));
        assert!(!is_version_reply(b"SSH-2.0-OpenSSH_9.6\r\n"));
    }

    /// Sixteen zero bytes start with the right command but carry no version.
    #[test]
    fn zeros_are_not_a_version_header() {
        assert!(!is_version_reply(&[0u8; HEADER_LEN]));
    }

    #[test]
    fn a_header_with_a_payload_is_not_a_version_reply() {
        let mut reply = version_header(13);
        reply[2..4].copy_from_slice(&8u16.to_be_bytes());
        assert!(!is_version_reply(&reply));
    }

    #[test]
    fn a_truncated_reply_is_not_a_header() {
        assert!(!is_version_reply(&version_header(13)[..8]));
        assert!(!is_version_reply(b""));
    }

    /// The request is itself a version header, so a server reading it the way this probe reads
    /// the reply sees a well-formed one.
    #[test]
    fn the_request_is_a_version_header() {
        assert!(is_version_reply(&version_request()));
    }
}
