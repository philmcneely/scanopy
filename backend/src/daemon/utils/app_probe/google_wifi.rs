//! Google Wifi / Nest Wifi identity, read from the on-device `GET /api/v1/status` API.
//!
//! Every Google Wifi and Nest Wifi unit — router and mesh point alike — serves an unauthenticated
//! status API on port 80. Its JSON states the firmware (`software.softwareVersion`) and the board
//! (`system.hardwareId`, e.g. `GALE` for first-gen Google Wifi), which no other signal on the host
//! carries: the MAC OUI gives only the vendor, and the endpoint scan matches bodies by substring,
//! so it cannot pull a field out of the document.
//!
//! An `AppProbe` rather than a `Pattern::Endpoint` for that reason — the match the endpoint scan
//! already does would see "a page containing `leaf`", but recording the model and firmware needs
//! the document parsed. The probe both drives the `Google Nest Repeater` match (via
//! `ClientProbe::GoogleWifi`) and enriches the host, so the API is fetched once.

use anyhow::Error;
use async_trait::async_trait;
use serde::Deserialize;

use crate::daemon::utils::app_probe::{
    AppProbe, AppProbeOutcome, DeviceIdentity, ProbeContext, identity_field, request_response,
};
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::r#impl::patterns::ClientProbe;
use crate::server::shared::attribution::AttributeSource;

/// Upper bound on the reply. A status document is ~1.3 KB; the bound only stops a peer that streams
/// without closing from holding the read past its timeout's worth of data.
const MAX_REPLY: usize = 16 * 1024;

/// The fields of `/api/v1/status` this probe reads. `setup_state`, `software` and `system` are
/// required: a JSON document lacking any of them is some other service's `/api/v1/status`, not a
/// Google Wifi unit's.
#[derive(Debug, Deserialize)]
struct Status {
    #[serde(rename = "setupState")]
    setup_state: String,
    software: Software,
    system: System,
}

#[derive(Debug, Deserialize)]
struct Software {
    #[serde(rename = "softwareVersion")]
    software_version: String,
}

#[derive(Debug, Deserialize)]
struct System {
    /// Board, with a per-unit suffix: `GALE C2I-A2A-A3C-A4I-E87`. The first token is the model.
    #[serde(rename = "hardwareId")]
    hardware_id: Option<String>,
    #[serde(rename = "modelId")]
    model_id: Option<String>,
}

pub struct GoogleWifiProbe;

#[async_trait]
impl AppProbe for GoogleWifiProbe {
    fn client_probe(&self) -> ClientProbe {
        ClientProbe::GoogleWifi
    }

    async fn run(&self, ctx: &ProbeContext, port: PortType) -> Result<AppProbeOutcome, Error> {
        let reply = request_response(ctx, port, &request(ctx, port), MAX_REPLY).await;
        Ok(match parse_reply(&reply) {
            Some(identity) => AppProbeOutcome::Answered {
                identity: Some(identity),
            },
            None => AppProbeOutcome::NoAnswer,
        })
    }
}

/// `Connection: close` so the device ends the reply; the unit answers `Keep-Alive` otherwise and
/// holds the socket until the read's timeout.
fn request(ctx: &ProbeContext, port: PortType) -> Vec<u8> {
    format!(
        "GET /api/v1/status HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        ctx.target(port.number())
    )
    .into_bytes()
}

