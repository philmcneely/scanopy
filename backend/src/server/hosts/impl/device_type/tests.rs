use super::*;
use crate::server::hosts::r#impl::attributes::{HostHostnameValue, HostOsValue};
use crate::server::hosts::r#impl::os::{HostOs, HostOsFamily};
use crate::server::ip_addresses::r#impl::base::{MacEvidence, MacEvidenceValue};
use crate::server::services::definitions::{
    airplay_receiver::AirPlayReceiver, docker_daemon::Docker, kerberos::Kerberos,
    open_ports::OpenPorts, pf_sense::PfSense, plex::Plex, postgresql::PostgreSQL, proxmox::Proxmox,
    samba::Samba, snmp::Snmp, spotify_connect::SpotifyConnect, true_nas::TrueNAS,
};
use crate::server::services::r#impl::base::ServiceBase;
use crate::server::services::r#impl::virtualization::{
    DockerVirtualization, ServiceVirtualization,
};
use crate::server::shared::attribution::{AttributeSource, Attributed};
use crate::tests::ip_address;
use uuid::Uuid;

fn host_with_os(os: Option<HostOs>) -> Host {
    let mut host = crate::server::shared::types::examples::host();
    host.base.os = os.map(|os| Attributed::new(HostOsValue(os), AttributeSource::SshBannerMatch));
    host
}

fn service(definition: Box<dyn ServiceDefinition>) -> Service {
    Service {
        base: ServiceBase {
            service_definition: definition,
            ..Default::default()
        },
        ..Default::default()
    }
}

fn containerized(definition: Box<dyn ServiceDefinition>) -> Service {
    let mut service = service(definition);
    service.base.virtualization_metadata =
        Some(ServiceVirtualization::Docker(DockerVirtualization {
            state: Default::default(),
            container_name: None,
            container_id: None,
            compose_project: None,
        }));
    service
}

/// Software in a container says what the container does, not what the machine is: it ranks
/// below the OS, and makes a host a server only when nothing else names it.
#[test]
fn containers_rank_below_the_os_and_above_nothing() {
    let os = |family| Some(HostOs::family_only(family));
    let db_in_docker = [
        service(Box::new(Docker)),
        containerized(Box::new(PostgreSQL)),
    ];

    // A Mac running Postgres in Docker is a workstation.
    assert_eq!(
        host_with_os(os(HostOsFamily::MacOs)).device_type(&db_in_docker, []),
        DeviceType::Workstation
    );
    // A Linux host gives no OS device type, so the same containers make it a server, and so does the
    // runtime alone.
    assert_eq!(
        host_with_os(os(HostOsFamily::Linux)).device_type(&db_in_docker, []),
        DeviceType::Server
    );
    assert_eq!(
        host_with_os(os(HostOsFamily::Linux)).device_type(&db_in_docker[..1], []),
        DeviceType::Server
    );
    // Software installed on the host itself implies only `Server`, which yields to the OS too.
    assert_eq!(
        host_with_os(os(HostOsFamily::MacOs)).device_type(&[service(Box::new(PostgreSQL))], []),
        DeviceType::Workstation
    );
    // And a host's own device type outranks its containers.
    let nas_with_containers = [
        service(Box::new(TrueNAS)),
        containerized(Box::new(PostgreSQL)),
    ];
    assert_eq!(
        host_with_os(os(HostOsFamily::Linux)).device_type(&nas_with_containers, []),
        DeviceType::Nas
    );
}

