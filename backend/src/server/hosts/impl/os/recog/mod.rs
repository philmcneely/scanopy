//! Fingerprints from Rapid7 Recog, matched against strings a host emits about itself.
//!
//! Recog (BSD-2-Clause, `assets/recog/LICENSE`) publishes one XML database per kind of string: SNMP
//! `sysDescr`, SSH banners, HTTP `Server` headers, certificate subjects, page titles, favicon
//! hashes and so on. Each fingerprint is a regex plus the params a match yields (`os.family`,
//! `hw.vendor`, `hw.device`, `service.product`…), and carries `<example>` strings stating the
//! params they must yield, which the tests below run as a corpus. The files are vendored from
//! upstream commit `1714aaed118861b4567bb6754920b4b929f59380` (release 3.3.2); refresh by
//! replacing them, and the tests say what a refresh broke. One file is not vendored unmodified:
//! `favicons.xml` has every hash it shares with the OWASP Favicon Database removed, because those
//! are OWASP's CC BY data and Recog's BSD-2 notice does not grant them. Its header says how.
//!
//! What a match yields is an inference: the string was the host's own, but the OS, vendor, model,
//! device type or software is our reading of it. An OS needs [`MIN_OS_CERTAINTY`]; hardware and
//! software need [`MIN_SHARED_CERTAINTY`], and only families that name an operating system
//! ([`families::classify`]) produce one.

mod devices;
mod families;

use std::collections::HashMap;
use std::sync::LazyLock;

use quick_xml::XmlVersion;
use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;
use regex::{Regex, RegexBuilder};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use super::{HostOs, HostOsFamily};
use crate::server::hosts::r#impl::device_type::DeviceType;
use devices::{RecogDevice, classify_device};
use families::{RecogFamily, classify};

/// The lowest `os.certainty` a fingerprint may carry and still name an OS. Recog leaves it unset on
/// fingerprints it treats as certain; the ones it marks lower identify a product line from a
/// string that other operating systems also emit.
const MIN_OS_CERTAINTY: f32 = 0.9;

/// The lowest `hw.certainty` or `service.certainty` a fingerprint may carry and still name
/// hardware or software. Lower than the OS bar because of how Recog grades these: every favicon
/// match is 0.5, since an icon can be reused by a fork, while 0.0 to 0.4 mark a string Recog
/// itself calls vague ("Ubiquiti WAP (Vague)", a bare "login:" prompt).
const MIN_SHARED_CERTAINTY: f32 = 0.5;

/// One vendored Recog database: the kind of string it matches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum RecogDatabase {
    /// SNMP `sysDescr.0`.
    SnmpSysDescr,
    /// SNMP `sysObjectID.0`, in dotted form.
    SnmpSysObjectId,
    /// An SSH server's identification line, without the `SSH-2.0-` prefix.
    SshBanner,
    /// An HTTP `Server` header value.
    HttpServer,
    /// An HTTP `Server` header value, read for the platform Apache appends in parentheses.
    ApacheOs,
    /// One `key=value` pair from an mDNS `_device-info._tcp` TXT record.
    MdnsDeviceInfo,
    /// A TLS certificate's subject, as `CN=…,OU=…,O=…,C=…`.
    X509Subject,
    /// A TLS certificate's issuer, in the same form.
    X509Issuer,
    /// The text of an HTML page's `<title>`.
    HtmlTitle,
    /// An HTTP `WWW-Authenticate` header value.
    HttpWwwAuth,
    /// One HTTP `Set-Cookie` header value.
    HttpCookie,
    /// The lowercase hex MD5 of a server's `/favicon.ico`.
    Favicon,
    /// An FTP greeting, after the reply code.
    FtpBanner,
    /// A Telnet banner, trimmed.
    TelnetBanner,
    /// An RTSP `Server` header value.
    RtspServer,
}

