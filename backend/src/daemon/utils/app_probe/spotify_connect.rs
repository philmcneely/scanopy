//! Spotify Connect's `getInfo`, read at the port and path the device's mDNS advertisement names.
//!
//! Every Spotify Connect receiver answers `GET {CPath}?action=getInfo` with the same JSON, and that
//! JSON states the device's brand and model (`brandDisplayName`, `modelDisplayName`) whoever made
//! it. Neither the port nor the path is fixed: the SRV record of `_spotify-connect._tcp` names the
//! port and its TXT `CPath` the path, and both differ by vendor (Bose `8200` `/zc`, Sonos `1400`
//! `/spotifyzc`).
//!
//! Not an [`AppProbe`](super::AppProbe) for that reason. The trait's `port()` is fixed, and is
//! also the port a definition's pattern scans; this probe learns its port from the advertisement,
//! so it runs only where the mDNS browse reached the host and only on the port the host named.

use serde::Deserialize;

use crate::daemon::discovery::service::network::mdns::DnsSdHost;
use crate::daemon::utils::app_probe::{
    AppProbeResult, DeviceIdentity, ProbeContext, identity_field, request_exact,
};
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::r#impl::patterns::{ClientProbe, DnsSdServiceType};
use crate::server::shared::attribution::AttributeSource;

/// The path Spotify's embedded SDK serves `getInfo` on, for an advertisement that omits `CPath`.
const DEFAULT_PATH: &str = "/zc";

/// Upper bound on the reply. A `getInfo` body is under 1 KB; the bound only stops a peer that
/// streams without closing from holding the read past its timeout's worth of data.
const MAX_REPLY: usize = 16 * 1024;

/// The fields of a `getInfo` reply this probe reads. `spotifyError` and `deviceID` are required:
/// both are present in every implementation's reply, and a JSON document lacking them is some
/// other service on the port.
#[derive(Debug, Deserialize)]
struct GetInfo {
    #[serde(rename = "spotifyError")]
    _spotify_error: i64,
    #[serde(rename = "deviceID")]
    _device_id: String,
    #[serde(rename = "brandDisplayName")]
    brand_display_name: Option<String>,
    #[serde(rename = "modelDisplayName")]
    model_display_name: Option<String>,
}

/// Ask the host for `getInfo` where its Spotify Connect advertisement says to, and read its brand
/// and model from the reply.
///
/// `None` when the host advertises no Spotify Connect port, or the reply is not `getInfo`'s.
pub async fn probe_advertised(ctx: &ProbeContext, dns_sd: &DnsSdHost) -> Option<AppProbeResult> {
    let port = dns_sd.advertised_port(DnsSdServiceType::SPOTIFY_CONNECT)?;
    // The same gate `select_probes` applies to every registered probe: a raw print port prints
    // whatever it receives.
    if !ctx.probe_raw_socket_ports && port.is_raw_socket() {
        return None;
    }
    let path = dns_sd
        .services
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case(DnsSdServiceType::SPOTIFY_CONNECT))
        .and_then(|(_, txt)| txt.get("cpath"))
        .map(String::as_str)
        .filter(|path| path.starts_with('/'))
        .unwrap_or(DEFAULT_PATH);

    let reply = request_exact(ctx, port, &request(ctx, port, path), MAX_REPLY).await;
    let info = parse_reply(&reply)?;

    let source = AttributeSource::Probe(ClientProbe::SpotifyConnect);
    Some(AppProbeResult {
        port,
        client_probe: Some(ClientProbe::SpotifyConnect),
        identity: Some(DeviceIdentity {
            manufacturer: identity_field(info.brand_display_name, source),
            model: identity_field(info.model_display_name, source),
            serial_number: None,
            firmware_revision: None,
            os: None,
        }),
    })
}

/// `Connection: close` so the device ends the reply, which is what lets the read stop before its
/// timeout.
fn request(ctx: &ProbeContext, port: PortType, path: &str) -> Vec<u8> {
    format!(
        "GET {path}?action=getInfo HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        ctx.target(port.number())
    )
    .into_bytes()
}

