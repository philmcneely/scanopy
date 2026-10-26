//! iOS and iPadOS detection, and the device's model and OS, from the lockdown service on 62078.
//!
//! `lockdownd` is the service a Mac or PC pairs and syncs with over Wi-Fi. It speaks property
//! lists framed by a 4-byte big-endian length. Two requests need no pairing:
//!
//! - `QueryType`, answered with the service's own type, `com.apple.mobile.lockdown`. That string
//!   is the protocol naming itself, and is what confirms the port.
//! - `GetValue` for `ProductType` (`iPad14,1`, `iPhone15,2`, `AppleTV11,1`) and `ProductVersion`
//!   (`27.0.1`). The product prefix names the OS: an iPad runs iPadOS, an Apple TV tvOS.
//!
//! Nothing is changed on the device: both requests only read.

use anyhow::Error;
use async_trait::async_trait;

use tokio::io::AsyncWriteExt;

use crate::daemon::utils::app_probe::{
    AppProbe, AppProbeOutcome, DeviceIdentity, ProbeContext, ProbeStream, RESPONSE_READ_TIMEOUT,
    identity_field, presence, read_reply,
};
use crate::daemon::utils::scanner::SCAN_TIMEOUT;
use crate::server::hosts::r#impl::attributes::HostOsValue;
use crate::server::hosts::r#impl::os::{HostOs, HostOsFamily};
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::r#impl::patterns::ClientProbe;
use crate::server::shared::attribution::{AttributeSource, Attributed};

/// The type a lockdown service reports for itself.
const LOCKDOWN_TYPE: &str = "com.apple.mobile.lockdown";

/// How much of a reply to read. Lockdown's replies to these requests are a few hundred bytes.
const MAX_REPLY: usize = 2048;

pub struct AppleLockdownProbe;

#[async_trait]
impl AppProbe for AppleLockdownProbe {
    fn port(&self) -> PortType {
        PortType::new_tcp(62078)
    }

    fn client_probe(&self) -> Option<ClientProbe> {
        Some(ClientProbe::AppleLockdown)
    }

    /// All three requests on one connection. An iPhone in Wi-Fi power save answers in short waking
    /// windows: in a live scan it answered `QueryType` and slept through the `GetValue`
    /// connections that followed, so the reads ride the connection that already got an answer.
    async fn run(&self, ctx: &ProbeContext) -> Result<AppProbeOutcome, Error> {
        let Some(mut stream) = ctx.connect(self.port().number()).await else {
            return Ok(AppProbeOutcome::NoAnswer);
        };
        let reply = exchange(&mut stream, &request("QueryType", None)).await;
        if parse_query_type(&reply) == AppProbeOutcome::NoAnswer {
            return Ok(AppProbeOutcome::NoAnswer);
        }
        let product_type =
            string_value(&exchange(&mut stream, &request("GetValue", Some("ProductType"))).await);
        let product_version = string_value(
            &exchange(&mut stream, &request("GetValue", Some("ProductVersion"))).await,
        );
        Ok(AppProbeOutcome::Answered {
            identity: identity(product_type, product_version),
        })
    }
}

/// Send one request on an open lockdown connection and read its reply.
async fn exchange(stream: &mut Box<dyn ProbeStream>, request: &[u8]) -> Vec<u8> {
    if tokio::time::timeout(SCAN_TIMEOUT, stream.write_all(request))
        .await
        .is_err()
    {
        return Vec::new();
    }
    read_reply(stream, MAX_REPLY, RESPONSE_READ_TIMEOUT).await
}

/// A lockdown request, framed by its length.
fn request(name: &str, key: Option<&str>) -> Vec<u8> {
    let key = key
        .map(|key| format!("<key>Key</key><string>{key}</string>"))
        .unwrap_or_default();
    let plist = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \
         \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n\
         <plist version=\"1.0\"><dict><key>Label</key><string>scanopy</string>{key}\
         <key>Request</key><string>{name}</string></dict></plist>\n"
    );
    let mut framed = (plist.len() as u32).to_be_bytes().to_vec();
    framed.extend_from_slice(plist.as_bytes());
    framed
}

/// The property list in a framed reply.
fn body(bytes: &[u8]) -> Option<&str> {
    let body = std::str::from_utf8(bytes.get(4..)?).ok()?;
    body.starts_with("<?xml").then_some(body)
}

/// Whether the reply is a property list naming the lockdown service.
fn parse_query_type(bytes: &[u8]) -> AppProbeOutcome {
    presence(body(bytes).is_some_and(|body| body.contains(LOCKDOWN_TYPE)))
}

/// The `<string>` that follows `<key>Value</key>` in a `GetValue` reply. An error reply carries
/// `<key>Error</key>` and no value.
fn string_value(bytes: &[u8]) -> Option<String> {
    let body = body(bytes)?;
    let after_key = &body[body.find("<key>Value</key>")? + "<key>Value</key>".len()..];
    let start = after_key.find("<string>")? + "<string>".len();
    let end = after_key[start..].find("</string>")?;
    Some(after_key[start..start + end].trim().to_string()).filter(|v| !v.is_empty())
}

