use crate::server::shared::types::metadata::{EntityMetadataProvider, HasId, TypeMetadataProvider};
use crate::server::shared::types::{Color, Icon};
use strum::{EnumIter, IntoStaticStr};

/// The counted limits a plan card lists, each explained on the card. Snapshot retention is a
/// plan feature (`Feature::SnapshotRetentionDays`) and carries its own description there.
#[derive(Debug, Clone, Copy, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum PlanLimit {
    Seats,
    Sites,
    /// Production servers one license key covers (self-hosted plans).
    Servers,
    /// Staging, test and recovery servers one license key covers (self-hosted plans).
    NonProductionServers,
}

impl HasId for PlanLimit {
    fn id(&self) -> &'static str {
        self.into()
    }
}

impl EntityMetadataProvider for PlanLimit {
    fn color(&self) -> Color {
        Color::Gray
    }

    fn icon(&self) -> Icon {
        match self {
            Self::Seats => Icon::Users,
            Self::Sites => Icon::LandPlot,
            Self::Servers => Icon::Server,
            Self::NonProductionServers => Icon::FlaskConical,
        }
    }
}

impl TypeMetadataProvider for PlanLimit {
    fn name(&self) -> &'static str {
        match self {
            Self::Seats => "Seats",
            Self::Sites => "Sites",
            Self::Servers => "Servers",
            Self::NonProductionServers => "Non-production servers",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Self::Seats => {
                "Each user in your organization uses one seat. Daemons and API keys don't use seats."
            }
            Self::Sites => {
                "The locations your network runs in, such as an office or a data center. Each site holds its own hosts, services and topology."
            }
            Self::Servers => {
                "Production Scanopy servers this license key covers. Every container and replica on one database counts as one server."
            }
            Self::NonProductionServers => {
                "Servers for staging, testing or recovery, started with SCANOPY_NON_PRODUCTION=true. They count against their own limit."
            }
        }
    }
}
