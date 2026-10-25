use crate::server::hosts::r#impl::base::Host;
use crate::server::ip_addresses::r#impl::base::{IPAddress, mac_of};
use crate::server::services::r#impl::base::Service;
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::{
    ServiceDefinition, ServiceDefinitionExt, VirtualizationRole,
};
use crate::server::services::r#impl::patterns::Vendor;
use crate::server::shared::entities::EntityDiscriminants;
use crate::server::shared::oui;
use crate::server::shared::types::metadata::{EntityMetadataProvider, HasId, TypeMetadataProvider};
use crate::server::shared::types::{Color, Icon};
use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, IntoStaticStr};
use utoipa::ToSchema;

/// What a host is, as a device: the class its icon stands for on the map.
///
/// Declaration order is precedence, highest first. A host whose services claim several device types takes
/// the first one here, so a firewall that also serves files reads as a firewall, and `Server`,
/// which any application implies, never hides a more specific device type.
///
/// `Unidentified` means no evidence yet, not a device class outside this list: every class the
/// evidence names gets its own variant.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    ToSchema,
    EnumIter,
    IntoStaticStr,
)]
pub enum DeviceType {
    Firewall,
    Router,
    Switch,
    AccessPoint,
    Hypervisor,
    Storage,
    Printer,
    Camera,
    Phone,
    Workstation,
    Tablet,
    Mobile,
    MediaPlayer,
    Speaker,
    Industrial,
    IoT,
    Server,
    /// Also what a device type this build doesn't know reads as, so a newer server cannot break an older
    /// reader.
    #[default]
    #[serde(other)]
    Unidentified,
}

impl HasId for DeviceType {
    fn id(&self) -> &'static str {
        self.into()
    }
}

impl EntityMetadataProvider for DeviceType {
    /// Every device type wears the Host entity's colour: a device is a host first, and the glyph alone
    /// names its device type. Software takes its category's colour instead, so the two never compete.
    fn color(&self) -> Color {
        EntityDiscriminants::Host.color()
    }

    fn icon(&self) -> Icon {
        match self {
            DeviceType::Firewall => Icon::BrickWallFire,
            DeviceType::Router => Icon::Router,
            DeviceType::Switch => Icon::Network,
            DeviceType::AccessPoint => Icon::Wifi,
            DeviceType::Hypervisor => ServiceCategory::Hypervisor.icon(),
            DeviceType::Storage => Icon::HardDrive,
            DeviceType::Printer => Icon::Printer,
            DeviceType::Camera => Icon::Cctv,
            DeviceType::Phone => Icon::Phone,
            DeviceType::Workstation => Icon::Laptop,
            DeviceType::Tablet => Icon::Tablet,
            DeviceType::Mobile => Icon::Smartphone,
            DeviceType::MediaPlayer => Icon::Tv,
            DeviceType::Speaker => Icon::Speaker,
            DeviceType::Industrial => Icon::Factory,
            DeviceType::IoT => Icon::CircuitBoard,
            DeviceType::Server => Icon::Server,
            // The glyph every other unknown value draws, so a host nothing identifies never reads as
            // a device type. The Host icon is `Server`'s.
            DeviceType::Unidentified => Icon::CircleQuestionMark,
        }
    }
}

