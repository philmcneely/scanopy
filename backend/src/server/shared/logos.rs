//! Every logo Scanopy draws.
//!
//! A service definition, OS family, sign-in provider or email names a [`Logo`] variant; where the
//! logo comes from and what it is drawn on are stated once, in [`Logo::spec`]. `generate-fixtures`
//! downloads every variant into `ui/static/logos/catalog/` and writes `logos.json` (each logo's
//! path and background, which the UI draws from) and `logo-credits.json` (Settings > System).

use crate::server::auth::r#impl::oidc::IdentityProviderLogo;
use crate::server::hosts::r#impl::os::HostOsFamily;
use crate::server::services::definitions::ServiceDefinitionRegistry;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::shared::fixtures::logo_ext;
use crate::server::shared::types::metadata::TypeMetadataProvider;
use serde::Serialize;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};

/// Every logo Scanopy draws. Serializes as its variant name, the id `logos.json` is keyed by.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, IntoStaticStr, Serialize)]
pub enum Logo {
    ActiveMq,
    ActualBudget,
    AdguardHome,
    Alexa,
    Ansible,
    Apc,
    Apple,
    Appwrite,
    ArgoCd,
    Aruba,
    Asterisk,
    Audiobookshelf,
    Authelia,
    Authentik,
    Autobrr,
    Backrest,
    Bacula,
    Bamboo,
    Bazarr,
    Beszel,
    BigBlueButton,
    Bitbucket,
    Bitwarden,
    BookLore,
    Bookstack,
    Bose,
    Brocade,
    Brother,
    Caddy,
    Cadvisor,
    Calibre,
    CalibreWeb,
    Cassandra,
    Celery,
    Ceph,
    Checkmk,
    Cisco,
    Cleanuperr,
    Cloudflare,
    Cockpit,
    Confluence,
    Consul,
    CoolerControl,
    CouchDb,
    Crowdsec,
    Cups,
    Dashy,
    Dawarich,
    Denodo,
    Discord,
    Discourse,
    Docker,
    Dockge,
    Docmost,
    Domoticz,
    Drone,
    Duplicati,
    Eero,
    Elastic,
    Elasticsearch,
    Emby,
    EntraId,
    EspHome,
    Espressif,
    Esxi,
    Filezilla,
    Fios,
    FireflyIii,
    Forgejo,
    Fortinet,
    FreeBsd,
    FreeIpa,
    FreePbx,
    FreshRss,
    Frigate,
    Gatus,
    Ghost,
    GitHub,
    GitLab,
    Gitea,
    Glance,
    Glances,
    Google,
    GoogleCast,
    GoogleHome,
    Grafana,
    Graylog,
    Grocy,
    HaProxy,
    HashicorpVault,
    Homarr,
    HomeAssistant,
    HomeKit,
    Homepage,
    Hp,
    Hpe,
    Huawei,
    Huntarr,
    Icinga,
    Immich,
    InfluxDb,
    Jaeger,
    Jellyfin,
    Jellyseerr,
    Jellystat,
    Jenkins,
    Jira,
    JitsiMeet,
    Jotty,
    JuniperNetworks,
    Kafka,
    Kanidm,
    Karakeep,
    Keycloak,
    Komga,
    Kong,
    Kubernetes,
    Libvirt,
    Lidarr,
    Linkstack,
    Loki,
    Lubelogger,
    Mailcow,
    MariaDb,
    Mattermost,
    Mealie,
    Memos,
    Metube,
    Microsoft,
    MicrosoftSqlServer,
    MikroTik,
    Minio,
    MongoDb,
    Mqtt,
    MyQ,
    Mysql,
    N8n,
    Nagios,
    Nats,
    Neo4j,
    NetApp,
    NetBsd,
    NetbootXyz,
    Netdata,
    Nextcloud,
    NginxProxyManager,
    Nomad,
    Ntfy,
    Nut,
    Nvidia,
    Okta,
    Ollama,
    Ombi,
    OpenBsd,
    OpenHab,
    OpenLdap,
    OpenMediaVault,
    OpenShift,
    OpenSpeedtest,
    OpenVpn,
    OpenWebUi,
    OpenWrt,
    OpnSense,
    Oracle,
    Overseerr,
    Owncloud,
    PaloAltoNetworks,
    PaperlessNgx,
    Peanut,
    PfSense,
    PhilipsHue,
    PiHole,
    Plex,
    PocketId,
    Podman,
    Portainer,
    PostgreSql,
    PowerDns,
    Printer,
    Prometheus,
    Prowlarr,
    Proxmox,
    Pulse,
    QBittorrent,
    Qnap,
    RabbitMq,
    Radarr,
    Radicale,
    Rancher,
    Redis,
    Restic,
    Ring,
    Roborock,
    RocketChat,
    Roku,
    RolleaseAcmeda,
    Rustdesk,
    SabNzbd,
    SaltProject,
    Scanopy,
    Seafile,
    Sensu,
    Sentry,
    Serviio,
    Sidekiq,
    Slskd,
    Sonarr,
    SonicWall,
    Sonos,
    Spinnaker,
    Splunk,
    Spotify,
    Spring,
    Syncthing,
    Synology,
    Tasmota,
    Tautulli,
    TeamCity,
    Termix,
    Tomcat,
    TpLink,
    Traefik,
    TrueNas,
    Tux,
    Tuya,
    Tyk,
    Ubiquiti,
    Unbound,
    UniFi,
    Unimus,
    UptimeKuma,
    VMware,
    Vaultwarden,
    Veeam,
    Wazuh,
    WindowsLogo,
    Wireguard,
    Wiz,
    Wizarr,
    Wordpress,
    ZWaveJsUi,
    Zabbix,
    Zigbee2Mqtt,
    Zitadel,
}