/// The OS a product type runs, from its prefix.
fn os_family(product_type: &str) -> Option<HostOsFamily> {
    [
        ("iPhone", HostOsFamily::Ios),
        ("iPod", HostOsFamily::Ios),
        ("iPad", HostOsFamily::IpadOs),
        ("AppleTV", HostOsFamily::TvOs),
        ("AudioAccessory", HostOsFamily::AudioOs),
    ]
    .into_iter()
    .find_map(|(prefix, family)| product_type.starts_with(prefix).then_some(family))
}

fn identity(
    product_type: Option<String>,
    product_version: Option<String>,
) -> Option<DeviceIdentity> {
    let source = AttributeSource::Probe(ClientProbe::AppleLockdown);
    let os = product_type.as_deref().and_then(os_family).map(|family| {
        Box::new(Attributed::new(
            HostOsValue(HostOs {
                version: product_version,
                ..HostOs::family_only(family)
            }),
            source,
        ))
    });
    let model = identity_field(product_type, source);
    if model.is_none() && os.is_none() {
        return None;
    }
    Some(DeviceIdentity {
        manufacturer: identity_field(Some("Apple".to_string()), source),
        model,
        serial_number: None,
        firmware_revision: None,
        os,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Replies from an iPad on 62078, framed as lockdown sends them.
    fn framed(body: &str) -> Vec<u8> {
        let mut bytes = (body.len() as u32).to_be_bytes().to_vec();
        bytes.extend_from_slice(body.as_bytes());
        bytes
    }

    const PLIST_HEAD: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!DOCTYPE plist PUBLIC \
         \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n\
         <plist version=\"1.0\">\n<dict>\n";

    fn get_value_reply(key: &str, value: &str) -> Vec<u8> {
        framed(&format!(
            "{PLIST_HEAD}\t<key>Key</key>\n\t<string>{key}</string>\n\t<key>Request</key>\n\
             \t<string>GetValue</string>\n\t<key>Value</key>\n\t<string>{value}</string>\n\
             </dict>\n</plist>\n"
        ))
    }

    #[test]
    fn a_lockdown_reply_names_itself() {
        let reply = framed(&format!(
            "{PLIST_HEAD}\t<key>Request</key>\n\t<string>QueryType</string>\n\t<key>Type</key>\n\
             \t<string>com.apple.mobile.lockdown</string>\n</dict>\n</plist>\n"
        ));
        assert_eq!(
            parse_query_type(&reply),
            AppProbeOutcome::Answered { identity: None }
        );
        for other in [
            &b""[..],
            b"\x00\x00",
            b"HTTP/1.1 400 Bad Request\r\n\r\n",
            b"\x00\x00\x00\x10<?xml version=\"1.0\"?><plist></plist>",
        ] {
            assert_eq!(parse_query_type(other), AppProbeOutcome::NoAnswer);
        }
    }

    #[test]
    fn get_value_reads_the_value_and_not_the_key() {
        assert_eq!(
            string_value(&get_value_reply("ProductType", "iPad14,1")).as_deref(),
            Some("iPad14,1")
        );
        let refused = framed(&format!(
            "{PLIST_HEAD}\t<key>Error</key>\n\t<string>MissingValue</string>\n\t<key>Key</key>\n\
             \t<string>DeviceClass</string>\n</dict>\n</plist>\n"
        ));
        assert_eq!(string_value(&refused), None);
    }

    /// The iPad's answers become model iPad14,1 and iPadOS 27.0.1, which types it a tablet; an
    /// iPhone's become iOS and a phone.
    #[test]
    fn the_product_type_names_the_os() {
        use crate::server::hosts::r#impl::device_type::DeviceType;

        let ipad = identity(Some("iPad14,1".into()), Some("27.0.1".into())).expect("identity");
        let os = ipad.os.expect("an OS").value().0.clone();
        assert_eq!(os.family, HostOsFamily::IpadOs);
        assert_eq!(os.version.as_deref(), Some("27.0.1"));
        assert_eq!(os.device_type(), Some(DeviceType::Tablet));

        let iphone = identity(Some("iPhone15,2".into()), None).expect("identity");
        let os = iphone.os.expect("an OS").value().0.clone();
        assert_eq!(os.device_type(), Some(DeviceType::Mobile));

        // A product type nobody listed still names the model, and claims no OS.
        let unknown = identity(Some("Watch7,1".into()), None).expect("identity");
        assert!(unknown.os.is_none() && unknown.model.is_some());
        assert!(identity(None, None).is_none());
    }

    #[test]
    fn requests_are_framed_by_their_length() {
        for framed in [
            request("QueryType", None),
            request("GetValue", Some("ProductType")),
        ] {
            let declared = u32::from_be_bytes(framed[..4].try_into().unwrap()) as usize;
            assert_eq!(declared, framed.len() - 4);
        }
    }
}