impl TypeMetadataProvider for DeviceType {
    fn name(&self) -> &'static str {
        match self {
            DeviceType::Firewall => "Firewall",
            DeviceType::Router => "Router",
            DeviceType::Switch => "Switch",
            DeviceType::AccessPoint => "Access Point",
            DeviceType::Hypervisor => "Hypervisor",
            DeviceType::Storage => "Storage",
            DeviceType::Printer => "Printer",
            DeviceType::Camera => "Camera",
            DeviceType::Phone => "Phone",
            DeviceType::Workstation => "Workstation",
            DeviceType::Tablet => "Tablet",
            DeviceType::Mobile => "Mobile Device",
            DeviceType::MediaPlayer => "Media Player",
            DeviceType::Speaker => "Speaker",
            DeviceType::Industrial => "Industrial Controller",
            DeviceType::IoT => "IoT Device",
            DeviceType::Server => "Server",
            DeviceType::Unidentified => "Unidentified",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            DeviceType::Firewall => "Filters traffic between networks",
            DeviceType::Router => "Routes traffic between networks",
            DeviceType::Switch => "Forwards frames between ports on one network",
            DeviceType::AccessPoint => "Bridges wireless clients onto the network",
            DeviceType::Hypervisor => "Runs virtual machines",
            DeviceType::Storage => "Holds files or blocks for other hosts",
            DeviceType::Printer => "Prints or scans",
            DeviceType::Camera => "Streams video",
            DeviceType::Phone => "Carries voice calls",
            DeviceType::Workstation => "A computer a person works at",
            DeviceType::Tablet => "A tablet",
            DeviceType::Mobile => "A phone or other handheld device",
            DeviceType::MediaPlayer => "Plays video to a screen",
            DeviceType::Speaker => "Plays audio",
            DeviceType::Industrial => "Controls building or industrial equipment",
            DeviceType::IoT => "A smart-home or embedded device",
            DeviceType::Server => "Runs applications for other hosts",
            DeviceType::Unidentified => "Nothing found on this host names what it is",
        }
    }
}

impl Host {
    /// The device type this host's icon shows.
    ///
    /// The ladder, first answer wins:
    /// 1. The highest-precedence device type the host's own services declare, other than `Server`.
    ///    A matched definition names the product the host runs, so it outranks the OS.
    /// 2. The device type its OS family implies.
    /// 3. The device type its hostname or name implies: Apple names a device after its product
    ///    ("Maya's iPhone", "Mayas-MacBook-Pro"), and that name reaches us through PTR, mDNS and
    ///    controller DHCP leases.
    /// 4. The device type its MAC vendor implies, for vendors whose every product is one class of
    ///    device: Espressif sells only embedded Wi-Fi chips.
    /// 5. `Server`, when one of its services implies it.
    /// 6. `Server`, when the host runs containers: a container runtime, or software inside one.
    /// 7. `Unidentified`.
    ///
    /// `Server` is what any application implies, so it yields to an OS or a name that says what
    /// the machine is: a Mac sharing files over SMB, or running Postgres, is still a workstation.
    /// A name sits below the OS because anyone can type one. Containerized software says what the
    /// container does, not what the machine is, so it ranks with the weakest evidence. A Linux host
    /// gives no OS device type, so its applications or containers still make it a server.
    pub fn device_type<'a, 'b>(
        &self,
        services: impl IntoIterator<Item = &'a Service>,
        ip_addresses: impl IntoIterator<Item = &'b IPAddress>,
    ) -> DeviceType {
        let (containerized, own): (Vec<&Service>, Vec<&Service>) = services
            .into_iter()
            .partition(|s| s.base.virtualization_metadata.is_some());
        let runs_containers = !containerized.is_empty()
            || own.iter().any(|s| {
                matches!(
                    s.base.service_definition.virtualization_role(),
                    Some(VirtualizationRole::ContainerRuntime { .. })
                )
            });

        let declared: Vec<DeviceType> = own
            .iter()
            .filter_map(|s| s.base.service_definition.device_type())
            .collect();
        let implies_server = declared.contains(&DeviceType::Server);

        declared
            .into_iter()
            .filter(|device_type| *device_type != DeviceType::Server)
            .min()
            .or_else(|| {
                self.base
                    .os
                    .as_ref()
                    .and_then(|os| os.value().0.device_type())
            })
            .or_else(|| self.named_device_type())
            .or_else(|| {
                ip_addresses
                    .into_iter()
                    .filter_map(|ip| mac_of(&ip.base.mac_address))
                    .find_map(|mac| device_type_from_mac(&mac.to_string()))
            })
            .or((implies_server || runs_containers).then_some(DeviceType::Server))
            .unwrap_or_default()
    }

    /// The device type the hostname or name names by product, for products that name themselves.
    fn named_device_type(&self) -> Option<DeviceType> {
        let hostname = self.base.hostname.as_ref().map(|h| h.value().0.clone());
        let name = self.base.name.value().as_str().into_owned();
        [hostname, Some(name)]
            .into_iter()
            .flatten()
            .find_map(|label| device_type_from_label(&label))
    }
}