impl RecogDatabase {
    fn xml(&self) -> &'static str {
        match self {
            Self::SnmpSysDescr => include_str!("../../../../../../assets/recog/snmp_sysdescr.xml"),
            Self::SnmpSysObjectId => {
                include_str!("../../../../../../assets/recog/snmp_sysobjid.xml")
            }
            Self::SshBanner => include_str!("../../../../../../assets/recog/ssh_banners.xml"),
            Self::HttpServer => include_str!("../../../../../../assets/recog/http_servers.xml"),
            Self::ApacheOs => include_str!("../../../../../../assets/recog/apache_os.xml"),
            Self::MdnsDeviceInfo => {
                include_str!("../../../../../../assets/recog/mdns_device-info_txt.xml")
            }
            Self::X509Subject => include_str!("../../../../../../assets/recog/x509_subjects.xml"),
            Self::X509Issuer => include_str!("../../../../../../assets/recog/x509_issuers.xml"),
            Self::HtmlTitle => include_str!("../../../../../../assets/recog/html_title.xml"),
            Self::HttpWwwAuth => include_str!("../../../../../../assets/recog/http_wwwauth.xml"),
            Self::HttpCookie => include_str!("../../../../../../assets/recog/http_cookies.xml"),
            Self::Favicon => include_str!("../../../../../../assets/recog/favicons.xml"),
            Self::FtpBanner => include_str!("../../../../../../assets/recog/ftp_banners.xml"),
            Self::TelnetBanner => {
                include_str!("../../../../../../assets/recog/telnet_banners.xml")
            }
            Self::RtspServer => include_str!("../../../../../../assets/recog/rtsp_servers.xml"),
        }
    }

    /// The fingerprints in this database, parsed and compiled on first use.
    fn fingerprints(&self) -> &'static [Fingerprint] {
        static DATABASES: LazyLock<HashMap<RecogDatabase, Vec<Fingerprint>>> = LazyLock::new(
            || {
                RecogDatabase::iter()
                    .map(|db| {
                        let fingerprints = parse(db.xml()).unwrap_or_else(|e| {
                            tracing::error!(database = ?db, error = %e, "Unreadable Recog database");
                            Vec::new()
                        });
                        (db, fingerprints)
                    })
                    .collect()
            },
        );
        DATABASES.get(self).map(Vec::as_slice).unwrap_or_default()
    }

    /// The OS `input` names: the first matching Recog fingerprint's, if it names one with enough
    /// certainty, and otherwise Scanopy's own entry for the string ([`super::supplement`]).
    ///
    /// First match only within Recog, which is its own semantics: the databases are ordered from
    /// specific to generic, so a later fingerprint matching too is the less precise reading. Recog
    /// is consulted first because its specific fingerprints name more (a distribution, a product)
    /// than the supplement's broad ones, which exist only to fill what Recog leaves unnamed.
    pub fn os(&self, input: &str) -> Option<HostOs> {
        self.identify(input)
            .and_then(|m| m.os)
            .or_else(|| super::supplement::os(*self, input.trim()))
    }

    /// Everything the first matching fingerprint names about the host, each part held to its
    /// certainty bar. `None` when nothing matches or the match names nothing usable.
    pub fn identify(&self, input: &str) -> Option<RecogMatch> {
        let input = input.trim();
        if input.is_empty() {
            return None;
        }
        self.fingerprints()
            .iter()
            .find_map(|fp| fp.extract(input))
            .map(|params| RecogMatch::from_params(&params))
            .filter(|m| !m.is_empty())
    }

    /// Every fixed `service.vendor` and `service.product` pair this database can name, for
    /// checking the software a service definition matches on exists.
    pub fn software(self) -> impl Iterator<Item = (&'static str, &'static str)> {
        self.fingerprints().iter().filter_map(|fp| {
            let fixed = |name: &str| {
                fp.params
                    .iter()
                    .find(|p| p.name == name && p.pos == 0)
                    .and_then(|p| p.value.as_deref())
            };
            Some((
                fixed("service.vendor").unwrap_or_default(),
                fixed("service.product")?,
            ))
        })
    }
}

/// The software a fingerprint names: Recog's `service.vendor` (empty when it gives none) and
/// `service.product`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RecogSoftware {
    pub vendor: String,
    pub product: String,
}