/// The `getInfo` document in a complete `200` reply.
fn parse_reply(bytes: &[u8]) -> Option<GetInfo> {
    let mut headers = [httparse::EMPTY_HEADER; 32];
    let mut response = httparse::Response::new(&mut headers);
    let httparse::Status::Complete(head_len) = response.parse(bytes).ok()? else {
        return None;
    };
    if response.code != Some(200) {
        return None;
    }
    serde_json::from_slice(&bytes[head_len..]).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    use async_trait::async_trait;

    /// Captured from a Bose Music Amplifier on 8200, identifiers replaced.
    const BOSE: &[u8] = b"HTTP/1.1 200 OK\r\nServer: eSDK\r\nContent-Type: application/json\r\n\
Connection: close\r\n\r\n\
{\"status\":101,\"statusString\":\"OK\",\"spotifyError\":0,\"responseSource\":\"eSDK\",\
\"version\":\"2.10.0\",\"deviceID\":\"0000000000000000000000000000000000000001\",\"remoteName\":\"\",\
\"deviceType\":\"SPEAKER\",\"libraryVersion\":\"3.211.126-g38bc1087\",\"brandDisplayName\":\"Bose\",\
\"modelDisplayName\":\"Bose_Music_Amplifier\",\"resolverVersion\": \"0\",\"groupStatus\": \"NONE\",\
\"productID\":100002,\"supported_drm_media_formats\": [\n{\"drm\": 1, \"formats\": 65606}\n],\
\"aliases\": [\n{\n\"name\": \"Bose amp\",\"id\": \"1\",\"isGroup\": \"false\"\n}\n]}";

    /// Captured from a Sonos Port on 1400, identifiers replaced. No `responseSource`, which is
    /// why the probe does not require it.
    const SONOS: &[u8] = b"HTTP/1.1 200 OK\r\n\
Content-type: application/json; charset=utf-8\r\nServer: Linux UPnP/1.0 Sonos/97.2-81220 (ZPS23)\r\n\
Connection: close\r\n\r\n\
{\"status\":101,\"statusString\":\"OK\",\"spotifyError\":0,\"version\":\"2.9.0\",\
\"deviceID\":\"0000000000000000000000000000000000000002\",\"deviceType\":\"AVR\",\
\"modelDisplayName\":\"Port\",\"brandDisplayName\":\"Sonos\",\"remoteName\":\"Living room\"}";

    #[test]
    fn brand_and_model_are_read_from_each_vendors_reply() {
        for (reply, brand, model) in [
            (BOSE, "Bose", "Bose_Music_Amplifier"),
            (SONOS, "Sonos", "Port"),
        ] {
            let info = parse_reply(reply).expect("a getInfo reply");
            assert_eq!(info.brand_display_name.as_deref(), Some(brand));
            assert_eq!(info.model_display_name.as_deref(), Some(model));
        }
    }

    #[test]
    fn json_that_is_not_get_info_or_a_non_200_is_refused() {
        for reply in [
            &b""[..],
            &b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"status\":\"ok\"}"[..],
            &b"HTTP/1.1 404 Not Found\r\nContent-Type: application/json\r\n\r\n\
{\"spotifyError\":0,\"deviceID\":\"x\"}"[..],
            &b"HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<html></html>"[..],
        ] {
            assert!(
                parse_reply(reply).is_none(),
                "{}",
                String::from_utf8_lossy(reply)
            );
        }
    }

    /// Answers `reply` to a request for `path` on `port`, and a 404 to anything else.
    #[derive(Debug)]
    struct Receiver {
        port: u16,
        path: &'static str,
        reply: &'static [u8],
    }

    #[async_trait]
    impl crate::daemon::utils::app_probe::ProbeTunnel for Receiver {
        async fn open(
            &self,
            port: u16,
        ) -> std::io::Result<Box<dyn crate::daemon::utils::app_probe::ProbeStream>> {
            use tokio::io::{AsyncReadExt, AsyncWriteExt};
            let expected = format!("GET {}?action=getInfo ", self.path);
            let reply: &'static [u8] = if port == self.port {
                self.reply
            } else {
                b"HTTP/1.1 404 Not Found\r\n\r\n"
            };
            let (ours, mut theirs) = tokio::io::duplex(4096);
            tokio::spawn(async move {
                let mut request = [0u8; 1024];
                let Ok(n) = theirs.read(&mut request).await else {
                    return;
                };
                let line = String::from_utf8_lossy(&request[..n]).to_string();
                let reply: &[u8] = if line.starts_with(&expected) {
                    reply
                } else {
                    b"HTTP/1.1 404 Not Found\r\n\r\n"
                };
                let _ = theirs.write_all(reply).await;
            });
            Ok(Box::new(ours))
        }
    }

    fn context(receiver: Receiver) -> ProbeContext {
        ProbeContext {
            ip: "192.0.2.1".parse().unwrap(),
            subnet_cidr: "192.0.2.0/24".parse().unwrap(),
            is_gateway: false,
            cancel: tokio_util::sync::CancellationToken::new(),
            scan_controller: crate::daemon::utils::scanner::ScanConcurrencyController::new(1),
            probe_raw_socket_ports: false,
            port_map: Default::default(),
            tunnel: Some(std::sync::Arc::new(receiver)),
        }
    }

    fn advertising(port: u16, cpath: Option<&str>) -> DnsSdHost {
        let txt: BTreeMap<String, String> = cpath
            .map(|p| ("cpath".to_string(), p.to_string()))
            .into_iter()
            .collect();
        DnsSdHost {
            services: BTreeMap::from([(DnsSdServiceType::SPOTIFY_CONNECT.to_string(), txt)]),
            ports: BTreeMap::from([(DnsSdServiceType::SPOTIFY_CONNECT.to_string(), port)]),
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn the_probe_asks_at_the_advertised_port_and_path() {
        let ctx = context(Receiver {
            port: 1400,
            path: "/spotifyzc",
            reply: SONOS,
        });
        let result = probe_advertised(&ctx, &advertising(1400, Some("/spotifyzc")))
            .await
            .expect("the receiver answered");
        assert_eq!(result.port, PortType::new_tcp(1400));
        assert_eq!(result.client_probe, Some(ClientProbe::SpotifyConnect));
        let identity = result.identity.expect("an identity");
        assert_eq!(identity.manufacturer.unwrap().value().0, "Sonos");
        assert_eq!(identity.model.unwrap().value().0, "Port");
    }

    #[tokio::test]
    async fn an_advertisement_without_cpath_uses_the_sdk_default() {
        let ctx = context(Receiver {
            port: 8200,
            path: "/zc",
            reply: BOSE,
        });
        assert!(
            probe_advertised(&ctx, &advertising(8200, None))
                .await
                .is_some()
        );
    }

    #[tokio::test]
    async fn no_advertised_port_means_no_request() {
        let ctx = context(Receiver {
            port: 8200,
            path: "/zc",
            reply: BOSE,
        });
        let mut host = advertising(8200, None);
        host.ports.clear();
        assert!(probe_advertised(&ctx, &host).await.is_none());
    }
}