/// Where a logo's file comes from, holding only what varies within the source.
enum LogoSource {
    /// A file in Dashboard Icons, by its path in the set: `"svg/plex.svg"`.
    DashboardIcons(&'static str),
    /// A file in selfh.st/icons, by its path in the set.
    SelfhstIcons(&'static str),
    /// A Simple Icons icon, by its slug.
    SimpleIcons(&'static str),
    /// A VectorLogoZone file, by its path under `/logos/`.
    VectorLogoZone(&'static str),
    /// A file under libvirt.org/logos/.
    Libvirt(&'static str),
    /// A Wikimedia Commons file, by its path under `/wikipedia/commons/`, with the terms its page
    /// states.
    Wikimedia {
        path: &'static str,
        /// The file's Commons page, which states its terms.
        page: &'static str,
        title: &'static str,
        author: &'static str,
        terms: &'static str,
    },
    /// A company's own mark, from its own site, used to name its product.
    Vendor {
        owner: &'static str,
        homepage: &'static str,
        url: &'static str,
    },
    Scanopy,
}

/// What fills the frame a logo is drawn in. Read from each file once, when the logo is added: its
/// outline and edge colour, not anything decoded at runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Background {
    /// A free-standing mark, on the card's own colour.
    Clear,
    /// The frame takes this colour and the logo keeps its size: the colour of a logo drawn on its
    /// own square or tile, or white behind a dark mark that would vanish on a dark card.
    Solid(&'static str),
    /// The logo is a disc, drawn at the frame's size so it fills the circle.
    Round,
}

/// Whose a logo is and on what terms, as Settings > System lists it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Credit {
    pub name: &'static str,
    pub homepage: &'static str,
    pub copyright: &'static str,
    /// None for a company's own mark, used under no license to name its product.
    pub license: Option<&'static str>,
    pub license_url: Option<&'static str>,
}

impl Logo {
    /// The only place a logo is described: a new variant does not compile without its source and
    /// background.
    fn spec(&self) -> (LogoSource, Background) {
        use Background::*;
        use LogoSource::*;
        match self {
            Self::ActiveMq => (
                VectorLogoZone("apache_activemq/apache_activemq-icon.svg"),
                Clear,
            ),
            Self::ActualBudget => (DashboardIcons("svg/actual-budget.svg"), Solid("#805ad5")),
            Self::AdguardHome => (DashboardIcons("svg/adguard-home.svg"), Clear),
            Self::Alexa => (DashboardIcons("svg/alexa.svg"), Round),
            Self::Ansible => (DashboardIcons("svg/ansible.svg"), Round),
            Self::Apc => (DashboardIcons("svg/apc.svg"), Clear),
            Self::Apple => (DashboardIcons("svg/apple.svg"), Solid("#ffffff")),
            Self::Appwrite => (DashboardIcons("svg/appwrite.svg"), Clear),
            Self::ArgoCd => (DashboardIcons("svg/argo-cd.svg"), Clear),
            Self::Aruba => (DashboardIcons("svg/aruba.svg"), Clear),
            Self::Asterisk => (DashboardIcons("png/asterisk.png"), Clear),
            Self::Audiobookshelf => (DashboardIcons("svg/audiobookshelf.svg"), Round),
            Self::Authelia => (DashboardIcons("svg/authelia.svg"), Clear),
            Self::Authentik => (DashboardIcons("svg/authentik.svg"), Clear),
            Self::Autobrr => (DashboardIcons("svg/autobrr.svg"), Clear),
            Self::Backrest => (DashboardIcons("svg/backrest-light.svg"), Clear),
            Self::Bacula => (DashboardIcons("png/bacula.png"), Clear),
            Self::Bamboo => (DashboardIcons("svg/atlassian-bamboo.svg"), Clear),
            Self::Bazarr => (DashboardIcons("svg/bazarr.svg"), Round),
            Self::Beszel => (DashboardIcons("svg/beszel.svg"), Clear),
            Self::BigBlueButton => (SimpleIcons("bigbluebutton"), Round),
            Self::Bitbucket => (DashboardIcons("svg/bitbucket.svg"), Clear),
            Self::Bitwarden => (DashboardIcons("svg/bitwarden.svg"), Solid("#175ddc")),
            Self::BookLore => (DashboardIcons("svg/book-lore.svg"), Clear),
            Self::Bookstack => (DashboardIcons("svg/bookstack.svg"), Clear),
            Self::Bose => (SimpleIcons("bose"), Solid("#ffffff")),
            Self::Brocade => (DashboardIcons("svg/brocade.svg"), Clear),
            Self::Brother => (DashboardIcons("svg/brother.svg"), Solid("#1535a3")),
            Self::Caddy => (DashboardIcons("svg/caddy.svg"), Clear),
            Self::Cadvisor => (DashboardIcons("png/cadvisor.png"), Clear),
            Self::Calibre => (DashboardIcons("svg/calibre.svg"), Clear),
            Self::CalibreWeb => (DashboardIcons("svg/calibre-web.svg"), Clear),
            Self::Cassandra => (DashboardIcons("svg/apache-cassandra.svg"), Clear),
            Self::Celery => (DashboardIcons("png/celery.png"), Clear),
            Self::Ceph => (DashboardIcons("svg/ceph.svg"), Clear),
            Self::Checkmk => (DashboardIcons("svg/checkmk.svg"), Clear),
            Self::Cisco => (SimpleIcons("cisco"), Solid("#ffffff")),
            Self::Cleanuperr => (DashboardIcons("svg/cleanuperr.svg"), Round),
            Self::Cloudflare => (DashboardIcons("svg/cloudflare.svg"), Clear),
            Self::Cockpit => (DashboardIcons("svg/cockpit.svg"), Round),
            Self::Confluence => (DashboardIcons("svg/confluence.svg"), Clear),
            Self::Consul => (DashboardIcons("svg/consul.svg"), Clear),
            Self::CoolerControl => (DashboardIcons("svg/cooler-control.svg"), Clear),
            Self::CouchDb => (DashboardIcons("svg/couchdb.svg"), Clear),
            Self::Crowdsec => (DashboardIcons("svg/crowdsec.svg"), Clear),
            Self::Cups => (DashboardIcons("svg/cups.svg"), Solid("#ffffff")),
            Self::Dashy => (DashboardIcons("png/dashy.png"), Round),
            Self::Dawarich => (DashboardIcons("svg/dawarich.svg"), Clear),
            Self::Denodo => (DashboardIcons("svg/denodo.svg"), Clear),
            // Emails need PNG, so the email logos are PNGs.
            Self::Discord => (DashboardIcons("png/discord.png"), Clear),
            Self::Discourse => (DashboardIcons("svg/discourse.svg"), Clear),
            Self::Docker => (DashboardIcons("svg/docker.svg"), Clear),
            Self::Dockge => (DashboardIcons("svg/dockge.svg"), Clear),
            Self::Docmost => (DashboardIcons("png/docmost.png"), Solid("#000000")),
            Self::Domoticz => (DashboardIcons("png/domoticz.png"), Round),
            Self::Drone => (DashboardIcons("png/drone.png"), Solid("#ffffff")),
            Self::Duplicati => (DashboardIcons("svg/duplicati.svg"), Clear),
            Self::Eero => (VectorLogoZone("eero/eero-icon.svg"), Solid("#ffffff")),
            Self::Elastic => (DashboardIcons("svg/elastic.svg"), Clear),
            Self::Elasticsearch => (DashboardIcons("svg/elasticsearch.svg"), Clear),
            Self::Emby => (DashboardIcons("svg/emby.svg"), Clear),
            Self::EntraId => (DashboardIcons("svg/entra-id.svg"), Clear),
            Self::EspHome => (DashboardIcons("svg/esphome.svg"), Clear),
            Self::Espressif => (DashboardIcons("svg/espressif.svg"), Clear),
            Self::Esxi => (DashboardIcons("svg/vmware-esxi.svg"), Clear),
            Self::Filezilla => (DashboardIcons("svg/filezilla.svg"), Solid("#a70000")),
            Self::Fios => (DashboardIcons("svg/fios.svg"), Solid("#ffffff")),
            Self::FireflyIii => (DashboardIcons("svg/firefly-iii.svg"), Clear),
            Self::Forgejo => (DashboardIcons("svg/forgejo.svg"), Clear),
            Self::Fortinet => (DashboardIcons("svg/fortinet.svg"), Solid("#ffffff")),
            Self::FreeBsd => (SimpleIcons("freebsd"), Solid("#ffffff")),
            Self::FreeIpa => (DashboardIcons("svg/freeipa.svg"), Clear),
            Self::FreePbx => (DashboardIcons("svg/freepbx.svg"), Clear),
            Self::FreshRss => (DashboardIcons("svg/freshrss.svg"), Clear),
            Self::Frigate => (DashboardIcons("svg/frigate.svg"), Clear),
            Self::Gatus => (DashboardIcons("svg/gatus.svg"), Clear),
            Self::Ghost => (DashboardIcons("png/ghost.png"), Solid("#ffffff")),
            Self::GitHub => (DashboardIcons("png/github.png"), Solid("#ffffff")),
            Self::GitLab => (DashboardIcons("svg/gitlab.svg"), Clear),
            Self::Gitea => (DashboardIcons("svg/gitea.svg"), Clear),
            Self::Glance => (DashboardIcons("svg/glance.svg"), Clear),
            Self::Glances => (DashboardIcons("svg/glances.svg"), Clear),
            Self::Google => (DashboardIcons("svg/google.svg"), Clear),
            Self::GoogleCast => (SimpleIcons("googlecast"), Solid("#ffffff")),
            Self::GoogleHome => (DashboardIcons("svg/google-home.svg"), Clear),
            Self::Grafana => (DashboardIcons("svg/grafana.svg"), Clear),
            Self::Graylog => (DashboardIcons("svg/graylog.svg"), Round),
            Self::Grocy => (DashboardIcons("svg/grocy.svg"), Clear),
            Self::HaProxy => (DashboardIcons("svg/haproxy.svg"), Clear),
            Self::HashicorpVault => (DashboardIcons("svg/hashicorp-vault.svg"), Clear),
            Self::Homarr => (DashboardIcons("svg/homarr.svg"), Clear),
            Self::HomeAssistant => (DashboardIcons("svg/home-assistant.svg"), Clear),
            // Apple Home's house mark: Dashboard Icons has no HomeKit icon.
            Self::HomeKit => (SelfhstIcons("svg/apple-homekit.svg"), Clear),
            Self::Homepage => (DashboardIcons("webp/homepage.webp"), Clear),
            Self::Hp => (DashboardIcons("svg/hp.svg"), Solid("#ffffff")),
            Self::Hpe => (DashboardIcons("svg/hpe.svg"), Clear),
            Self::Huawei => (SimpleIcons("huawei"), Solid("#ffffff")),
            Self::Huntarr => (DashboardIcons("png/huntarr.png"), Clear),
            Self::Icinga => (DashboardIcons("svg/icinga.svg"), Solid("#ffffff")),
            Self::Immich => (DashboardIcons("svg/immich.svg"), Clear),
            Self::InfluxDb => (DashboardIcons("svg/influxdb.svg"), Solid("#ffffff")),
            Self::Jaeger => (DashboardIcons("svg/jaeger.svg"), Clear),
            Self::Jellyfin => (DashboardIcons("svg/jellyfin.svg"), Clear),
            Self::Jellyseerr => (DashboardIcons("svg/jellyseerr.svg"), Clear),
            Self::Jellystat => (DashboardIcons("svg/jellystat.svg"), Round),
            Self::Jenkins => (DashboardIcons("svg/jenkins.svg"), Clear),
            Self::Jira => (DashboardIcons("svg/jira.svg"), Clear),
            Self::JitsiMeet => (DashboardIcons("svg/jitsi-meet.svg"), Clear),
            Self::Jotty => (DashboardIcons("svg/jotty.svg"), Solid("#8b3bd0")),
            Self::JuniperNetworks => (SimpleIcons("junipernetworks"), Solid("#ffffff")),
            Self::Kafka => (SimpleIcons("apachekafka"), Solid("#ffffff")),
            Self::Kanidm => (DashboardIcons("svg/kanidm.svg"), Clear),
            Self::Karakeep => (DashboardIcons("svg/karakeep.svg"), Clear),
            Self::Keycloak => (DashboardIcons("svg/keycloak.svg"), Clear),
            Self::Komga => (DashboardIcons("svg/komga.svg"), Round),
            Self::Kong => (SimpleIcons("kong"), Solid("#ffffff")),
            Self::Kubernetes => (DashboardIcons("svg/kubernetes.svg"), Clear),
            // LGPL-2.1+; the licence ships as `backend/assets/libvirt/LICENSE`. Remove it if this
            // logo is replaced.
            Self::Libvirt => (Libvirt("logo-square.svg"), Clear),
            Self::Lidarr => (DashboardIcons("svg/lidarr.svg"), Round),
            Self::Linkstack => (DashboardIcons("svg/linkstack.svg"), Clear),
            Self::Loki => (DashboardIcons("svg/loki.svg"), Clear),
            Self::Lubelogger => (DashboardIcons("png/lubelogger.png"), Solid("#404040")),
            Self::Mailcow => (DashboardIcons("svg/mailcow.svg"), Clear),
            Self::MariaDb => (DashboardIcons("svg/mariadb.svg"), Solid("#ffffff")),
            Self::Mattermost => (DashboardIcons("svg/mattermost.svg"), Solid("#ffffff")),
            Self::Mealie => (DashboardIcons("svg/mealie.svg"), Clear),
            Self::Memos => (DashboardIcons("png/memos.png"), Solid("#0b0a18")),
            Self::Metube => (DashboardIcons("svg/metube.svg"), Solid("#f55848")),
            Self::Microsoft => (DashboardIcons("svg/microsoft.svg"), Clear),
            Self::MicrosoftSqlServer => (
                DashboardIcons("svg/microsoft-sql-server-light.svg"),
                Solid("#ffffff"),
            ),
            Self::MikroTik => (DashboardIcons("svg/mikrotik.svg"), Solid("#ffffff")),
            Self::Minio => (DashboardIcons("svg/minio.svg"), Solid("#ffffff")),
            Self::MongoDb => (DashboardIcons("svg/mongodb.svg"), Solid("#ffffff")),
            Self::Mqtt => (DashboardIcons("svg/mqtt.svg"), Clear),
            // No icon set carries myQ; this is the app icon from myq.com. The `context` token is what
            // the site's media server needs to serve it: without it the request gets a 400.
            Self::MyQ => (
                Vendor {
                    owner: "The Chamberlain Group",
                    homepage: "https://www.myq.com/",
                    url: "https://www.myq.com/medias/myQ-app-logo.png?context=bWFzdGVyfHJvb3R8MjcxN3xpbWFnZS9wbmd8YURjekwyZzNPUzg1TWpnNU5UVXhNVFV4TVRNMEwyMTVVUzFoY0hBdGJHOW5ieTV3Ym1jfGYwM2U2OWE1OWM3NGE3OTBkYmJlYmE3NmNkNDlhYThhNDg4YjUyZjY4MDVkZjY4OGUzNzBiOGRlOTlkZDM1MGM",
                },
                Solid("#0d1c99"),
            ),
            Self::Mysql => (DashboardIcons("svg/mysql.svg"), Solid("#ffffff")),
            Self::N8n => (DashboardIcons("svg/n8n.svg"), Clear),
            Self::Nagios => (DashboardIcons("svg/nagios.svg"), Clear),
            Self::Nats => (SimpleIcons("natsdotio"), Solid("#ffffff")),
            Self::Neo4j => (DashboardIcons("png/neo4j.png"), Clear),
            Self::NetApp => (SimpleIcons("netapp"), Solid("#ffffff")),
            Self::NetBsd => (SimpleIcons("netbsd"), Solid("#ffffff")),
            Self::NetbootXyz => (DashboardIcons("svg/netbootxyz.svg"), Clear),
            Self::Netdata => (DashboardIcons("svg/netdata.svg"), Clear),
            Self::Nextcloud => (DashboardIcons("svg/nextcloud.svg"), Clear),
            Self::NginxProxyManager => (DashboardIcons("svg/nginx-proxy-manager.svg"), Clear),
            Self::Nomad => (DashboardIcons("svg/nomad.svg"), Clear),
            Self::Ntfy => (DashboardIcons("svg/ntfy.svg"), Clear),
            Self::Nut => (DashboardIcons("svg/nut.svg"), Clear),
            Self::Nvidia => (DashboardIcons("svg/nvidia.svg"), Clear),
            Self::Okta => (DashboardIcons("svg/okta.svg"), Clear),
            Self::Ollama => (DashboardIcons("svg/ollama-dark.svg"), Clear),
            Self::Ombi => (DashboardIcons("svg/ombi.svg"), Round),
            Self::OpenBsd => (SimpleIcons("openbsd"), Solid("#ffffff")),
            Self::OpenHab => (DashboardIcons("svg/openhab.svg"), Clear),
            Self::OpenLdap => (DashboardIcons("svg/openldap.svg"), Clear),
            Self::OpenMediaVault => (DashboardIcons("svg/openmediavault.svg"), Solid("#5dacdf")),
            Self::OpenShift => (DashboardIcons("svg/openshift.svg"), Solid("#000000")),
            Self::OpenSpeedtest => (DashboardIcons("svg/openspeedtest.svg"), Clear),
            Self::OpenVpn => (DashboardIcons("svg/openvpn.svg"), Clear),
            Self::OpenWebUi => (DashboardIcons("svg/open-webui-light.svg"), Round),
            Self::OpenWrt => (DashboardIcons("svg/openwrt.svg"), Clear),
            Self::OpnSense => (DashboardIcons("svg/opnsense.svg"), Clear),
            Self::Oracle => (DashboardIcons("svg/oracle.svg"), Solid("#ffffff")),
            Self::Overseerr => (DashboardIcons("svg/overseerr.svg"), Round),
            Self::Owncloud => (DashboardIcons("svg/owncloud.svg"), Solid("#ffffff")),
            Self::PaloAltoNetworks => (SimpleIcons("paloaltonetworks"), Solid("#ffffff")),
            Self::PaperlessNgx => (DashboardIcons("svg/paperless-ngx.svg"), Solid("#ffffff")),
            Self::Peanut => (DashboardIcons("svg/peanut.svg"), Clear),
            Self::PfSense => (DashboardIcons("svg/pfsense.svg"), Solid("#1a1a1a")),
            Self::PhilipsHue => (SimpleIcons("philipshue"), Solid("#ffffff")),
            Self::PiHole => (DashboardIcons("svg/pi-hole.svg"), Clear),
            Self::Plex => (DashboardIcons("svg/plex.svg"), Solid("#282a2d")),
            Self::PocketId => (DashboardIcons("svg/pocket-id-light.svg"), Round),
            Self::Podman => (DashboardIcons("svg/podman.svg"), Clear),
            Self::Portainer => (DashboardIcons("svg/portainer.svg"), Clear),
            Self::PostgreSql => (DashboardIcons("svg/postgresql.svg"), Solid("#ffffff")),
            Self::PowerDns => (DashboardIcons("svg/powerdns.svg"), Clear),
            Self::Printer => (DashboardIcons("svg/printer.svg"), Clear),
            Self::Prometheus => (DashboardIcons("svg/prometheus.svg"), Round),
            Self::Prowlarr => (DashboardIcons("svg/prowlarr.svg"), Round),
            Self::Proxmox => (DashboardIcons("svg/proxmox.svg"), Clear),
            Self::Pulse => (DashboardIcons("svg/pulse.svg"), Round),
            Self::QBittorrent => (DashboardIcons("svg/qbittorrent.svg"), Round),
            Self::Qnap => (DashboardIcons("svg/qnap.svg"), Clear),
            Self::RabbitMq => (DashboardIcons("svg/rabbitmq.svg"), Clear),
            Self::Radarr => (DashboardIcons("svg/radarr.svg"), Clear),
            Self::Radicale => (DashboardIcons("svg/radicale.svg"), Clear),
            Self::Rancher => (DashboardIcons("svg/rancher.svg"), Clear),
            Self::Redis => (DashboardIcons("svg/redis.svg"), Clear),
            Self::Restic => (DashboardIcons("png/restic.png"), Round),
            Self::Ring => (SimpleIcons("ring"), Solid("#ffffff")),
            // Roborock's site serves its assets from this Webflow bucket.
            Self::Roborock => (
                Vendor {
                    owner: "Roborock",
                    homepage: "https://us.roborock.com/",
                    url: "https://cdn.prod.website-files.com/6720fa7290112f642d0a83ac/674de5c03f0d05e27b16b2cf_roborock_logo.webp",
                },
                Clear,
            ),
            Self::RocketChat => (DashboardIcons("svg/rocket-chat.svg"), Clear),
            Self::Roku => (SimpleIcons("roku"), Solid("#ffffff")),
            Self::RolleaseAcmeda => (
                Vendor {
                    owner: "Rollease Acmeda",
                    homepage: "https://www.rolleaseacmeda.com/",
                    url: "https://www.rolleaseacmeda.com/icon0.svg",
                },
                Clear,
            ),
            Self::Rustdesk => (DashboardIcons("svg/rustdesk.svg"), Solid("#0071ff")),
            Self::SabNzbd => (DashboardIcons("svg/sabnzbd.svg"), Clear),
            Self::SaltProject => (DashboardIcons("svg/salt-project.svg"), Clear),
            Self::Scanopy => (Scanopy, Solid("#3e2544")),
            Self::Seafile => (DashboardIcons("svg/seafile.svg"), Clear),
            Self::Sensu => (DashboardIcons("svg/sensu.svg"), Clear),
            Self::Sentry => (DashboardIcons("svg/sentry.svg"), Solid("#ffffff")),
            Self::Serviio => (DashboardIcons("svg/serviio.svg"), Solid("#ffce00")),
            Self::Sidekiq => (SimpleIcons("sidekiq"), Round),
            Self::Slskd => (DashboardIcons("svg/slskd.svg"), Clear),
            Self::Sonarr => (DashboardIcons("svg/sonarr.svg"), Round),
            Self::SonicWall => (SimpleIcons("sonicwall"), Solid("#ffffff")),
            Self::Sonos => (SimpleIcons("sonos"), Solid("#ffffff")),
            Self::Spinnaker => (SimpleIcons("spinnaker"), Solid("#ffffff")),
            Self::Splunk => (DashboardIcons("svg/splunk.svg"), Solid("#8dc151")),
            Self::Spotify => (DashboardIcons("svg/spotify.svg"), Round),
            Self::Spring => (VectorLogoZone("springio/springio-icon.svg"), Clear),
            Self::Syncthing => (DashboardIcons("svg/syncthing.svg"), Round),
            Self::Synology => (DashboardIcons("svg/synology.svg"), Solid("#ffffff")),
            Self::Tasmota => (DashboardIcons("svg/tasmota.svg"), Solid("#ffffff")),
            Self::Tautulli => (DashboardIcons("svg/tautulli.svg"), Round),
            Self::TeamCity => (DashboardIcons("svg/teamcity-light.svg"), Clear),
            Self::Termix => (DashboardIcons("svg/termix.svg"), Solid("#141513")),
            Self::Tomcat => (DashboardIcons("svg/apache-tomcat.svg"), Clear),
            Self::TpLink => (DashboardIcons("svg/tp-link.svg"), Clear),
            Self::Traefik => (DashboardIcons("svg/traefik.svg"), Clear),
            Self::TrueNas => (DashboardIcons("svg/truenas.svg"), Clear),
            Self::Tux => (
                Wikimedia {
                    path: "3/35/Tux.svg",
                    page: "https://commons.wikimedia.org/wiki/File:Tux.svg",
                    title: "Tux",
                    author: "Larry Ewing, Simon Budig and Garrett LeSage",
                    terms: "Use with acknowledgement",
                },
                Clear,
            ),
            Self::Tuya => (DashboardIcons("svg/tuya-smart-inc.svg"), Clear),
            Self::Tyk => (VectorLogoZone("tyk/tyk-icon.svg"), Clear),
            Self::Ubiquiti => (DashboardIcons("svg/ubiquiti.svg"), Round),
            Self::Unbound => (DashboardIcons("svg/unbound.svg"), Clear),
            Self::UniFi => (DashboardIcons("svg/unifi.svg"), Round),
            Self::Unimus => (DashboardIcons("svg/unimus.svg"), Clear),
            Self::UptimeKuma => (DashboardIcons("svg/uptime-kuma.svg"), Clear),
            Self::VMware => (DashboardIcons("svg/vmware.svg"), Solid("#ffffff")),
            Self::Vaultwarden => (DashboardIcons("svg/vaultwarden.svg"), Round),
            Self::Veeam => (DashboardIcons("svg/veeam.svg"), Clear),
            Self::Wazuh => (DashboardIcons("svg/wazuh.svg"), Round),
            Self::WindowsLogo => (
                Wikimedia {
                    path: "8/87/Windows_logo_-_2021.svg",
                    page: "https://commons.wikimedia.org/wiki/File:Windows_logo_-_2021.svg",
                    title: "Windows logo",
                    author: "Microsoft",
                    terms: "Public domain",
                },
                Clear,
            ),
            Self::Wireguard => (DashboardIcons("svg/wireguard.svg"), Round),
            Self::Wiz => (DashboardIcons("svg/wiz.svg"), Clear),
            Self::Wizarr => (DashboardIcons("svg/wizarr.svg"), Round),
            Self::Wordpress => (DashboardIcons("svg/wordpress.svg"), Clear),
            Self::ZWaveJsUi => (DashboardIcons("svg/z-wave-js-ui.svg"), Clear),
            Self::Zabbix => (DashboardIcons("svg/zabbix.svg"), Solid("#cb2415")),
            Self::Zigbee2Mqtt => (DashboardIcons("svg/zigbee2mqtt.svg"), Clear),
            // jsDelivr refuses the SVG; the PNG is the same icon.
            Self::Zitadel => (DashboardIcons("png/zitadel.png"), Clear),
        }
    }

    /// Where the file is downloaded from, and what an email links.
    pub fn url(&self) -> String {
        match self.spec().0 {
            LogoSource::DashboardIcons(path) => {
                format!("https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/{path}")
            }
            LogoSource::SelfhstIcons(path) => {
                format!("https://cdn.jsdelivr.net/gh/selfhst/icons/{path}")
            }
            LogoSource::SimpleIcons(slug) => format!("https://simpleicons.org/icons/{slug}.svg"),
            LogoSource::VectorLogoZone(path) => format!("https://www.vectorlogo.zone/logos/{path}"),
            LogoSource::Libvirt(path) => format!("https://libvirt.org/logos/{path}"),
            LogoSource::Wikimedia { path, .. } => {
                format!("https://upload.wikimedia.org/wikipedia/commons/{path}")
            }
            LogoSource::Vendor { url, .. } => url.to_string(),
            LogoSource::Scanopy => {
                "https://cdn.jsdelivr.net/gh/scanopy/scanopy@main/media/logo.png".to_string()
            }
        }
    }

    /// The downloaded copy's name: the variant's id and the file's extension. Keyed by the variant,
    /// so a change of source keeps the path.
    pub fn file_name(&self) -> String {
        let id: &'static str = self.into();
        format!("{id}.{}", logo_ext(&self.url()))
    }

    /// Where the UI serves it. Scanopy's own logo ships with the UI rather than being downloaded.
    pub fn path(&self) -> String {
        match self {
            Self::Scanopy => "/logos/scanopy-logo.png".to_string(),
            _ => format!("/logos/catalog/{}", self.file_name()),
        }
    }

    /// Whether `generate-fixtures` downloads it.
    pub fn is_downloaded(&self) -> bool {
        !matches!(self, Self::Scanopy)
    }

    pub fn background(&self) -> Background {
        self.spec().1
    }

    pub fn credit(&self) -> Credit {
        match self.spec().0 {
            LogoSource::DashboardIcons(_) => Credit {
                name: "Dashboard Icons",
                homepage: "https://github.com/homarr-labs/dashboard-icons",
                copyright: "2024 Bjorn Lammers, Meier Lukas, Thomas Camlong and Homarr Labs",
                license: Some("Apache License 2.0"),
                license_url: Some(
                    "https://github.com/scanopy/scanopy/blob/main/backend/assets/dashboard-icons/LICENSE",
                ),
            },
            LogoSource::SelfhstIcons(_) => Credit {
                name: "selfh.st/icons",
                homepage: "https://github.com/selfhst/icons",
                copyright: "selfh.st",
                license: Some("CC BY 4.0"),
                license_url: Some("https://github.com/selfhst/icons/blob/main/LICENSE"),
            },
            LogoSource::SimpleIcons(_) => Credit {
                name: "Simple Icons",
                homepage: "https://github.com/simple-icons/simple-icons",
                copyright: "Simple Icons contributors",
                license: Some("CC0 1.0"),
                license_url: Some(
                    "https://github.com/simple-icons/simple-icons/blob/develop/LICENSE.md",
                ),
            },
            LogoSource::VectorLogoZone(_) => Credit {
                name: "VectorLogoZone",
                homepage: "https://www.vectorlogo.zone/",
                copyright: "The respective trademark owners",
                license: None,
                license_url: None,
            },
            LogoSource::Libvirt(_) => Credit {
                name: "libvirt",
                homepage: "https://libvirt.org/logos/",
                copyright: "The libvirt project authors",
                license: Some("GNU LGPL 2.1 or later"),
                license_url: Some(
                    "https://github.com/scanopy/scanopy/blob/main/backend/assets/libvirt/LICENSE",
                ),
            },
            LogoSource::Wikimedia {
                page,
                title,
                author,
                terms,
                ..
            } => Credit {
                name: title,
                homepage: page,
                copyright: author,
                license: Some(terms),
                license_url: Some(page),
            },
            LogoSource::Vendor {
                owner, homepage, ..
            } => Credit {
                name: owner,
                homepage,
                copyright: owner,
                license: None,
                license_url: None,
            },
            LogoSource::Scanopy => Credit {
                name: "Scanopy",
                homepage: "https://scanopy.net",
                copyright: "Scanopy LLC",
                license: None,
                license_url: None,
            },
        }
    }
}

/// A logo as `logos.json` lists it for the UI.
#[derive(Debug, Serialize)]
pub struct LogoEntry {
    pub id: Logo,
    pub path: String,
    pub background: Background,
}

/// Every logo, for `logos.json`.
pub fn logo_entries() -> Vec<LogoEntry> {
    Logo::iter()
        .map(|logo| LogoEntry {
            id: logo,
            path: logo.path(),
            background: logo.background(),
        })
        .collect()
}

/// What each logo stands for, by the names of the services, OS families and sign-in providers that
/// draw it.
fn logo_uses() -> Vec<(Logo, &'static str)> {
    let services = ServiceDefinitionRegistry::all_service_definitions()
        .into_iter()
        .filter_map(|s| s.logo().map(|logo| (logo, ServiceDefinition::name(&s))));
    let os_families = HostOsFamily::iter()
        .filter_map(|f| f.logo().map(|logo| (logo, TypeMetadataProvider::name(&f))));
    let identity_providers = IdentityProviderLogo::iter().map(|p| (p.logo(), p.name()));
    services
        .chain(os_families)
        .chain(identity_providers)
        .collect()
}

#[derive(Debug, Serialize)]
pub struct LogoCredit {
    #[serde(flatten)]
    pub credit: Credit,
    /// What its logos stand for, sorted and without repeats.
    pub used_for: Vec<&'static str>,
}

/// Each credit with what its logos stand for, most-used first. A logo only an email draws is
/// listed by its id.
pub fn logo_credits() -> Vec<LogoCredit> {
    let uses = logo_uses();
    let mut credits: Vec<LogoCredit> = Vec::new();
    for logo in Logo::iter() {
        let credit = logo.credit();
        let mut names: Vec<&'static str> = uses
            .iter()
            .filter(|(used, _)| *used == logo)
            .map(|(_, name)| *name)
            .collect();
        if names.is_empty() {
            names.push(logo.into());
        }
        match credits.iter_mut().find(|c| c.credit == credit) {
            Some(existing) => existing.used_for.extend(names),
            None => credits.push(LogoCredit {
                credit,
                used_for: names,
            }),
        }
    }
    for credit in &mut credits {
        credit
            .used_for
            .sort_unstable_by_key(|name| name.to_lowercase());
        credit.used_for.dedup();
    }
    credits.sort_by(|a, b| {
        b.used_for
            .len()
            .cmp(&a.used_for.len())
            .then_with(|| a.credit.name.cmp(b.credit.name))
    });
    credits
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A slug given where a file path belongs (or the reverse) builds a URL that downloads nothing
    /// and a path the UI cannot serve.
    #[test]
    fn every_logo_builds_a_url_to_an_image_file() {
        let bad: Vec<(Logo, String)> = Logo::iter()
            .map(|logo| (logo, logo.url()))
            .filter(|(_, url)| {
                !reqwest::Url::parse(url).is_ok_and(|u| {
                    [".svg", ".png", ".webp"]
                        .iter()
                        .any(|ext| u.path().ends_with(ext))
                })
            })
            .collect();
        assert!(bad.is_empty(), "logos without an image URL: {bad:?}");
    }
}