/// What one Recog match names about the host that emitted the string.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RecogMatch {
    pub os: Option<HostOs>,
    /// `hw.vendor`: who made the device.
    pub manufacturer: Option<String>,
    /// `hw.product`: the device's model.
    pub model: Option<String>,
    /// `hw.device`, where it names a class of device.
    pub device_type: Option<DeviceType>,
    /// `service.vendor` and `service.product`: the software answering on the port, such as a web
    /// application. A pair because Recog's product names repeat across vendors ("Dashboard" is
    /// Kubernetes', OpenStack's and InfluxData's).
    pub software: Option<RecogSoftware>,
}

impl RecogMatch {
    fn from_params(params: &HashMap<String, String>) -> Self {
        let certain = |key: &str| {
            params
                .get(key)
                .and_then(|c| c.parse::<f32>().ok())
                .is_none_or(|c| c >= MIN_SHARED_CERTAINTY)
        };
        let value = |key: &str| {
            params
                .get(key)
                .map(|v| v.trim())
                .filter(|v| !v.is_empty())
                .map(str::to_string)
        };
        let hardware = certain("hw.certainty");
        let device_type = value("hw.device").and_then(|d| match classify_device(&d) {
            RecogDevice::Type(device_type) => Some(device_type),
            RecogDevice::Generic | RecogDevice::Unlisted => None,
        });
        RecogMatch {
            os: os_from_params(params),
            manufacturer: value("hw.vendor").filter(|_| hardware),
            model: value("hw.product").filter(|_| hardware),
            device_type: device_type.filter(|_| hardware),
            software: value("service.product")
                .filter(|_| certain("service.certainty"))
                .map(|product| RecogSoftware {
                    vendor: value("service.vendor").unwrap_or_default(),
                    product,
                }),
        }
    }

    fn is_empty(&self) -> bool {
        *self == RecogMatch::default()
    }

    /// Whether the match names anything about the host itself, as opposed to only the software
    /// answering on one of its ports.
    pub fn names_host(&self) -> bool {
        self.os.is_some()
            || self.manufacturer.is_some()
            || self.model.is_some()
            || self.device_type.is_some()
    }
}

/// One `<fingerprint>`: a pattern and the params a match yields.
struct Fingerprint {
    pattern: Regex,
    params: Vec<Param>,
    #[cfg_attr(not(test), allow(dead_code))]
    examples: Vec<Example>,
}

/// A `<param>`: a fixed value (`pos="0"`), or capture group `pos` of the match.
struct Param {
    pos: usize,
    name: String,
    value: Option<String>,
}

/// An `<example>`: an input, and the params it states a match yields.
#[cfg_attr(not(test), allow(dead_code))]
struct Example {
    input: String,
    expected: Vec<(String, String)>,
    /// Base64-encoded or file-backed examples are not run.
    runnable: bool,
}

impl Fingerprint {
    /// The params this fingerprint yields for `input`, or `None` when it does not match.
    fn extract(&self, input: &str) -> Option<HashMap<String, String>> {
        let captures = self.pattern.captures(input)?;
        let mut params = HashMap::new();
        for param in &self.params {
            let value = match (&param.value, param.pos) {
                (Some(value), 0) => value.clone(),
                (_, pos) if pos > 0 => match captures.get(pos) {
                    Some(m) if !m.as_str().is_empty() => m.as_str().to_string(),
                    _ => continue,
                },
                _ => continue,
            };
            params.insert(param.name.clone(), value);
        }
        // Fixed values may name another param, as in `{hw.product} Firmware`.
        let snapshot = params.clone();
        for value in params.values_mut() {
            if value.contains('{') {
                for (name, captured) in &snapshot {
                    *value = value.replace(&format!("{{{name}}}"), captured);
                }
            }
        }
        Some(params)
    }
}

