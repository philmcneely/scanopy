//! How the logging subscriber writes one event: [`EventLog`], decided once where the event is
//! built, and [`changes`], the field diff an entity update logs instead of the whole entity.

use serde::Serialize;
use serde_json::{Map, Value, json};
use treediff::{
    tools::{ChangeType, Recorder},
    value::Key,
};

use uuid::Uuid;

use crate::server::shared::events::types::{EventLogLevel, UpdateReason};

/// An entity's logging policy for its updates, applied by the CRUD layer, the one place that
/// holds both versions. Creates and deletes log the whole entity (a delete with its reason line),
/// from [`EntityOperation`](super::types::EntityOperation)'s own `Operation::log`.
pub trait EntityLog: Serialize + Sized {
    /// Paths [`changes`] leaves out for this entity, beyond `updated_at`: fields that move on
    /// every check-in and say nothing on their own.
    const LOG_IGNORED_PATHS: &'static [&'static str] = &[];

    /// How the update of `before` into `self` is logged. Default: the fields that changed, or
    /// nothing when none did. An entity overrides it to add a line for a transition that would
    /// get lost in the changes.
    fn update_log(&self, before: &Self, id: Uuid, reason: &UpdateReason) -> EventLog {
        changes_log(before, self, id, reason, Self::LOG_IGNORED_PATHS)
    }
}

/// The update's id, reason and changed fields, or [`EventLog::hidden`] when nothing changed.
pub fn changes_log<T: Serialize>(
    before: &T,
    after: &T,
    id: Uuid,
    reason: &UpdateReason,
    ignore: &[&str],
) -> EventLog {
    let changes = changes(before, after, ignore);
    if changes.is_empty() {
        return EventLog::hidden();
    }
    EventLog {
        body: LogBody::Changes(json!({ "id": id, "reason": reason, "changes": changes })),
        line: None,
    }
}

/// How the logging subscriber writes one event. Built with the event, never serialized, so
/// nothing in it reaches PostHog.
#[derive(Debug, Clone, Default)]
pub struct EventLog {
    pub body: LogBody,
    pub line: Option<LogLine>,
}

/// What the event's JSON line holds.
#[derive(Debug, Clone, Default)]
pub enum LogBody {
    /// The whole event.
    #[default]
    Full,
    /// The entity's id, name and reason, and only the fields that changed.
    Changes(Value),
    /// No JSON line: an update that changed nothing, or one too frequent to log (a heartbeat).
    Hidden,
}

/// A line a person can read, written after the body at its own level: a transition that would
/// get lost in a payload, or a fact the JSON doesn't say plainly.
#[derive(Debug, Clone)]
pub struct LogLine {
    pub level: EventLogLevel,
    pub text: String,
}

impl EventLog {
    pub fn hidden() -> Self {
        Self {
            body: LogBody::Hidden,
            line: None,
        }
    }

    pub fn with_line(mut self, level: EventLogLevel, text: impl Into<String>) -> Self {
        self.line = Some(LogLine {
            level,
            text: text.into(),
        });
        self
    }

    /// Nothing will be written for this event.
    pub fn is_silent(&self) -> bool {
        matches!(self.body, LogBody::Hidden) && self.line.is_none()
    }
}

/// The fields that differ between `before` and `after`, as dotted paths mapped to
/// `{"from", "to"}`. An added field has no `from`; a removed one has no `to`.
///
/// Arrays whose items all carry an `id` are matched by it, so one inserted row reads as one
/// addition rather than every later index changing. Paths ending in `updated_at`, which every
/// write moves, and paths in `ignore` are left out.
pub fn changes<T: Serialize>(before: &T, after: &T, ignore: &[&str]) -> Map<String, Value> {
    let (Ok(before), Ok(after)) = (serde_json::to_value(before), serde_json::to_value(after))
    else {
        return Map::new();
    };
    let (before, after) = (key_rows_by_id(before), key_rows_by_id(after));

    let mut recorder = Recorder::default();
    treediff::diff(&before, &after, &mut recorder);

    let mut out = Map::new();
    for change in recorder.calls {
        let (path, entry) = match change {
            ChangeType::Unchanged(..) => continue,
            ChangeType::Added(path, to) => (path, json!({ "to": to })),
            ChangeType::Removed(path, from) => (path, json!({ "from": from })),
            ChangeType::Modified(path, from, to) => (path, json!({ "from": from, "to": to })),
        };
        let path = dotted(&path);
        if path.is_empty()
            || path == "updated_at"
            || path.ends_with(".updated_at")
            || ignore.contains(&path.as_str())
        {
            continue;
        }
        out.insert(path, entry);
    }
    out
}

/// Arrays of objects that each carry an `id`, as objects keyed by that id.
fn key_rows_by_id(value: Value) -> Value {
    match value {
        Value::Array(items)
            if !items.is_empty()
                && items
                    .iter()
                    .all(|item| item.get("id").is_some_and(|id| id.is_string())) =>
        {
            Value::Object(
                items
                    .into_iter()
                    .map(|item| {
                        let id = item["id"].as_str().unwrap_or_default().to_string();
                        (id, key_rows_by_id(item))
                    })
                    .collect(),
            )
        }
        Value::Array(items) => Value::Array(items.into_iter().map(key_rows_by_id).collect()),
        Value::Object(map) => Value::Object(
            map.into_iter()
                .map(|(key, value)| (key, key_rows_by_id(value)))
                .collect(),
        ),
        other => other,
    }
}

fn dotted(path: &[Key]) -> String {
    path.iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(".")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn daemon_like(unreachable: bool, last_seen: &str, bindings: Value) -> Value {
        json!({
            "id": "d1",
            "updated_at": last_seen,
            "base": { "name": "d", "is_unreachable": unreachable, "last_seen": last_seen },
            "bindings": bindings,
        })
    }

    #[test]
    fn equal_values_have_no_changes() {
        let a = daemon_like(false, "t1", json!([]));
        assert!(changes(&a, &a.clone(), &[]).is_empty());
    }

    #[test]
    fn a_nested_change_is_reported_at_its_own_path_and_nothing_else() {
        let before = daemon_like(false, "t1", json!([]));
        let after = daemon_like(true, "t1", json!([]));
        let diff = changes(&before, &after, &[]);
        assert_eq!(diff.keys().collect::<Vec<_>>(), vec!["base.is_unreachable"]);
        assert_eq!(diff["base.is_unreachable"]["from"], json!(false));
        assert_eq!(diff["base.is_unreachable"]["to"], json!(true));
    }

    #[test]
    fn ignored_paths_and_updated_at_never_appear() {
        let before = daemon_like(false, "t1", json!([]));
        let after = daemon_like(false, "t2", json!([]));
        assert!(changes(&before, &after, &["base.last_seen"]).is_empty());
    }

    #[test]
    fn rows_with_ids_are_matched_by_id_not_position() {
        let row = |id: &str, port: u16| json!({ "id": id, "port": port });
        let before = daemon_like(false, "t1", json!([row("a", 1), row("b", 2)]));
        let after = daemon_like(false, "t1", json!([row("c", 3), row("a", 1), row("b", 2)]));
        let diff = changes(&before, &after, &[]);
        assert_eq!(diff.keys().collect::<Vec<_>>(), vec!["bindings.c"]);
        assert!(diff["bindings.c"].get("from").is_none());
    }
}