/// The identity in a `200` reply whose body is the Google Wifi status document, or `None` for any
/// other reply.
fn parse_reply(bytes: &[u8]) -> Option<DeviceIdentity> {
    let mut headers = [httparse::EMPTY_HEADER; 32];
    let mut response = httparse::Response::new(&mut headers);
    let httparse::Status::Complete(head_len) = response.parse(bytes).ok()? else {
        return None;
    };
    if response.code != Some(200) {
        return None;
    }
    let rest = &bytes[head_len..];
    let declared = response
        .headers
        .iter()
        .find(|h| h.name.eq_ignore_ascii_case("content-length"))
        .map(|h| {
            std::str::from_utf8(h.value)
                .ok()?
                .trim()
                .parse::<usize>()
                .ok()
        });
    let body = match declared {
        Some(Some(len)) => rest.get(..len)?,
        // A header that does not parse as a length is a malformed reply.
        Some(None) => return None,
        None => rest,
    };

    let status: Status = serde_json::from_slice(body).ok()?;
    // The shape gate: the required fields can deserialize as empty strings, which is not the API
    // answering, so reject those too.
    if status.setup_state.trim().is_empty() || status.software.software_version.trim().is_empty() {
        return None;
    }

    // The board codename, the honest model string: the device offers no marketing name here.
    let model = status
        .system
        .hardware_id
        .as_deref()
        .and_then(|id| id.split_whitespace().next())
        .map(str::to_string)
        .or(status.system.model_id);

    let probe = AttributeSource::Probe(ClientProbe::GoogleWifi);
    Some(DeviceIdentity {
        matched: None,
        // Not read off a field — the document never says "Google". Answering Google's own status
        // API is the evidence, so it carries the probe's own source, not a lower inference rung.
        manufacturer: identity_field(Some("Google".to_string()), probe),
        model: identity_field(model, probe),
        serial_number: None,
        firmware_revision: identity_field(Some(status.software.software_version), probe),
        os: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Captured from a Google Wifi point (`192.168.86.30`), trimmed to the fields read.
    const STATUS: &[u8] = b"{\
\"dns\":{\"mode\":\"automatic\",\"servers\":[]},\
\"setupState\":\"GWIFI_OOBE_COMPLETE\",\
\"software\":{\"softwareVersion\":\"14150.376.32\",\"updateChannel\":\"stable-channel\"},\
\"system\":{\"countryCode\":\"us\",\"groupRole\":\"leaf\",\
\"hardwareId\":\"GALE C2I-A2A-A3C-A4I-E87\",\"modelId\":\"ACc3d\"},\
\"wan\":{\"online\":true}}";

    /// Wrap a JSON body in a `200` reply with its `Content-Length`.
    fn reply_200(body: &[u8]) -> Vec<u8> {
        let mut out = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json; charset=utf-8\r\n\
             Connection: Keep-Alive\r\nContent-Length: {}\r\n\r\n",
            body.len()
        )
        .into_bytes();
        out.extend_from_slice(body);
        out
    }

    #[test]
    fn manufacturer_model_and_firmware_are_read_from_the_status_api() {
        let identity = parse_reply(&reply_200(STATUS)).expect("a Google Wifi status reply");
        assert_eq!(identity.manufacturer.unwrap().value().0, "Google");
        // The board codename, the first token of `hardwareId`, not its per-unit suffix.
        assert_eq!(identity.model.unwrap().value().0, "GALE");
        assert_eq!(
            identity.firmware_revision.unwrap().value().0,
            "14150.376.32"
        );
        assert!(identity.serial_number.is_none());
    }

    #[test]
    fn the_body_ends_at_its_declared_length() {
        // A Keep-Alive unit can leave bytes on the socket after the body; the parse stops at the
        // declared length rather than feeding the trailing bytes to serde.
        let mut stream = reply_200(STATUS);
        stream.extend_from_slice(b"GET /api/v1/status HTTP/1.1\r\n\r\n");
        let identity = parse_reply(&stream).expect("the first reply parses");
        assert_eq!(identity.model.unwrap().value().0, "GALE");
    }

    #[test]
    fn model_falls_back_to_model_id_without_a_hardware_id() {
        let body = b"{\"setupState\":\"GWIFI_OOBE_COMPLETE\",\
\"software\":{\"softwareVersion\":\"1.2.3\"},\"system\":{\"modelId\":\"ACc3d\"}}";
        let identity = parse_reply(&reply_200(body)).expect("a status reply");
        assert_eq!(identity.model.unwrap().value().0, "ACc3d");
    }

    #[test]
    fn json_that_is_not_the_status_api_or_a_non_200_is_refused() {
        for reply in [
            &b""[..],
            // 200, but not the status document.
            &reply_200(b"{\"status\":\"ok\"}")[..],
            // The status shape, but the required fields are empty.
            &reply_200(
                b"{\"setupState\":\"\",\"software\":{\"softwareVersion\":\"\"},\"system\":{}}",
            )[..],
            // A 404, the stock answer every other path gets.
            &b"HTTP/1.1 404 Not Found\r\nContent-Length: 9\r\n\r\nNot Found"[..],
            // Not JSON.
            &b"HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<html></html>"[..],
        ] {
            assert!(
                parse_reply(reply).is_none(),
                "{}",
                String::from_utf8_lossy(reply)
            );
        }
    }
}