/// The ladder from most specific evidence to none: a service's specific device type beats every
/// lower-ranked one and the OS, and the `Server` a service implies yields to the OS.
#[test]
fn services_outrank_each_other_by_precedence_then_the_os() {
    let macos = Some(HostOs::family_only(HostOsFamily::MacOs));

    let firewall_and_nas = [service(Box::new(TrueNAS)), service(Box::new(PfSense))];
    assert_eq!(
        host_with_os(None).device_type(&firewall_and_nas, []),
        DeviceType::Firewall
    );

    let hypervisor_with_db = [service(Box::new(PostgreSQL)), service(Box::new(Proxmox))];
    assert_eq!(
        host_with_os(None).device_type(&hypervisor_with_db, []),
        DeviceType::Hypervisor
    );

    // A service's specific device type wins over the OS device type.
    let nas = [service(Box::new(TrueNAS))];
    assert_eq!(
        host_with_os(macos.clone()).device_type(&nas, []),
        DeviceType::Nas
    );

    // `Server` from a service yields to the OS: a Mac sharing files over SMB is a workstation.
    let plumbing = [service(Box::new(Snmp)), service(Box::new(Samba))];
    assert_eq!(
        host_with_os(macos.clone()).device_type(&plumbing[..1], []),
        DeviceType::Workstation
    );
    assert_eq!(
        host_with_os(macos).device_type(&plumbing, []),
        DeviceType::Workstation
    );
    // With no OS or name to yield to, the same service still makes it a server.
    assert_eq!(
        host_with_os(None).device_type(&plumbing, []),
        DeviceType::Server
    );

    // The SMB probe's Windows reading names no edition, so it implies nothing, and a Windows
    // server's applications still make it a server.
    let windows_by_version = Some(HostOs {
        version: Some("10.0.17763".to_string()),
        ..HostOs::family_only(HostOsFamily::Windows)
    });
    assert_eq!(
        host_with_os(windows_by_version).device_type(&[service(Box::new(PostgreSQL))], []),
        DeviceType::Server
    );
}

/// A streaming receiver protocol suggests a media device, and decides only a host nothing else
/// names: desktops and servers run Spotify Connect and AirPlay too.
#[test]
fn a_streaming_receiver_protocol_decides_only_a_host_nothing_else_names() {
    // An Onkyo AV receiver from a user's scan: an unclaimed port 5000, Spotify Connect and
    // AirPlay, no OS reading and no product name.
    let receiver = [
        service(Box::new(OpenPorts)),
        service(Box::new(SpotifyConnect)),
        service(Box::new(AirPlayReceiver)),
    ];
    assert_eq!(
        host_with_os(None).device_type(&receiver, []),
        DeviceType::NetworkAudio
    );

    // A Mac receiving AirPlay is still a workstation.
    assert_eq!(
        host_with_os(Some(HostOs::family_only(HostOsFamily::MacOs)))
            .device_type(&[service(Box::new(AirPlayReceiver))], []),
        DeviceType::Workstation
    );
    // A Linux box running Plex and a Spotify Connect client is a server, and so is a Docker host
    // running one.
    let linux = || host_with_os(Some(HostOs::family_only(HostOsFamily::Linux)));
    assert_eq!(
        linux().device_type(
            &[service(Box::new(Plex)), service(Box::new(SpotifyConnect))],
            []
        ),
        DeviceType::Server
    );
    assert_eq!(
        linux().device_type(
            &[service(Box::new(Docker)), service(Box::new(SpotifyConnect))],
            []
        ),
        DeviceType::Server
    );
}

#[test]
fn a_host_with_no_evidence_is_unidentified() {
    let linux = Some(HostOs::family_only(HostOsFamily::Linux));
    let snmp = [service(Box::new(Snmp))];
    assert_eq!(
        host_with_os(linux).device_type(&snmp, []),
        DeviceType::Unidentified
    );
    assert_eq!(
        host_with_os(None).device_type(&[], []),
        DeviceType::Unidentified
    );
}

