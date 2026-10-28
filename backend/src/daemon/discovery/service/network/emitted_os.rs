//! What a scanned host sent of its own accord, read against Recog: its web servers' `Server`
//! headers, page titles, login challenges, cookies and favicons, the certificates its TLS ports
//! presented, and its mDNS `_device-info._tcp` and `_airplay._tcp` records.
//!
//! Each string is matched once per port into a [`PortFingerprint`], which service matching reads
//! (`Pattern::Recog`) and which [`offer_emitted`] offers to the host through
//! [`HostData::offer_match`]. That combines them: two naming a different OS family, maker, model
//! or device class leave that field empty. Banners (SSH, FTP, Telnet, RTSP) arrive the same way,
//! from their probes.

use std::collections::BTreeMap;

use crate::daemon::discovery::service::network::mdns::types::DnsSdHost;
use crate::daemon::discovery::service::ops::HostData;
use crate::daemon::utils::app_probe::tls::PortCertificate;
use crate::server::hosts::r#impl::os::recog::{RecogDatabase, RecogMatch};
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::r#impl::endpoints::EndpointResponse;
use crate::server::shared::attribution::AttributeSource;

/// The DNS-SD service whose TXT record describes the device itself.
const DEVICE_INFO_SERVICE: &str = "_device-info._tcp";
/// The AirPlay receiver service, whose TXT record carries the device's model.
const AIRPLAY_SERVICE: &str = "_airplay._tcp";

/// What one string a port emitted names, and which Recog database named it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortFingerprint {
    pub port: PortType,
    pub database: RecogDatabase,
    pub matched: RecogMatch,
}

impl PortFingerprint {
    /// The attribute source a value this fingerprint names is recorded under.
    pub fn source(&self) -> AttributeSource {
        match self.database {
            RecogDatabase::HttpServer | RecogDatabase::ApacheOs => AttributeSource::HttpServerMatch,
            RecogDatabase::X509Subject => AttributeSource::X509SubjectMatch,
            RecogDatabase::X509Issuer => AttributeSource::X509IssuerMatch,
            RecogDatabase::HtmlTitle => AttributeSource::HtmlTitleMatch,
            RecogDatabase::HttpWwwAuth => AttributeSource::HttpWwwAuthMatch,
            RecogDatabase::HttpCookie => AttributeSource::HttpCookieMatch,
            RecogDatabase::Favicon => AttributeSource::FaviconMatch,
            RecogDatabase::FtpBanner => AttributeSource::FtpBannerMatch,
            RecogDatabase::TelnetBanner => AttributeSource::TelnetBannerMatch,
            RecogDatabase::RtspServer => AttributeSource::RtspServerMatch,
            RecogDatabase::SshBanner => AttributeSource::SshBannerMatch,
            RecogDatabase::SnmpSysDescr => AttributeSource::SysDescrMatch,
            RecogDatabase::SnmpSysObjectId => AttributeSource::SysObjectIdMatch,
            RecogDatabase::MdnsDeviceInfo => AttributeSource::DnsSdDeviceInfoMatch,
        }
    }
}

/// Everything `database` names in `input`: the Recog match, with Scanopy's own OS entry for the
/// string where Recog names no OS ([`RecogDatabase::os`]).
fn recognize(database: RecogDatabase, input: &str) -> Option<RecogMatch> {
    let mut matched = database.identify(input).unwrap_or_default();
    if matched.os.is_none() {
        matched.os = database.os(input);
    }
    (matched != RecogMatch::default()).then_some(matched)
}

