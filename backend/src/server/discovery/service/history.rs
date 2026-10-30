//! Reading a discovery's run history.
use super::*;

impl DiscoveryService {
    /// The name a run's historical row is recorded under: its discovery's name, frozen at scan
    /// time, so the history and anything quoting it read as what was true then even if the
    /// discovery is later renamed or removed. A rescan's transient parent was minted as
    /// "Rescan of <host> (<ip>)", the only place host name and address are both known, and is
    /// deleted moments after. A run whose discovery is already gone falls back to a name from its
    /// type.
    pub(crate) async fn historical_run_name(
        &self,
        session: &DiscoveryUpdatePayload,
        discovery_id: Option<Uuid>,
    ) -> String {
        if let Some(discovery_id) = discovery_id
            && let Ok(Some(discovery)) = self.discovery_storage.get_by_id(&discovery_id).await
        {
            return discovery.base.name;
        }
        if matches!(session.discovery_type, DiscoveryType::Unified { .. }) {
            return "Discovery".to_string();
        }
        let site_name = match self.site_service.get_by_id(&session.site_id).await {
            Ok(Some(site)) => site.base.name,
            _ => "Unknown Site".to_string(),
        };
        format!("{} \u{2014} {}", session.discovery_type, site_name)
    }

    /// The historical rows of a streak of `runs` failed runs in a row, newest first, when the run
    /// recorded for `session_id` is the one that brought its discovery's streak to `runs`. `None`
    /// otherwise, including for every later failure in the same streak, so a caller acting on it
    /// acts once per streak.
    ///
    /// Reads the run history, so call it once the session's historical row is written.
    pub async fn failure_streak(
        &self,
        session_id: Uuid,
        runs: usize,
    ) -> Result<Option<Vec<Discovery>>> {
        let filter = StorableFilter::<Discovery>::new()
            .historical_session(session_id)
            .limit(1);
        let Some(discovery_id) = self
            .discovery_storage
            .get_all(filter)
            .await?
            .first()
            .and_then(|run| run.base.run_type.historical_results())
            .and_then(|results| results.discovery_id)
        else {
            return Ok(None);
        };

        let filter = StorableFilter::<Discovery>::new()
            .historical_runs_of(discovery_id)
            .limit(runs as u32 + 1);
        let history = self
            .discovery_storage
            .get_all_ordered(filter, "created_at DESC")
            .await?;

        // The newest run must be this session's: a later run already recorded has moved the
        // streak on, and it acts for itself.
        let newest_is_this_session = history
            .first()
            .and_then(|run| run.base.run_type.historical_results())
            .is_some_and(|results| results.session_id == session_id);
        let phases: Vec<DiscoveryPhase> = history
            .iter()
            .filter_map(|run| run.base.run_type.historical_results())
            .map(|results| results.phase)
            .collect();
        if !newest_is_this_session || !streak_just_reached(&phases, runs) {
            return Ok(None);
        }
        Ok(Some(history.into_iter().take(runs).collect()))
    }
}

/// Whether the newest `runs` outcomes, newest first, all failed while the one before them, if
/// there is one, did not.
fn streak_just_reached(newest_first: &[DiscoveryPhase], runs: usize) -> bool {
    let failed = |phase: &DiscoveryPhase| *phase == DiscoveryPhase::Failed;
    newest_first.len() >= runs
        && newest_first[..runs].iter().all(failed)
        && !newest_first.get(runs).is_some_and(failed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use DiscoveryPhase::{Cancelled, Complete, Failed};

    #[test]
    fn a_streak_is_reached_by_the_failure_that_completes_it() {
        assert!(streak_just_reached(&[Failed, Failed, Failed, Complete], 3));
        assert!(streak_just_reached(&[Failed, Failed, Failed], 3));
    }

    #[test]
    fn later_failures_in_a_streak_do_not_reach_it_again() {
        assert!(!streak_just_reached(&[Failed, Failed, Failed, Failed], 3));
    }

    #[test]
    fn a_run_that_did_not_fail_breaks_the_streak() {
        assert!(!streak_just_reached(
            &[Failed, Cancelled, Failed, Failed],
            3
        ));
        assert!(!streak_just_reached(&[Failed, Failed, Complete], 3));
        assert!(!streak_just_reached(&[Failed, Failed], 3));
    }
}