/// The device type a MAC's vendor implies. Only vendors that make one class of device are listed:
/// Google's OUI covers phones, laptops and speakers alike, so it implies nothing. A randomized
/// (locally administered) MAC has no vendor and implies nothing either.
fn device_type_from_mac(mac: &str) -> Option<DeviceType> {
    match oui::lookup_by_mac(mac)?.company_name.as_str() {
        Vendor::ESPRESSIF | Vendor::NEST => Some(DeviceType::IoT),
        _ => None,
    }
}

/// `"Mayas-iPhone"`, `"Maya's iPad"`, `"iphone15"` and `"FB-Macbook"` name a product;
/// `"iphoneclone"` does not. A word matches when it is the product name, optionally followed by a
/// model number. A Mac name split in two (`"mac-mini"`, `"Mac Studio"`) matches as the pair; a bare
/// `"mac"` names nothing. A brand's tablet (`"xiaomi-pad-mini"`) matches as the pair; a bare `"pad"`
/// names nothing.
fn device_type_from_label(label: &str) -> Option<DeviceType> {
    let words: Vec<String> = label
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_ascii_lowercase)
        .collect();
    words.iter().enumerate().find_map(|(i, word)| {
        let product = word.trim_end_matches(|c: char| c.is_ascii_digit());
        match product {
            "iphone" => Some(DeviceType::Mobile),
            // Android's default device name is "<Owner>'s Phone". A desk phone calls itself an IP
            // or SIP phone, so those pairs stay unmatched.
            "phone"
                if !matches!(
                    i.checked_sub(1)
                        .and_then(|p| words.get(p))
                        .map(String::as_str),
                    Some("ip" | "voip" | "sip")
                ) =>
            {
                Some(DeviceType::Mobile)
            }
            "ipad" => Some(DeviceType::Tablet),
            "pad"
                if matches!(
                    i.checked_sub(1)
                        .and_then(|p| words.get(p))
                        .map(String::as_str),
                    Some("xiaomi" | "redmi")
                ) =>
            {
                Some(DeviceType::Tablet)
            }
            "doorbell" => Some(DeviceType::Camera),
            // A Schlage Wi-Fi lock's default DHCP name. It sleeps between cloud check-ins and opens
            // no ports, so the name is the only evidence a scan gets.
            "schlage" => Some(DeviceType::IoT),
            "macbook" | "macbookpro" | "macbookair" | "imac" | "imacpro" | "macmini" | "macpro"
            | "macstudio" => Some(DeviceType::Workstation),
            "mac"
                if matches!(
                    words.get(i + 1).map(String::as_str),
                    Some("mini" | "pro" | "studio")
                ) =>
            {
                Some(DeviceType::Workstation)
            }
            _ => None,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::hosts::r#impl::attributes::{HostHostnameValue, HostOsValue};
    use crate::server::hosts::r#impl::os::{HostOs, HostOsFamily};
    use crate::server::ip_addresses::r#impl::base::{MacEvidence, MacEvidenceValue};
    use crate::server::services::definitions::{
        docker_daemon::Docker, kerberos::Kerberos, pf_sense::PfSense, postgresql::PostgreSQL,
        proxmox::Proxmox, samba::Samba, snmp::Snmp, true_nas::TrueNAS,
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
        host.base.os =
            os.map(|os| Attributed::new(HostOsValue(os), AttributeSource::SshBannerMatch));
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
            DeviceType::Storage
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
            DeviceType::Storage
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
        assert_eq!(named("schlage.lan").device_type(&[], []), DeviceType::IoT);
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
        for hostname in [
            "mac-mini.lan",
            "FB-Macbook.local",
            "Mayas-MacBook-Pro.local",
        ] {
            assert_eq!(
                named(hostname).device_type(&mac_services, []),
                DeviceType::Workstation,
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
            DeviceType::Storage
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
            host_with_os(Some(HostOs::family_only(HostOsFamily::MacOs)))
                .device_type(&[], &espressif),
            DeviceType::Workstation
        );
        assert_eq!(
            host_with_os(None).device_type(&[service(Box::new(TrueNAS))], &espressif),
            DeviceType::Storage
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
}