/// Fingerprint every string the host's ports emitted: each HTTP response's `Server` header, page
/// title, `WWW-Authenticate` challenge and cookies, each TLS certificate's subject and issuer, and
/// each favicon's hash. In that order per port, which is the order a service definition's
/// `Pattern::Recog` takes the first match from: a header or title names the software more surely
/// than an icon a fork may share.
pub fn port_fingerprints(
    endpoint_responses: &[EndpointResponse],
    certificates: &[PortCertificate],
    favicons: &[(PortType, String)],
) -> Vec<PortFingerprint> {
    let mut fingerprints = Vec::new();
    let mut add = |port: PortType, database: RecogDatabase, input: &str| {
        if let Some(matched) = recognize(database, input) {
            fingerprints.push(PortFingerprint {
                port,
                database,
                matched,
            });
        }
    };

    for response in endpoint_responses {
        let port = response.endpoint.port_type;
        if let Some(server) = response.headers.get("server") {
            // `apache_os` reads the platform Apache appends in parentheses, which `http_servers`
            // leaves to it.
            match recognize(RecogDatabase::HttpServer, server) {
                Some(_) => add(port, RecogDatabase::HttpServer, server),
                None => add(port, RecogDatabase::ApacheOs, server),
            }
        }
        if let Some(title) = html_title(&response.body) {
            add(port, RecogDatabase::HtmlTitle, &title);
        }
        if let Some(challenge) = response.headers.get("www-authenticate") {
            add(port, RecogDatabase::HttpWwwAuth, challenge);
        }
        for cookie in &response.set_cookies {
            add(port, RecogDatabase::HttpCookie, cookie);
        }
    }
    for certificate in certificates {
        add(
            certificate.port,
            RecogDatabase::X509Subject,
            &certificate.subject,
        );
        add(
            certificate.port,
            RecogDatabase::X509Issuer,
            &certificate.issuer,
        );
    }
    for (port, hash) in favicons {
        add(*port, RecogDatabase::Favicon, hash);
    }

    fingerprints.dedup();
    fingerprints
}

/// The text of an HTML document's `<title>`, entity-decoded and with its whitespace collapsed, as
/// Recog's title fingerprints are written.
fn html_title(body: &str) -> Option<String> {
    let lower = body.to_ascii_lowercase();
    let open = lower.find("<title")?;
    let start = open + lower[open..].find('>')? + 1;
    let end = start + lower[start..].find("</title")?;
    let raw = &body[start..end];
    let decoded = quick_xml::escape::unescape(raw)
        .map(|text| text.into_owned())
        .unwrap_or_else(|_| raw.to_string());
    let title = decoded.split_whitespace().collect::<Vec<_>>().join(" ");
    (!title.is_empty()).then_some(title)
}

/// Offer what every port fingerprint and the device-info and AirPlay records name about the host.
pub(super) fn offer_emitted(
    host_data: &mut HostData,
    fingerprints: &[PortFingerprint],
    dns_sd: Option<&DnsSdHost>,
) {
    for fingerprint in fingerprints {
        if fingerprint.matched.names_host() {
            host_data.offer_match(fingerprint.matched.clone(), fingerprint.source());
        }
    }

    let Some(dns_sd) = dns_sd else {
        return;
    };
    // The device-info record describes the device itself; read every pair, which is how Recog's
    // database is keyed (`model=…`, `osxvers=…`).
    if let Some(txt) = service_txt(dns_sd, DEVICE_INFO_SERVICE) {
        let pairs = txt.iter().map(|(key, value)| format!("{key}={value}"));
        offer_mdns_matches(host_data, pairs, AttributeSource::DnsSdDeviceInfoMatch, txt);
    }
    // A Mac that shares nothing publishes no device-info record but still advertises AirPlay, and
    // that record's `model` is the same Apple model identifier (`model=Mac17,2` from a MacBook Pro,
    // captured 2026-10-02). Only `model` is read: the other AirPlay keys are protocol details.
    if let Some(txt) = service_txt(dns_sd, AIRPLAY_SERVICE) {
        let pairs = txt.get("model").map(|model| format!("model={model}"));
        offer_mdns_matches(host_data, pairs, AttributeSource::DnsSdAirPlayMatch, txt);
    }
}