/// The OS a match's params name, when the fingerprint is certain enough and its family is an OS.
fn os_from_params(params: &HashMap<String, String>) -> Option<HostOs> {
    let certainty = params
        .get("os.certainty")
        .and_then(|c| c.parse::<f32>().ok());
    if certainty.is_some_and(|c| c < MIN_OS_CERTAINTY) {
        return None;
    }
    let family_text = params.get("os.family")?;
    let product = params.get("os.product").map(String::as_str);
    let RecogFamily::Os(family) = classify(family_text, product) else {
        return None;
    };

    let vendor = params.get("os.vendor").map(String::as_str);
    let meaningful = |s: &&str| !s.trim().is_empty() && *s != family_text;
    // The product names the OS more precisely than the family ("Windows Server 2008 R2"), except
    // where it only repeats it. On Linux the vendor is the distribution ("Ubuntu").
    let name = product
        .filter(meaningful)
        .or_else(|| {
            (family == HostOsFamily::Linux)
                .then_some(vendor)
                .flatten()
                .filter(meaningful)
        })
        .map(str::to_string);

    Some(HostOs {
        family,
        name,
        version: params.get("os.version").filter(|v| !v.is_empty()).cloned(),
        edition: None,
        codename: None,
        // Recog's Linux fingerprints capture the kernel release under this name.
        kernel_version: params
            .get("linux.kernel.version")
            .filter(|v| !v.is_empty())
            .cloned(),
    })
}

/// A Recog pattern as a Rust regex. Recog's patterns are Ruby regexes, where `^` and `$` always
/// match at line boundaries, so every pattern runs multi-line. Recog's `flags` map onto Ruby's:
/// `REG_ICASE` ignores case, and `REG_MULTILINE` and `REG_DOT_NEWLINE` let `.` match a newline.
fn compile(pattern: &str, flags: Option<&str>) -> Result<Regex, String> {
    let mut inline = String::from("m");
    for flag in flags.into_iter().flat_map(|f| f.split(',')).map(str::trim) {
        match flag {
            "REG_ICASE" => inline.push('i'),
            "REG_MULTILINE" | "REG_DOT_NEWLINE" | "REG_LINE_ANY_CRLF" => inline.push('s'),
            _ => {}
        }
    }
    // Ruby's `\w` and `\d` are ASCII and the regex crate's are Unicode, so a bounded repeat of one
    // (`[\w.-]{1,512}` in an FTP banner) compiles far larger here than Recog wrote it for.
    RegexBuilder::new(&format!("(?{inline}){pattern}"))
        .size_limit(64 << 20)
        .build()
        .map_err(|err| format!("pattern {pattern:?} does not compile: {err}"))
}

/// Parse one Recog XML database.
fn parse(xml: &str) -> Result<Vec<Fingerprint>, String> {
    let mut reader = Reader::from_str(xml);
    let mut fingerprints = Vec::new();
    let mut current: Option<Fingerprint> = None;
    let mut example: Option<Example> = None;

    loop {
        let event = reader.read_event().map_err(|e| e.to_string())?;
        match event {
            Event::Start(e) | Event::Empty(e) if e.name().as_ref() == "fingerprint" => {
                let attrs = attributes(&e)?;
                let pattern = attrs
                    .get("pattern")
                    .ok_or("fingerprint without a pattern")?;
                let pattern = compile(pattern, attrs.get("flags").map(String::as_str))?;
                current = Some(Fingerprint {
                    pattern,
                    params: Vec::new(),
                    examples: Vec::new(),
                });
            }
            Event::Start(e) | Event::Empty(e) if e.name().as_ref() == "param" => {
                let attrs = attributes(&e)?;
                if let Some(fp) = current.as_mut() {
                    fp.params.push(Param {
                        pos: attrs
                            .get("pos")
                            .and_then(|p| p.parse().ok())
                            .unwrap_or_default(),
                        name: attrs.get("name").cloned().unwrap_or_default(),
                        value: attrs.get("value").cloned(),
                    });
                }
            }
            Event::Start(e) if e.name().as_ref() == "example" => {
                let attrs = attributes(&e)?;
                let runnable = !attrs.contains_key("_encoding") && !attrs.contains_key("_filename");
                example = Some(Example {
                    input: String::new(),
                    expected: attrs
                        .into_iter()
                        .filter(|(k, _)| !k.starts_with('_'))
                        .collect(),
                    runnable,
                });
            }
            Event::Text(t) => {
                if let Some(ex) = example.as_mut() {
                    ex.input.push_str(&t.xml_content(XmlVersion::Implicit1_0));
                }
            }
            Event::GeneralRef(r) => {
                if let Some(ex) = example.as_mut() {
                    let resolved = match r.resolve_char_ref() {
                        Ok(Some(c)) => Some(c.to_string()),
                        _ => quick_xml::escape::resolve_predefined_entity(r.as_ref())
                            .map(str::to_string),
                    };
                    ex.input.push_str(&resolved.unwrap_or_default());
                }
            }
            Event::End(e) if e.name().as_ref() == "example" => {
                if let (Some(ex), Some(fp)) = (example.take(), current.as_mut()) {
                    fp.examples.push(ex);
                }
            }
            Event::End(e) if e.name().as_ref() == "fingerprint" => {
                fingerprints.extend(current.take());
            }
            Event::Eof => break,
            _ => {}
        }
    }
    Ok(fingerprints)
}

