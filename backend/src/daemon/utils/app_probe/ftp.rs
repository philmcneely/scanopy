//! FTP detection by the greeting RFC 959 requires before any command is accepted.
//!
//! A server announces itself with a `220` reply as soon as the control connection opens. Rejections
//! count too: `421 Service not available` is a server refusing this client, and it is still an FTP
//! server, which is the claim the definition makes.

use anyhow::Error;
use async_trait::async_trait;

use crate::daemon::utils::app_probe::{
    AppProbe, AppProbeOutcome, DeviceIdentity, ProbeContext, read_greeting,
};
use crate::server::hosts::r#impl::os::recog::RecogDatabase;
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::r#impl::patterns::ClientProbe;
use crate::server::shared::attribution::AttributeSource;

pub struct FtpProbe;

#[async_trait]
impl AppProbe for FtpProbe {
    fn client_probe(&self) -> ClientProbe {
        ClientProbe::Ftp
    }

    async fn run(&self, ctx: &ProbeContext, port: PortType) -> Result<AppProbeOutcome, Error> {
        Ok(parse_greeting(&read_greeting(ctx, port, 512).await))
    }
}

/// Whether the greeting is an FTP reply, and what the server software's banner names.
///
/// A reply line is three digits followed by a space or a hyphen (the hyphen marking a multi-line
/// reply). Only the two codes a connection can legitimately open with are accepted: `220` ready and
/// `421` refusing. Accepting any 3-digit code would match SMTP and NNTP, which greet the same way.
///
/// The text after the first line's code is what Recog's FTP fingerprints read: embedded servers
/// name the device (`TiMOS-C-13.0.R9 cpm/hops64 ALCATEL SR 7750`).
fn parse_greeting(bytes: &[u8]) -> AppProbeOutcome {
    let opens_with =
        |code: &[u8]| bytes.starts_with(code) && matches!(bytes.get(3), Some(b' ') | Some(b'-'));
    if !(opens_with(b"220") || opens_with(b"421")) {
        return AppProbeOutcome::NoAnswer;
    }
    let greeting = String::from_utf8_lossy(&bytes[4..]);
    let first_line = greeting.lines().next().unwrap_or_default();
    AppProbeOutcome::Answered {
        identity: DeviceIdentity::matched(
            RecogDatabase::FtpBanner,
            first_line,
            AttributeSource::FtpBannerMatch,
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_ready_or_refusing_greeting_is_ftp() {
        for greeting in [
            &b"220 ProFTPD Server (Debian) [::ffff:172.17.0.2]\r\n"[..],
            &b"220-FileZilla Server 1.7.3\r\n220 Please visit\r\n"[..],
            &b"421 Service not available, closing control connection\r\n"[..],
        ] {
            assert!(matches!(
                parse_greeting(greeting),
                AppProbeOutcome::Answered { .. }
            ));
        }
    }

    #[test]
    fn an_embedded_server_greeting_names_the_device() {
        let AppProbeOutcome::Answered { identity } =
            parse_greeting(b"220 TiMOS-C-13.0.R9 cpm/hops64 ALCATEL SR 7750 Copyright (\r\n")
        else {
            panic!("a 220 greeting answers");
        };
        let (matched, source) = *identity
            .and_then(|i| i.matched)
            .expect("the TiMOS greeting is fingerprinted");
        assert_eq!(matched.model.as_deref(), Some("SR 7750"));
        assert_eq!(source, AttributeSource::FtpBannerMatch);
    }

    #[test]
    fn silence_or_another_greeting_protocol_is_not_ftp() {
        for greeting in [
            &b""[..],
            &b"SSH-2.0-OpenSSH_9.6\r\n"[..],
            // A 3-digit code we do not accept: SMTP and NNTP greet in the same shape.
            &b"200 news.example.com NNRP Service Ready\r\n"[..],
            &b"2200 not a reply code\r\n"[..],
            &b"220\r\n"[..],
        ] {
            assert_eq!(parse_greeting(greeting), AppProbeOutcome::NoAnswer);
        }
    }
}