/// The TXT pairs a host advertised for one DNS-SD service type, if it advertised it.
fn service_txt<'a>(dns_sd: &'a DnsSdHost, service: &str) -> Option<&'a BTreeMap<String, String>> {
    dns_sd
        .services
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case(service))
        .map(|(_, txt)| txt)
}

/// Offer what each `key=value` pair names. A record that arrived and named nothing is logged with
/// what it held, so a device the fingerprints do not cover can be identified from the log rather
/// than guessed at.
fn offer_mdns_matches(
    host_data: &mut HostData,
    pairs: impl IntoIterator<Item = String>,
    source: AttributeSource,
    txt: &BTreeMap<String, String>,
) {
    let mut named = false;
    for pair in pairs {
        if let Some(matched) = recognize(RecogDatabase::MdnsDeviceInfo, &pair)
            && matched.names_host()
        {
            named = true;
            host_data.offer_match(matched, source);
        }
    }
    if !named {
        tracing::debug!(
            address = ?host_data.ip_addresses.first().map(|ip| ip.base.ip_address),
            %source,
            txt = ?txt,
            "mDNS record named nothing"
        );
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};

    use super::*;
    use crate::server::hosts::r#impl::base::{Host, HostBase};
    use crate::server::hosts::r#impl::os::HostOsFamily;
    use crate::server::ports::r#impl::base::PortType;
    use crate::server::services::r#impl::endpoints::{ApplicationProtocol, Endpoint};

    fn offer(host_data: &mut HostData, responses: &[EndpointResponse], dns_sd: Option<&DnsSdHost>) {
        offer_emitted(host_data, &port_fingerprints(responses, &[], &[]), dns_sd);
    }

    fn page(title: &str) -> EndpointResponse {
        EndpointResponse {
            body: format!("<html><head><TITLE>\n  {title}\n</title></head></html>"),
            headers: HashMap::new(),
            ..server_header("")
        }
    }

    #[test]
    fn a_page_title_names_the_device_its_maker_and_its_class() {
        let mut host_data = host();
        offer(&mut host_data, &[page("Hanwha Techwin")], None);
        let base = &host_data.host.base;
        assert_eq!(
            base.manufacturer.as_ref().map(|m| m.value().0.as_str()),
            Some("Hanwha Techwin")
        );
        assert_eq!(
            base.matched_device_type.as_ref().map(|d| d.value().0),
            Some(crate::server::hosts::r#impl::device_type::DeviceType::Camera)
        );
        assert_eq!(
            base.manufacturer.as_ref().unwrap().source(),
            AttributeSource::HtmlTitleMatch
        );
    }

    #[test]
    fn a_certificate_and_a_title_naming_different_makers_leave_no_maker() {
        let mut host_data = host();
        let certificate = PortCertificate {
            port: PortType::Https,
            subject: "CN=00:22:6b:ef:1e:d0,OU=RV042,O=Cisco-Linksys\\, LLC,L=Irvine,C=US"
                .to_string(),
            issuer: String::new(),
        };
        let fingerprints = port_fingerprints(
            &[page("Hanwha Techwin")],
            std::slice::from_ref(&certificate),
            &[],
        );
        offer_emitted(&mut host_data, &fingerprints, None);
        assert_eq!(host_data.host.base.manufacturer, None);
    }

    #[test]
    fn a_favicon_hash_is_fingerprinted_on_its_port() {
        // A Rapid7 hash from the vendored file's first fingerprint.
        let fingerprints = port_fingerprints(
            &[],
            &[],
            &[(
                PortType::Http,
                "4297c114f263c206ed12aaff4b0c7a50".to_string(),
            )],
        );
        assert_eq!(fingerprints.len(), 1);
        assert_eq!(fingerprints[0].database, RecogDatabase::Favicon);
        assert_eq!(fingerprints[0].source(), AttributeSource::FaviconMatch);
    }

    #[test]
    fn a_title_reads_through_entities_and_whitespace() {
        assert_eq!(
            html_title("<title>\n  Router &amp; Modem  </title>").as_deref(),
            Some("Router & Modem")
        );
        assert_eq!(html_title("<html>no title</html>"), None);
        assert_eq!(html_title("<title>   </title>"), None);
    }

    fn host() -> HostData {
        HostData::new(
            Host::new(HostBase::default()),
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        )
    }

    fn server_header(value: &str) -> EndpointResponse {
        EndpointResponse {
            endpoint: Endpoint {
                protocol: ApplicationProtocol::Http,
                ip: None,
                port_type: PortType::Http,
                path: "/".to_string(),
            },
            body: String::new(),
            over_tls: false,
            set_cookies: Vec::new(),
            headers: HashMap::from([("server".to_string(), value.to_string())]),
            status: 200,
        }
    }

    fn device_info(model: &str) -> DnsSdHost {
        DnsSdHost {
            services: BTreeMap::from([(
                DEVICE_INFO_SERVICE.to_string(),
                BTreeMap::from([("model".to_string(), model.to_string())]),
            )]),
            ..Default::default()
        }
    }

    /// An AirPlay record as a Mac or a speaker announces it: `model` plus protocol keys.
    fn airplay(model: &str) -> DnsSdHost {
        DnsSdHost {
            services: BTreeMap::from([(
                AIRPLAY_SERVICE.to_string(),
                BTreeMap::from([
                    ("model".to_string(), model.to_string()),
                    ("srcvers".to_string(), "980.77.5".to_string()),
                ]),
            )]),
            ..Default::default()
        }
    }

    /// The record a MacBook Pro advertised on a live LAN, with no device-info record beside it.
    #[test]
    fn a_mac_that_only_advertises_airplay_names_macos() {
        let mut host_data = host();
        offer(&mut host_data, &[], Some(&airplay("Mac17,2")));
        assert_eq!(family(&host_data), Some(HostOsFamily::MacOs));
        assert_eq!(
            host_data.host.base.os.as_ref().unwrap().source(),
            AttributeSource::DnsSdAirPlayMatch
        );
    }

    #[test]
    fn an_airplay_speaker_model_names_no_os() {
        let mut host_data = host();
        offer(&mut host_data, &[], Some(&airplay("Five")));
        assert_eq!(family(&host_data), None);
    }

    fn family(host_data: &HostData) -> Option<HostOsFamily> {
        host_data
            .host
            .base
            .os
            .as_ref()
            .map(|os| os.value().0.family)
    }

    #[test]
    fn a_server_header_names_the_platform() {
        let mut host_data = host();
        offer(&mut host_data, &[server_header("Microsoft-IIS/8.5")], None);
        assert_eq!(family(&host_data), Some(HostOsFamily::Windows));
        assert_eq!(
            host_data.host.base.os.as_ref().unwrap().source(),
            AttributeSource::HttpServerMatch
        );
    }

    #[test]
    fn a_mac_model_in_device_info_names_macos() {
        let mut host_data = host();
        offer(&mut host_data, &[], Some(&device_info("MacBookPro18,3")));
        assert_eq!(family(&host_data), Some(HostOsFamily::MacOs));
    }

    #[test]
    fn matches_that_disagree_leave_no_os_and_a_later_one_cannot_restore_it() {
        let mut host_data = host();
        offer(
            &mut host_data,
            &[server_header("Microsoft-IIS/8.5")],
            Some(&device_info("MacBookPro18,3")),
        );
        assert_eq!(family(&host_data), None);

        offer(&mut host_data, &[server_header("Microsoft-IIS/8.5")], None);
        assert_eq!(family(&host_data), None);
    }

    #[test]
    fn a_header_that_names_no_os_writes_none() {
        let mut host_data = host();
        offer(&mut host_data, &[server_header("nginx")], None);
        assert_eq!(family(&host_data), None);
    }
}