fn attributes(e: &BytesStart<'_>) -> Result<HashMap<String, String>, String> {
    e.attributes()
        .map(|attr| {
            let attr = attr.map_err(|err| err.to_string())?;
            let key = attr.key.as_ref().to_string();
            let value = attr
                .normalized_value(XmlVersion::Implicit1_0)
                .map_err(|err| err.to_string())?
                .into_owned();
            Ok((key, value))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_vendored_database_parses_and_every_pattern_compiles() {
        for db in RecogDatabase::iter() {
            let parsed = parse(db.xml());
            assert!(
                parsed.is_ok(),
                "{db:?}: {}",
                parsed.err().unwrap_or_default()
            );
            assert!(!parsed.unwrap().is_empty(), "{db:?} has no fingerprints");
        }
    }

    /// Recog's own corpus: every example matches its fingerprint and yields the params it states.
    #[test]
    fn every_example_yields_the_params_it_states() {
        let mut failures = Vec::new();
        for db in RecogDatabase::iter() {
            for fp in db.fingerprints() {
                for ex in fp.examples.iter().filter(|ex| ex.runnable) {
                    let Some(params) = fp.extract(&ex.input) else {
                        failures.push(format!("{db:?}: {:?} does not match", ex.input));
                        continue;
                    };
                    for (name, expected) in &ex.expected {
                        if params.get(name) != Some(expected) {
                            failures.push(format!(
                                "{db:?}: {:?} gave {name}={:?}, expected {expected:?}",
                                ex.input,
                                params.get(name)
                            ));
                        }
                    }
                }
            }
        }
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }

    /// A refresh that adds a family fails here until someone says whether it is an OS.
    #[test]
    fn every_vendored_family_is_classified() {
        let mut unlisted = std::collections::BTreeSet::new();
        for db in RecogDatabase::iter() {
            for fp in db.fingerprints() {
                let fixed = fp
                    .params
                    .iter()
                    .filter(|p| p.name == "os.family")
                    .filter_map(|p| p.value.clone());
                let captured = fp.examples.iter().flat_map(|ex| {
                    ex.expected
                        .iter()
                        .filter(|(k, _)| k == "os.family")
                        .map(|(_, v)| v.clone())
                        .collect::<Vec<_>>()
                });
                for family in fixed.chain(captured) {
                    if classify(&family, None) == RecogFamily::Unlisted {
                        unlisted.insert(family);
                    }
                }
            }
        }
        assert!(
            unlisted.is_empty(),
            "classify() or NOT_AN_OS must list: {unlisted:?}"
        );
    }

    /// A refresh that adds a device class fails here until someone maps it to a device type.
    #[test]
    fn every_vendored_device_is_classified() {
        let mut unlisted = std::collections::BTreeSet::new();
        for db in RecogDatabase::iter() {
            for fp in db.fingerprints() {
                let fixed = fp
                    .params
                    .iter()
                    .filter(|p| p.name == "hw.device")
                    .filter_map(|p| p.value.clone());
                for device in fixed {
                    if classify_device(&device) == RecogDevice::Unlisted {
                        unlisted.insert(device);
                    }
                }
            }
        }
        assert!(
            unlisted.is_empty(),
            "classify_device() must list: {unlisted:?}"
        );
    }

    #[test]
    fn a_certificate_subject_names_the_device() {
        let m = RecogDatabase::X509Subject
            .identify("CN=00:22:6b:ef:1e:d0,OU=RV042,O=Cisco-Linksys\\, LLC,L=Irvine,C=US")
            .expect("a Linksys RV042 certificate is fingerprinted");
        assert_eq!(m.model.as_deref(), Some("RV042"));
        assert!(m.manufacturer.is_some());
        assert!(m.device_type.is_some());
    }

    #[test]
    fn a_vague_fingerprint_names_no_hardware() {
        let params = HashMap::from([
            ("hw.vendor".to_string(), "Ubiquiti".to_string()),
            ("hw.device".to_string(), "WAP".to_string()),
            ("hw.certainty".to_string(), "0.25".to_string()),
        ]);
        assert!(RecogMatch::from_params(&params).is_empty());
    }

    #[test]
    fn case_insensitive_and_multiline_flags_apply() {
        let icase = compile("^foo$", Some("REG_ICASE")).unwrap();
        assert!(icase.is_match("FOO"));
        let dotall = compile("^a.b$", Some("REG_MULTILINE")).unwrap();
        assert!(dotall.is_match("a\nb"));
        // Ruby anchors match at every line, which multi-line banners depend on.
        assert!(
            compile("^login:$", None)
                .unwrap()
                .is_match("Welcome\nlogin:")
        );
    }

    #[test]
    fn a_distribution_banner_names_the_distribution() {
        let os = RecogDatabase::SshBanner
            .os("OpenSSH_6.6.1p1 Ubuntu-2ubuntu2.13")
            .expect("an Ubuntu banner names an OS");
        assert_eq!(os.family, HostOsFamily::Linux);
        assert_eq!(os.name.as_deref(), Some("Ubuntu"));
    }

    #[test]
    fn a_banner_that_names_no_os_yields_none() {
        assert_eq!(RecogDatabase::SshBanner.os("OpenSSH_9.6"), None);
        assert_eq!(RecogDatabase::HttpServer.os("nginx"), None);
    }

    #[test]
    fn a_hardware_line_yields_no_os() {
        assert_eq!(classify("JetDirect", None), RecogFamily::NotAnOs);
    }

    #[test]
    fn cisco_ios_products_resolve_to_their_own_family() {
        let os = RecogDatabase::SnmpSysDescr
            .os("Cisco IOS Software [Dublin], Catalyst L3 Switch Software (CAT9K_LITE_IOSXE), Version 17.12.5, RELEASE SOFTWARE (fc5)")
            .expect("an IOS XE sysDescr names an OS");
        assert_eq!(os.family, HostOsFamily::CiscoIosXe);
        assert_eq!(os.version.as_deref(), Some("17.12.5"));
        assert_eq!(
            classify("IOS", Some("IOS-XE")),
            RecogFamily::Os(HostOsFamily::CiscoIosXe)
        );
        assert_eq!(classify("iOS", None), RecogFamily::Os(HostOsFamily::Ios));
    }

    #[test]
    fn a_low_certainty_fingerprint_names_no_os() {
        let params = HashMap::from([
            ("os.family".to_string(), "Linux".to_string()),
            ("os.certainty".to_string(), "0.5".to_string()),
        ]);
        assert_eq!(os_from_params(&params), None);
    }

    #[test]
    fn a_device_info_model_names_macos() {
        let os = RecogDatabase::MdnsDeviceInfo
            .os("model=MacBookPro18,3")
            .expect("a Mac model names an OS");
        assert_eq!(os.family, HostOsFamily::MacOs);
    }
}
