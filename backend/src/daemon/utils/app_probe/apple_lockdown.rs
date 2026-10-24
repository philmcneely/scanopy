//! iOS and iPadOS detection by the lockdown service on 62078.
//!
//! `lockdownd` is the service a Mac or PC pairs and syncs with over Wi-Fi. It speaks property
//! lists framed by a 4-byte big-endian length, and answers a `QueryType` request, which needs no
//! pairing, with its own type: `com.apple.mobile.lockdown`. That string is the protocol naming
//! itself. Nothing is changed on the device: `QueryType` only asks the service what it is.

use anyhow::Error;
use async_trait::async_trait;

use crate::daemon::utils::app_probe::{
    AppProbe, AppProbeOutcome, ProbeContext, presence, request_response,
};
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::r#impl::patterns::ClientProbe;

/// The type a lockdown service reports for itself.
const LOCKDOWN_TYPE: &[u8] = b"com.apple.mobile.lockdown";

const QUERY_TYPE: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \
\"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n\
<plist version=\"1.0\"><dict><key>Label</key><string>scanopy</string>\
<key>Request</key><string>QueryType</string></dict></plist>\n";

pub struct AppleLockdownProbe;

#[async_trait]
impl AppProbe for AppleLockdownProbe {
    fn port(&self) -> PortType {
        PortType::new_tcp(62078)
    }

    fn client_probe(&self) -> Option<ClientProbe> {
        Some(ClientProbe::AppleLockdown)
    }

    async fn run(&self, ctx: &ProbeContext) -> Result<AppProbeOutcome, Error> {
        let reply = request_response(ctx, self.port(), &query_type(), 2048).await;
        Ok(parse_reply(&reply))
    }
}

/// `QueryType`, framed by its length as lockdown expects.
fn query_type() -> Vec<u8> {
    let mut framed = (QUERY_TYPE.len() as u32).to_be_bytes().to_vec();
    framed.extend_from_slice(QUERY_TYPE.as_bytes());
    framed
}

/// Whether the reply is a framed property list naming the lockdown service.
fn parse_reply(bytes: &[u8]) -> AppProbeOutcome {
    let Some(body) = bytes.get(4..) else {
        return AppProbeOutcome::NoAnswer;
    };
    presence(
        body.starts_with(b"<?xml")
            && body
                .windows(LOCKDOWN_TYPE.len())
                .any(|window| window == LOCKDOWN_TYPE),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// What an iPad answered on 62078.
    const IPAD: &[u8] = b"\x00\x00\x01\x29<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \
\"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n<plist version=\"1.0\">\n<dict>\n\
\t<key>Request</key>\n\t<string>QueryType</string>\n\t<key>Type</key>\n\
\t<string>com.apple.mobile.lockdown</string>\n</dict>\n</plist>\n";

    #[test]
    fn a_lockdown_reply_names_itself() {
        assert_eq!(
            parse_reply(IPAD),
            AppProbeOutcome::Answered { identity: None }
        );
    }

    #[test]
    fn other_replies_are_not_lockdown() {
        for reply in [
            &b""[..],
            b"\x00\x00",
            b"HTTP/1.1 400 Bad Request\r\n\r\n",
            b"\x00\x00\x00\x10<?xml version=\"1.0\"?><plist></plist>",
        ] {
            assert_eq!(parse_reply(reply), AppProbeOutcome::NoAnswer);
        }
    }

    #[test]
    fn the_query_is_framed_by_its_length() {
        let framed = query_type();
        let declared = u32::from_be_bytes(framed[..4].try_into().unwrap()) as usize;
        assert_eq!(declared, framed.len() - 4);
    }
}
