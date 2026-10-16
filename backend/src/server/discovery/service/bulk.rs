//! What starting, cancelling or scheduling a discovery on request requires, shared by the single
//! and bulk endpoints, and the order a bulk cancel runs in.
use super::*;

/// Why a discovery can't be started on request, if it can't.
pub fn start_refusal(discovery: &Discovery, running: bool) -> Option<StartRefusal> {
    if discovery.base.run_type.is_server_managed() {
        Some(StartRefusal::PastRun)
    } else if discovery
        .base
        .discovery_type
        .rescan_target_host_ids()
        .is_some()
    {
        Some(StartRefusal::Rescan)
    } else if running {
        Some(StartRefusal::Running)
    } else {
        None
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum StartRefusal {
    /// A historical record of a finished run.
    PastRun,
    /// A rescan, which the server starts once when it creates it.
    Rescan,
    /// Already has a session.
    Running,
}

impl StartRefusal {
    pub fn into_api_error(self, name: &str) -> ApiError {
        match self {
            StartRefusal::PastRun => {
                ApiError::bad_request(&format!("\"{name}\" is a past run and can't be started."))
            }
            StartRefusal::Rescan => ApiError::bad_request(&format!(
                "\"{name}\" is a rescan, which runs once and can't be started again."
            )),
            StartRefusal::Running => {
                ApiError::conflict(&format!("\"{name}\" already has a session running."))
            }
        }
    }
}

/// Why a discovery's schedule can't be turned on or off, if it can't.
pub fn set_enabled_refusal(discovery: &Discovery, running: bool) -> Option<ApiError> {
    let name = &discovery.base.name;
    if !matches!(discovery.base.run_type, RunType::Scheduled { .. }) {
        Some(ApiError::bad_request(&format!(
            "\"{name}\" has no schedule to turn on or off."
        )))
    } else if running {
        Some(ApiError::conflict(&format!(
            "\"{name}\" is running. Turn its schedule on or off once it finishes."
        )))
    } else {
        None
    }
}

/// Puts sessions that haven't reached their daemon first. Cancelling a running session promotes
/// the next one queued on its daemon, which then starts; when that one is also being cancelled,
/// cancelling it first keeps it from starting at all.
pub fn order_for_cancel<T>(items: &mut [T], phase: impl Fn(&T) -> DiscoveryPhase) {
    items.sort_by_key(|item| {
        !matches!(
            phase(item),
            DiscoveryPhase::Queued | DiscoveryPhase::Pending | DiscoveryPhase::AwaitingSnapshot
        )
    });
}

impl DiscoveryService {
    /// The session running for this discovery, if any.
    pub async fn session_for_discovery(
        &self,
        discovery_id: &Uuid,
    ) -> Option<DiscoveryUpdatePayload> {
        let session_id = *self.discovery_sessions.read().await.get(discovery_id)?;
        self.get_session(&session_id).await
    }

    /// Turns a scheduled discovery's schedule on or off. Goes through `update`, so the cron job
    /// is added or removed and the Updated event published, and changes nothing else: a stale
    /// copy of the rest of the row can't overwrite it.
    pub async fn set_enabled(
        &self,
        mut discovery: Discovery,
        enabled: bool,
        authentication: AuthenticatedEntity,
    ) -> Result<Discovery> {
        if let RunType::Scheduled {
            enabled: current, ..
        } = &mut discovery.base.run_type
        {
            *current = enabled;
        }
        self.update(&mut discovery, authentication).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::shared::types::examples;

    fn scheduled() -> Discovery {
        let mut discovery = examples::discovery();
        discovery.base.run_type = RunType::Scheduled {
            cron_schedule: "0 0 * * *".to_string(),
            last_run: None,
            enabled: true,
            timezone: None,
        };
        discovery
    }

    #[test]
    fn only_an_idle_configured_discovery_starts_on_request() {
        let mut rescan = examples::discovery();
        rescan.base.discovery_type = DiscoveryType::Rescan {
            host_id: Uuid::nil(),
            target_host_ids: vec![Uuid::nil()],
            ips: vec![],
            ports: vec![],
            settings: Default::default(),
        };
        assert_eq!(start_refusal(&examples::discovery(), false), None);
        assert_eq!(
            start_refusal(&examples::discovery(), true),
            Some(StartRefusal::Running)
        );
        assert_eq!(start_refusal(&rescan, false), Some(StartRefusal::Rescan));
    }

    #[test]
    fn only_an_idle_scheduled_discovery_turns_its_schedule_on_or_off() {
        assert!(set_enabled_refusal(&scheduled(), false).is_none());
        assert!(set_enabled_refusal(&scheduled(), true).is_some());
        assert!(set_enabled_refusal(&examples::discovery(), false).is_some());
    }

    #[test]
    fn a_bulk_cancel_reaches_undispatched_sessions_before_running_ones() {
        let mut phases = vec![
            DiscoveryPhase::Scanning,
            DiscoveryPhase::Queued,
            DiscoveryPhase::Started,
            DiscoveryPhase::Pending,
        ];
        order_for_cancel(&mut phases, |p| *p);
        assert_eq!(
            phases,
            vec![
                DiscoveryPhase::Queued,
                DiscoveryPhase::Pending,
                DiscoveryPhase::Scanning,
                DiscoveryPhase::Started,
            ]
        );
    }
}
