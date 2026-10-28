use crate::server::daemons::r#impl::api::DiscoveryUpdatePayload;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// An event on the live-update stream (`GET /api/v1/live-updates/stream`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
#[serde(tag = "type", content = "data")]
pub enum LiveUpdate {
    /// Progress or an end state of a discovery session.
    Discovery(DiscoveryUpdatePayload),
    /// A site's live entity set changed, so its topology needs refetching.
    Topology(LiveTopologyUpdate),
}

/// A site whose live topology changed.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct LiveTopologyUpdate {
    /// The site whose live entities changed.
    pub site_id: Uuid,
}

impl LiveUpdate {
    pub fn site_id(&self) -> Uuid {
        match self {
            LiveUpdate::Discovery(update) => update.site_id,
            LiveUpdate::Topology(update) => update.site_id,
        }
    }

    /// Whether a subscriber should get this event: it must be on one of their sites, and topology
    /// events need the topology view on their plan.
    pub fn is_visible_to(&self, site_ids: &[Uuid], topology_on_plan: bool) -> bool {
        if matches!(self, LiveUpdate::Topology(_)) && !topology_on_plan {
            return false;
        }
        site_ids.contains(&self.site_id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::discovery::r#impl::types::DiscoveryType;

    fn discovery(site_id: Uuid) -> LiveUpdate {
        LiveUpdate::Discovery(DiscoveryUpdatePayload::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            site_id,
            DiscoveryType::default(),
            None,
        ))
    }

    fn topology(site_id: Uuid) -> LiveUpdate {
        LiveUpdate::Topology(LiveTopologyUpdate { site_id })
    }

    #[test]
    fn subscribers_get_only_their_sites() {
        let mine = Uuid::new_v4();
        let other = Uuid::new_v4();

        for update in [discovery(mine), topology(mine)] {
            assert!(update.is_visible_to(&[mine], true));
        }
        for update in [discovery(other), topology(other)] {
            assert!(!update.is_visible_to(&[mine], true));
        }
    }

    #[test]
    fn a_plan_without_topology_gets_discovery_only() {
        let site = Uuid::new_v4();

        assert!(discovery(site).is_visible_to(&[site], false));
        assert!(!topology(site).is_visible_to(&[site], false));
    }
}
