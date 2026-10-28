//! Which part of the product an entity or a plan feature belongs to.
//!
//! One grouping for both: the sidebar builds its sections from the areas entities belong to, plan
//! cards and the website's Product menu head their feature lists with the areas features belong
//! to, and the global search lists its result groups in the same order. Variant order is display
//! order. An area shows wherever it has members, so Platform appears only in the sidebar and API
//! through Billing only on plans.

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, IntoStaticStr};

use crate::server::shared::types::{
    Color, Icon,
    metadata::{EntityMetadataProvider, HasId, TypeMetadataProvider},
};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    Display,
    EnumIter,
    IntoStaticStr,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ProductArea {
    /// How the network is drawn and shared: topologies, snapshots, shares and dependencies.
    Topology,
    /// What is on the network: sites, VLANs, subnets, hosts and what runs on them.
    Inventory,
    /// What finds it: discovery runs, the daemons that perform them and the credentials they use.
    Discovery,
    /// What runs Scanopy across the organization: tags, users and API keys.
    Platform,
    Api,
    Support,
    Security,
    Deployment,
    Billing,
}

impl HasId for ProductArea {
    fn id(&self) -> &'static str {
        self.into()
    }
}

impl EntityMetadataProvider for ProductArea {
    fn color(&self) -> Color {
        Color::Gray
    }

    fn icon(&self) -> Icon {
        match self {
            Self::Topology => Icon::Map,
            Self::Inventory => Icon::Server,
            Self::Discovery => Icon::Radar,
            Self::Platform => Icon::Settings,
            Self::Api => Icon::Code,
            Self::Support => Icon::Headset,
            Self::Security => Icon::ShieldCheck,
            Self::Deployment => Icon::Rocket,
            Self::Billing => Icon::Receipt,
        }
    }
}

impl TypeMetadataProvider for ProductArea {
    fn name(&self) -> &'static str {
        match self {
            Self::Topology => "Topology",
            Self::Inventory => "Inventory",
            Self::Discovery => "Discovery",
            Self::Platform => "Platform",
            Self::Api => "API",
            Self::Support => "Support",
            Self::Security => "Security",
            Self::Deployment => "Deployment",
            Self::Billing => "Billing",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Self::Topology => {
                "Diagrams of your network, workloads and applications, their snapshots, and ways to share them"
            }
            Self::Inventory => {
                "Searchable records of every host, VLAN and subnet discovery finds, and CSV export of them"
            }
            Self::Discovery => {
                "How Scanopy finds hosts, services and devices on your network, and the credentials it uses"
            }
            Self::Platform => "Tags, users and API keys across your organization",
            Self::Api => "Programmatic access to your Scanopy data",
            Self::Support => "Ways to reach the Scanopy team",
            Self::Security => "Sign-in and audit controls for your organization",
            Self::Deployment => "Ways to run Scanopy in your own environment",
            Self::Billing => "Payment and procurement terms",
        }
    }
}
