//! Telnet detection by option negotiation.
//!
//! RFC 854's command escape is `IAC` (255), and a Telnet server opens by negotiating options —
//! `IAC DO`, `IAC WILL` — before any login prompt. That byte is what identifies the protocol: a
//! plain text banner could come from anything, but 255 as the first byte of a stream that is
//! otherwise ASCII is Telnet's own framing. Some servers clear the screen before negotiating (a
//! Brother NC-8200h print server opens with `ESC[2J ESC[1;1f`), so ANSI control sequences ahead of
//! the first `IAC` are skipped.

use anyhow::Error;
use async_trait::async_trait;

use crate::daemon::utils::app_probe::{
    AppProbe, AppProbeOutcome, DeviceIdentity, ProbeContext, read_greeting,
};
use crate::server::hosts::r#impl::os::recog::RecogDatabase;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::r#impl::patterns::ClientProbe;
use crate::server::shared::attribution::AttributeSource;

/// Interpret As Command.
const IAC: u8 = 255;
/// The three command bytes that can follow `IAC` in an opening negotiation.
const WILL: u8 = 251;
const DO: u8 = 253;
const DONT: u8 = 254;
const WONT: u8 = 252;
/// Opens an ANSI escape sequence.
const ESC: u8 = 0x1B;

pub struct TelnetProbe;

#[async_trait]
impl AppProbe for TelnetProbe {
    fn client_probe(&self) -> ClientProbe {
        ClientProbe::Telnet
    }

    async fn run(&self, ctx: &ProbeContext, port: PortType) -> Result<AppProbeOutcome, Error> {
        Ok(parse_negotiation(&read_greeting(ctx, port, 256).await))
    }
}

/// Whether the opening bytes are Telnet option negotiation, and what any banner text among them
/// names.
///
/// `IAC` followed by one of the four negotiation commands, rather than `IAC` alone: a lone 255 is a
/// plausible first byte of binary junk, and the command that must follow it is what makes this a
/// protocol match instead of a coincidence.
fn parse_negotiation(bytes: &[u8]) -> AppProbeOutcome {
    let bytes = skip_ansi_sequences(bytes);
    if !(bytes.first() == Some(&IAC) && matches!(bytes.get(1), Some(&WILL | &WONT | &DO | &DONT))) {
        return AppProbeOutcome::NoAnswer;
    }
    let banner = banner_text(bytes);
    AppProbeOutcome::Answered {
        identity: DeviceIdentity::matched(
            RecogDatabase::TelnetBanner,
            &banner,
            AttributeSource::TelnetBannerMatch,
        ),
    }
}

/// The text a server sent alongside its negotiation, with every Telnet command removed: `IAC`
/// plus a command and option byte, or a subnegotiation from `IAC SB` to `IAC SE`. A server that
/// prints its banner before waiting for our replies sends it here; one that waits sends none.
fn banner_text(bytes: &[u8]) -> String {
    const SB: u8 = 250;
    const SE: u8 = 240;
    let mut text = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match &bytes[i..] {
            [IAC, SB, ..] => {
                i = bytes[i..]
                    .windows(2)
                    .position(|w| w == [IAC, SE])
                    .map_or(bytes.len(), |end| i + end + 2);
            }
            [IAC, IAC, ..] => {
                text.push(IAC);
                i += 2;
            }
            [IAC, WILL | WONT | DO | DONT, ..] => i += 3,
            [IAC, ..] => i += 2,
            [b, ..] => {
                text.push(*b);
                i += 1;
            }
            [] => break,
        }
    }
    String::from_utf8_lossy(&text).trim().to_string()
}

/// `bytes` past any leading ANSI CSI sequences (`ESC [`, parameter bytes, one final byte in
/// `0x40..=0x7E`). An unterminated sequence leaves nothing, so it cannot match.
fn skip_ansi_sequences(mut bytes: &[u8]) -> &[u8] {
    while let [ESC, b'[', rest @ ..] = bytes {
        match rest.iter().position(|b| (0x40..=0x7E).contains(b)) {
            Some(end) => bytes = &rest[end + 1..],
            None => return &[],
        }
    }
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_negotiation_is_telnet() {
        for greeting in [
            // IAC DO TERMINAL-TYPE, IAC WILL ECHO
            &[IAC, DO, 24, IAC, WILL, 1][..],
            &[IAC, WILL, 3, IAC, DO, 31][..],
            &[IAC, WONT, 1][..],
            // Brother NC-8200h: clear screen, cursor home, then IAC WILL ECHO.
            &[
                ESC, b'[', b'2', b'J', ESC, b'[', b'1', b';', b'1', b'f', IAC, WILL, 1,
            ][..],
        ] {
            assert_eq!(
                parse_negotiation(greeting),
                AppProbeOutcome::Answered { identity: None }
            );
        }
    }

    #[test]
    fn a_banner_sent_with_the_negotiation_names_the_device() {
        let mut greeting = vec![IAC, WILL, 1, IAC, SB_FOR_TEST, 24, 1, IAC, 240];
        greeting
            .extend_from_slice(b"Grandstream HT812 Command Shell Copyright 2006-2017\r\nPassword:");
        let AppProbeOutcome::Answered { identity } = parse_negotiation(&greeting) else {
            panic!("option negotiation answers");
        };
        let (matched, source) = *identity
            .and_then(|i| i.matched)
            .expect("the Grandstream banner is fingerprinted");
        assert_eq!(matched.model.as_deref(), Some("HT812"));
        assert_eq!(source, AttributeSource::TelnetBannerMatch);
    }

    const SB_FOR_TEST: u8 = 250;

    #[test]
    fn silence_a_text_banner_or_a_lone_iac_is_not_telnet() {
        for greeting in [
            &[][..],
            b"login: ",
            b"SSH-2.0-OpenSSH_9.6\r\n",
            // A lone 255 followed by something that is not a negotiation command.
            &[IAC, 0x00, 0x01][..],
            &[IAC][..],
            // A screen clear followed by text, or a sequence that never terminates.
            &[ESC, b'[', b'2', b'J', b'l', b'o', b'g', b'i', b'n'][..],
            &[ESC, b'[', b'2', IAC, WILL, 1][..],
        ] {
            assert_eq!(parse_negotiation(greeting), AppProbeOutcome::NoAnswer);
        }
    }
}
