//! SMUX detection with an `OpenPDU` the agent refuses.
//!
//! SMUX (RFC 1227) is how an SNMP subagent registers its MIB subtrees with the agent, over TCP 199.
//! The agent sends nothing first. The peer opens with an `OpenPDU` naming itself and a password, and
//! an agent that has no `smuxpeer` entry for that identity answers with a `ClosePDU` carrying
//! `authenticationFailure` and closes. That refusal is SMUX's own encoding, so it confirms the port.
//!
//! net-snmp's `snmpd` opens 199 on every address by default, so it shows on every host running it,
//! whether or not anything uses SMUX.
//!
//! Nothing is registered: the open is refused before any subtree could be.

use anyhow::Error;
use async_trait::async_trait;

use crate::daemon::utils::app_probe::{
    AppProbe, AppProbeOutcome, ProbeContext, presence, request_response,
};
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::r#impl::patterns::ClientProbe;

/// `OpenPDU ::= [APPLICATION 0] IMPLICIT SEQUENCE { version 0, identity 1.3.6.1, description
/// "test", password "" }`, BER-encoded.
const OPEN_PDU: &[u8] = &[
    0x60, 0x10, // [APPLICATION 0], constructed, 16 bytes
    0x02, 0x01, 0x00, // version INTEGER 0
    0x06, 0x03, 0x2b, 0x06, 0x01, // identity OBJECT IDENTIFIER 1.3.6.1
    0x04, 0x04, b't', b'e', b's', b't', // description OCTET STRING "test"
    0x04, 0x00, // password OCTET STRING, empty
];

/// `ClosePDU ::= [APPLICATION 1] IMPLICIT INTEGER`, primitive.
const CLOSE_PDU_TAG: u8 = 0x41;

/// The highest `ClosePDU` reason RFC 1227 defines, `authenticationFailure`.
const MAX_CLOSE_REASON: u8 = 5;

pub struct SmuxProbe;

#[async_trait]
impl AppProbe for SmuxProbe {
    fn client_probe(&self) -> ClientProbe {
        ClientProbe::Smux
    }

    async fn run(&self, ctx: &ProbeContext, port: PortType) -> Result<AppProbeOutcome, Error> {
        Ok(parse_reply(
            &request_response(ctx, port, OPEN_PDU, 64).await,
        ))
    }
}

/// Whether the reply is a `ClosePDU` with one of the reasons RFC 1227 defines.
fn parse_reply(bytes: &[u8]) -> AppProbeOutcome {
    presence(matches!(
        bytes,
        [CLOSE_PDU_TAG, 0x01, reason, ..] if *reason <= MAX_CLOSE_REASON
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_refused_open_is_smux() {
        // net-snmp refusing a peer it has no `smuxpeer` for: authenticationFailure.
        assert_eq!(
            parse_reply(&[0x41, 0x01, 0x05]),
            AppProbeOutcome::Answered { identity: None }
        );
        // goingDown.
        assert_eq!(
            parse_reply(&[0x41, 0x01, 0x00]),
            AppProbeOutcome::Answered { identity: None }
        );
    }

    #[test]
    fn silence_or_another_reply_is_not_smux() {
        for reply in [
            &[][..],
            // A reason RFC 1227 does not define.
            &[0x41, 0x01, 0x09][..],
            // An SNMP message: a universal SEQUENCE.
            &[0x30, 0x26, 0x02, 0x01, 0x01][..],
            &b"HTTP/1.1 400 Bad Request\r\n"[..],
        ] {
            assert_eq!(parse_reply(reply), AppProbeOutcome::NoAnswer, "{reply:?}");
        }
    }
}