/// A product name in the hostname identifies a host nothing else does, and yields to every
/// stronger rung.
#[test]
fn a_product_name_identifies_a_host_below_services_and_the_os() {
    let named = |hostname: &str| {
        let mut host = host_with_os(None);
        host.base.hostname = Some(Attributed::new(
            HostHostnameValue(hostname.to_string()),
            AttributeSource::ReverseDns,
        ));
        host
    };
    assert_eq!(
        named("Mayas-iPhone.local").device_type(&[], []),
        DeviceType::Mobile
    );
    assert_eq!(named("ipad13.lan").device_type(&[], []), DeviceType::Tablet);
    assert_eq!(
        named("iphoneclone.lan").device_type(&[], []),
        DeviceType::Unidentified
    );
    assert_eq!(
        named("mac.lan").device_type(&[], []),
        DeviceType::Unidentified
    );
    // Hostnames from a live scan with randomized MACs, where the name is the only evidence.
    assert_eq!(
        named("josep-s-phone.lan").device_type(&[], []),
        DeviceType::Mobile
    );
    assert_eq!(
        named("xiaomi-pad-mini.lan").device_type(&[], []),
        DeviceType::Tablet
    );
    assert_eq!(
        named("nest-doorbell-battery.lan").device_type(&[], []),
        DeviceType::Camera
    );
    assert_eq!(
        named("schlage.lan").device_type(&[], []),
        DeviceType::AccessControl
    );
    // A desk phone and a bare "pad" name no handheld.
    assert_eq!(
        named("lobby-ip-phone").device_type(&[], []),
        DeviceType::Unidentified
    );
    assert_eq!(
        named("pad.lan").device_type(&[], []),
        DeviceType::Unidentified
    );

    // The two Macs from a live scan, each sharing files and running macOS's local Kerberos KDC,
    // with no OS reading.
    let mac_services = [service(Box::new(Samba)), service(Box::new(Kerberos))];
    for (hostname, expected) in [
        ("mac-mini.lan", DeviceType::Desktop),
        ("FB-Macbook.local", DeviceType::Laptop),
        ("Mayas-MacBook-Pro.local", DeviceType::Laptop),
    ] {
        assert_eq!(
            named(hostname).device_type(&mac_services, []),
            expected,
            "{hostname}"
        );
    }

    // The OS and services both outrank the name.
    let mut mac = named("Mayas-iPhone");
    mac.base.os = Some(Attributed::new(
        HostOsValue(HostOs::family_only(HostOsFamily::MacOs)),
        AttributeSource::SshBannerMatch,
    ));
    assert_eq!(mac.device_type(&[], []), DeviceType::Workstation);
    assert_eq!(
        named("ipad").device_type(&[service(Box::new(TrueNAS))], []),
        DeviceType::Nas
    );
}

/// A single-purpose vendor's MAC types a host nothing else names, below the OS and the name and
/// above the `Server` an application implies.
#[test]
fn a_single_purpose_mac_vendor_ranks_below_the_name_and_above_server() {
    let with_mac = |mac: &str| {
        let mut ip = ip_address(&Uuid::new_v4(), &Uuid::new_v4());
        ip.base.mac_address = Some(MacEvidence::new(
            MacEvidenceValue(mac.parse().expect("valid MAC")),
            AttributeSource::ArpReply,
        ));
        ip
    };
    // Espressif and Nest Labs OUIs from a live scan.
    let espressif = [with_mac("0C:B8:15:01:44:10")];
    let nest = [with_mac("18:B4:30:CF:84:53")];
    assert_eq!(
        host_with_os(None).device_type(&[], &espressif),
        DeviceType::IoT
    );
    assert_eq!(host_with_os(None).device_type(&[], &nest), DeviceType::IoT);

    // Above `Server`: an application that implies a server does not outrank the chip vendor.
    assert_eq!(
        host_with_os(None).device_type(&[service(Box::new(PostgreSQL))], &espressif),
        DeviceType::IoT
    );
    // Below the OS and services.
    assert_eq!(
        host_with_os(Some(HostOs::family_only(HostOsFamily::MacOs))).device_type(&[], &espressif),
        DeviceType::Workstation
    );
    assert_eq!(
        host_with_os(None).device_type(&[service(Box::new(TrueNAS))], &espressif),
        DeviceType::Nas
    );

    // A multi-product vendor (Google) and a randomized MAC imply nothing.
    for mac in ["B4:23:A2:59:5B:2C", "AA:26:68:D2:1D:1C"] {
        assert_eq!(
            host_with_os(None).device_type(&[], &[with_mac(mac)]),
            DeviceType::Unidentified,
            "{mac}"
        );
    }
}

