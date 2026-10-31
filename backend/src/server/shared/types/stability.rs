use serde::{Deserialize, Serialize};
use strum::IntoStaticStr;
use utoipa::ToSchema;

/// Release maturity of something Scanopy ships: a credential type's integration, or a plan
/// feature.
///
/// Each owner declares it in an exhaustive match, so promoting one to `Stable` is a one-line
/// reviewable change. This is presentation metadata about the *code*, like
/// `last_unsupported_daemon_version`. It is never stored on a row, so it carries no
/// deploy-coexistence obligation.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, IntoStaticStr, ToSchema, PartialEq, Eq)]
pub enum Stability {
    /// Generally available.
    Stable,
    /// Shipped for validation, or open to beta program members first. Behaviour and data shape
    /// may change in a future release. Clearly marked in the UI.
    Beta,
}

/// `Beta < Stable`. Not derived: declaration order puts `Stable` first, which would order
/// these the other way round.
impl PartialOrd for Stability {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;
        Some(match (self, other) {
            (Self::Beta, Self::Stable) => Ordering::Less,
            (Self::Stable, Self::Beta) => Ordering::Greater,
            (Self::Beta, Self::Beta) | (Self::Stable, Self::Stable) => Ordering::Equal,
        })
    }
}