#[test]
fn windows_is_a_server_only_when_its_edition_or_name_says_so() {
    let windows = |name: Option<&str>, edition: Option<&str>| HostOs {
        name: name.map(str::to_string),
        edition: edition.map(str::to_string),
        ..HostOs::family_only(HostOsFamily::Windows)
    };
    assert_eq!(
        windows(Some("Windows Server 2022"), None).device_type(),
        Some(DeviceType::Server)
    );
    assert_eq!(
        windows(Some("Windows 11"), Some("Pro")).device_type(),
        Some(DeviceType::Workstation)
    );
    assert_eq!(windows(None, None).device_type(), None);
}

/// Every word of a device type's name is capitalized, matching service definition names, so
/// an L3 group reads "Access Point" beside "Google Nest Router". A word with a capital anywhere
/// in it (IoT) is spelled as written.
#[test]
fn every_word_of_a_device_type_name_is_capitalized() {
    use strum::IntoEnumIterator;

    let offenders: Vec<&'static str> = DeviceType::iter()
        .map(|device_type| device_type.name())
        .filter(|name| {
            name.split_whitespace().any(|word| {
                let Some(first) = word.chars().next() else {
                    return false;
                };
                !(first.is_uppercase() || word.chars().any(char::is_uppercase))
            })
        })
        .collect();

    assert!(
        offenders.is_empty(),
        "device type names with an uncapitalized word: {offenders:?}"
    );
}

/// A fingerprinted device class sits between what the host's services declare and what its OS
/// implies.
#[test]
fn a_matched_device_type_ranks_below_services_and_above_the_os() {
    use crate::server::hosts::r#impl::attributes::HostMatchedDeviceTypeValue;
    let matched = |os: Option<HostOs>| {
        let mut host = host_with_os(os);
        host.base.matched_device_type = Some(Attributed::new(
            HostMatchedDeviceTypeValue(DeviceType::Camera),
            AttributeSource::X509SubjectMatch,
        ));
        host
    };
    assert_eq!(
        matched(Some(HostOs::family_only(HostOsFamily::MacOs))).device_type(&[], []),
        DeviceType::Camera
    );
    assert_eq!(
        matched(None).device_type(&[service(Box::new(TrueNAS))], []),
        DeviceType::Nas
    );
    // And above `Server`, which any application implies.
    assert_eq!(
        matched(None).device_type(&[service(Box::new(PostgreSQL))], []),
        DeviceType::Camera
    );
}

/// The source names the rung that won, so a stored match carries its own source and a type read
/// off a service or the OS says what it was derived from.
#[test]
fn the_source_names_the_rung_that_decided_the_device_type() {
    use crate::server::hosts::r#impl::attributes::HostMatchedDeviceTypeValue;
    let mac = Some(HostOs::family_only(HostOsFamily::MacOs));
    let nas = service(Box::new(TrueNAS));
    assert_eq!(
        host_with_os(mac.clone()).device_type_with_source([&nas], []),
        (
            DeviceType::Nas,
            HostDeviceTypeSource::Service {
                service_id: nas.id,
                service_name: nas.base.name.clone(),
            }
        )
    );

    let mut matched = host_with_os(mac.clone());
    matched.base.matched_device_type = Some(Attributed::new(
        HostMatchedDeviceTypeValue(DeviceType::Camera),
        AttributeSource::HtmlTitleMatch,
    ));
    assert_eq!(
        matched.device_type_with_source([], []).1,
        HostDeviceTypeSource::Matched {
            source: AttributeSource::HtmlTitleMatch
        }
    );

    assert_eq!(
        host_with_os(mac).device_type_with_source([], []).1,
        HostDeviceTypeSource::OperatingSystem {
            source: AttributeSource::SshBannerMatch
        }
    );
    assert_eq!(
        host_with_os(None).device_type_with_source([], []),
        (DeviceType::Unidentified, HostDeviceTypeSource::NoEvidence)
    );
}

#[test]
fn no_two_device_types_share_an_icon() {
    use strum::IntoEnumIterator;
    let mut seen = std::collections::HashMap::new();
    let mut shared = Vec::new();
    for device_type in DeviceType::iter() {
        if let Some(other) = seen.insert(format!("{:?}", device_type.icon()), device_type) {
            shared.push((other, device_type));
        }
    }
    assert!(
        shared.is_empty(),
        "device types sharing an icon: {shared:?}"
    );
}
