//! Legacy daemon wire transformations.
//!
//! Cleanup: remove [`rewrite_response_for_legacy_daemon`] once minimum_supported >= 0.16.0, and
//! the network-wire rewrites once the floor passes `last_network_wire()`, and the single-target
//! rescan rewrites once it passes `last_single_target_rescan()`.
//!
//! The network-wire rename is done here, on whole JSON bodies, rather than with
//! `#[serde(alias = "network_id")]` on each `site_id` field: serde ignores an alias on a field of
//! a `#[serde(flatten)]`ed struct (serde-rs/serde#1504), and every entity flattens its base.

use axum::{
    body::{Body, to_bytes},
    extract::Request,
    http::{HeaderMap, StatusCode, header},
    middleware::Next,
    response::Response,
};

use crate::server::daemons::r#impl::version::{reads_single_target_rescan, speaks_network_wire};

/// The server side of the network wire. A daemon's request body is read with
/// [`rewrite_from_network_wire`] whatever its version (one built on site names sends none of the
/// old ones), and its JSON response is written with [`rewrite_for_network_wire`] when it is at or
/// below `last_network_wire()`, and with [`rewrite_for_single_target_rescan`] when it is at or
/// below `last_single_target_rescan()`. A daemon is recognised by its `X-Daemon-ID` header, which every
/// supported version sends; one without `X-Daemon-Version` predates 0.14.10 and is old enough.
pub async fn network_wire_middleware(request: Request, next: Next) -> Response {
    let headers = request.headers();
    if !headers.contains_key("X-Daemon-ID") {
        return next.run(request).await;
    }
    let version = headers
        .get("X-Daemon-Version")
        .and_then(|v| v.to_str().ok());
    let network_wire = speaks_network_wire(version);
    let single_target_rescan = reads_single_target_rescan(version);

    let request = match rewrite_request(request, rewrite_from_network_wire).await {
        Ok(request) => request,
        Err(response) => return response,
    };
    let response = next.run(request).await;
    if !(network_wire || single_target_rescan) || !is_json(response.headers()) {
        return response;
    }

    let (mut parts, body) = response.into_parts();
    let rewrite = |json: &mut serde_json::Value| {
        if network_wire {
            rewrite_for_network_wire(json);
        }
        if single_target_rescan {
            rewrite_for_single_target_rescan(json);
        }
    };
    match rewrite_body(body, rewrite).await {
        Ok(body) => {
            parts.headers.remove(header::CONTENT_LENGTH);
            Response::from_parts(parts, body)
        }
        Err(response) => response,
    }
}

/// The daemon side of the wire: every request the server sends a daemon is read with
/// [`rewrite_from_server_wire`], since a server that predates the rename sends `network_id`, and
/// one that sends this daemon's version a single rescan target adds `target_host_id`.
pub async fn site_wire_request_middleware(request: Request, next: Next) -> Response {
    match rewrite_request(request, rewrite_from_server_wire).await {
        Ok(request) => next.run(request).await,
        Err(response) => response,
    }
}

fn is_json(headers: &HeaderMap) -> bool {
    headers
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|ct| ct.starts_with("application/json"))
}

async fn rewrite_request(
    request: Request,
    rewrite: fn(&mut serde_json::Value),
) -> Result<Request, Response> {
    if !is_json(request.headers()) {
        return Ok(request);
    }
    let (mut parts, body) = request.into_parts();
    let body = rewrite_body(body, rewrite).await?;
    parts.headers.remove(header::CONTENT_LENGTH);
    Ok(Request::from_parts(parts, body))
}

/// The body with `rewrite` applied, or unchanged when it is not JSON.
async fn rewrite_body(
    body: Body,
    rewrite: impl FnOnce(&mut serde_json::Value),
) -> Result<Body, Response> {
    let bytes = to_bytes(body, usize::MAX).await.map_err(|e| {
        tracing::error!(error = %e, "Could not buffer a body for the network wire");
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::empty())
            .unwrap_or_default()
    })?;
    let Ok(mut json) = serde_json::from_slice::<serde_json::Value>(&bytes) else {
        return Ok(Body::from(bytes));
    };
    rewrite(&mut json);
    Ok(serde_json::to_vec(&json).map_or(Body::from(bytes), Body::from))
}

/// Rewrite JSON bound for a daemon at or below `last_network_wire()` to the names it was built
/// with: every `site_id` key becomes `network_id`, and an integration target's `"scope":"Site"`
/// becomes `"Network"`. Those daemons require `network_id` on every entity they parse, so without
/// this they fail to read registration, work and discovery responses.
pub fn rewrite_for_network_wire(value: &mut serde_json::Value) {
    rename_wire_keys(value, ("site_id", "network_id"), ("Site", "Network"));
}

/// Read JSON written with network-era names as the current types expect it: every `network_id`
/// key becomes `site_id`, and an integration target's `"scope":"Network"` becomes `"Site"`. The
/// inverse of [`rewrite_for_network_wire`].
pub fn rewrite_from_network_wire(value: &mut serde_json::Value) {
    rename_wire_keys(value, ("network_id", "site_id"), ("Network", "Site"));
}

/// Rewrite JSON bound for a daemon at or below `last_single_target_rescan()`: every rescan
/// (`"type":"Rescan"`) also gets the single `target_host_id` those daemons require, set to its
/// first target. They ignore `target_host_ids` and scan only `ips`, which already holds every
/// target's addresses.
pub fn rewrite_for_single_target_rescan(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            if map.get("type").and_then(|v| v.as_str()) == Some("Rescan")
                && let Some(first) = map
                    .get("target_host_ids")
                    .and_then(|ids| ids.as_array())
                    .and_then(|ids| ids.first())
                    .cloned()
            {
                map.insert("target_host_id".to_string(), first);
            }
            for v in map.values_mut() {
                rewrite_for_single_target_rescan(v);
            }
        }
        serde_json::Value::Array(arr) => {
            for v in arr {
                rewrite_for_single_target_rescan(v);
            }
        }
        _ => {}
    }
}

/// Read a rescan the way the current types expect it: when it carries both `target_host_ids`
/// and the single `target_host_id` [`rewrite_for_single_target_rescan`] adds, drop the single
/// one. A daemon built from unreleased code still reports the last released version, so it gets
/// both, and serde refuses a field together with its alias.
pub fn rewrite_from_single_target_rescan(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            if map.get("type").and_then(|v| v.as_str()) == Some("Rescan")
                && map.contains_key("target_host_ids")
            {
                map.remove("target_host_id");
            }
            for v in map.values_mut() {
                rewrite_from_single_target_rescan(v);
            }
        }
        serde_json::Value::Array(arr) => {
            for v in arr {
                rewrite_from_single_target_rescan(v);
            }
        }
        _ => {}
    }
}

/// Everything a daemon reads from the server, in today's names: [`rewrite_from_network_wire`]
/// and [`rewrite_from_single_target_rescan`].
pub fn rewrite_from_server_wire(value: &mut serde_json::Value) {
    rewrite_from_network_wire(value);
    rewrite_from_single_target_rescan(value);
}

fn rename_wire_keys(value: &mut serde_json::Value, key: (&str, &str), scope: (&str, &str)) {
    match value {
        serde_json::Value::Object(map) => {
            if let Some(val) = map.remove(key.0) {
                map.insert(key.1.to_string(), val);
            }
            if map.get("scope").and_then(|v| v.as_str()) == Some(scope.0) {
                map.insert(
                    "scope".to_string(),
                    serde_json::Value::String(scope.1.to_string()),
                );
            }
            for v in map.values_mut() {
                rename_wire_keys(v, key, scope);
            }
        }
        serde_json::Value::Array(arr) => {
            for v in arr {
                rename_wire_keys(v, key, scope);
            }
        }
        _ => {}
    }
}

/// Rewrite response JSON for pre-0.16.0 daemons to use old entity/field names:
/// - BindingType "IPAddress" → "Interface", "ip_address_id" → "interface_id"
/// - HostResponse field "ip_addresses" → "interfaces", "interfaces" → "if_entries"
pub fn rewrite_response_for_legacy_daemon(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            // Rename HostResponse child entity fields:
            // current "ip_addresses" → old "interfaces", current "interfaces" → old "if_entries"
            // Order matters: rename interfaces→if_entries first, then ip_addresses→interfaces
            if let Some(val) = map.remove("interfaces") {
                map.insert("if_entries".to_string(), val);
            }
            if let Some(val) = map.remove("ip_addresses") {
                map.insert("interfaces".to_string(), val);
            }

            // Rewrite BindingType: "IPAddress" → "Interface"
            if map.get("type").and_then(|v| v.as_str()) == Some("IPAddress") {
                map.insert(
                    "type".to_string(),
                    serde_json::Value::String("Interface".to_string()),
                );
            }

            // Rewrite ip_address_id → interface_id in bindings (IPAddress and Port variants)
            if map.contains_key("type")
                && let Some(val) = map.remove("ip_address_id")
            {
                map.insert("interface_id".to_string(), val);
            }

            // Recurse into all values
            for v in map.values_mut() {
                rewrite_response_for_legacy_daemon(v);
            }
        }
        serde_json::Value::Array(arr) => {
            for v in arr {
                rewrite_response_for_legacy_daemon(v);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::daemon::runtime::state::DaemonStatus;
    use crate::server::daemons::r#impl::api::{DaemonRegistrationRequest, DiscoveryUpdatePayload};
    use crate::server::daemons::r#impl::version::last_network_wire;
    use crate::server::hosts::r#impl::api::DiscoveryHostRequest;
    use crate::server::shared::types::api::ApiJson;
    use crate::server::subnets::r#impl::base::Subnet;
    use axum::{
        Json, Router,
        http::Request,
        routing::{get, post},
    };
    use serde::Serialize;
    use serde::de::DeserializeOwned;
    use std::path::Path;
    use tower::Service;

    /// String values found under `key`, anywhere in `value`.
    fn values_under(value: &serde_json::Value, key: &str, out: &mut Vec<String>) {
        match value {
            serde_json::Value::Object(map) => {
                for (k, v) in map {
                    if k == key
                        && let Some(s) = v.as_str()
                    {
                        out.push(s.to_string());
                    }
                    values_under(v, key, out);
                }
            }
            serde_json::Value::Array(arr) => arr.iter().for_each(|v| values_under(v, key, out)),
            _ => {}
        }
    }

    /// Posts `body` as a daemon at `version` through [`network_wire_middleware`] to a handler that
    /// reads it with `ApiJson<T>`, as the real endpoint does, and returns what the handler read.
    async fn ingest<T: DeserializeOwned + Serialize + Send + 'static>(
        body: &serde_json::Value,
        version: &str,
        source: &str,
    ) -> serde_json::Value {
        let mut app =
            Router::new()
                .route(
                    "/",
                    // Plain text, so the response half of the middleware leaves the reading alone.
                    post(|ApiJson(read): ApiJson<T>| async move {
                        serde_json::to_string(&read).unwrap()
                    }),
                )
                .layer(axum::middleware::from_fn(network_wire_middleware));
        let request = Request::post("/")
            .header(DAEMON_ID.0, DAEMON_ID.1)
            .header("X-Daemon-Version", version)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(body.to_string()))
            .unwrap();
        // A plain Router is always ready, so it can be called without polling.
        let response = app.call(request).await.unwrap();
        let status = response.status();
        let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert!(
            status.is_success(),
            "{source}: the old shape was refused ({status}): {}",
            String::from_utf8_lossy(&bytes)
        );
        serde_json::from_slice(&bytes).unwrap()
    }

    /// Every request a captured daemon sent (0.13 onward) still reads into the type its endpoint
    /// takes, and every `network_id` it carried lands in `site_id`.
    #[tokio::test]
    async fn captured_daemon_requests_carry_network_id_into_site_id() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/integration/compat/fixtures");
        let mut checked = 0;
        for entry in std::fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path().join("daemon_to_server.json");
            let Ok(raw) = std::fs::read_to_string(&path) else {
                continue;
            };
            let manifest: serde_json::Value = serde_json::from_str(&raw).unwrap();
            let version = manifest["version"].as_str().unwrap();
            for exchange in manifest["exchanges"].as_array().unwrap() {
                let status = exchange["response_status"].as_u64().unwrap();
                if !(200..300).contains(&status) || exchange["method"] != "POST" {
                    continue;
                }
                let route = exchange["path"].as_str().unwrap();
                let body = &exchange["request_body"];
                let source = format!("{} {route}", path.display());
                let ingested = if route == "/api/daemons/register" {
                    ingest::<DaemonRegistrationRequest>(body, version, &source).await
                } else if route.ends_with("/request-work") {
                    ingest::<DaemonStatus>(body, version, &source).await
                } else if route.starts_with("/api/v1/discovery/") && route.ends_with("/update") {
                    ingest::<DiscoveryUpdatePayload>(body, version, &source).await
                } else if route == "/api/v1/hosts/discovery" {
                    ingest::<DiscoveryHostRequest>(body, version, &source).await
                } else if route == "/api/v1/subnets" {
                    ingest::<Subnet>(body, version, &source).await
                } else {
                    continue;
                };

                let mut sent = Vec::new();
                values_under(body, "network_id", &mut sent);
                let mut read = Vec::new();
                values_under(&ingested, "site_id", &mut read);
                for id in &sent {
                    assert!(
                        read.contains(id),
                        "{}: {route} sent network_id {id}, which did not reach site_id",
                        path.display()
                    );
                }
                checked += sent.len();
            }
        }
        assert!(checked > 0, "no captured request carried a network_id");
    }

    async fn served_to(headers: &[(&str, &str)]) -> serde_json::Value {
        let mut app = Router::new()
            .route(
                "/",
                get(|| async {
                    Json(serde_json::json!({
                        "data": {
                            "site_id": "7eb98070-8e88-4434-b534-e3edc542498c",
                            "integration_targets": [{
                                "scope": "Site",
                                "credential_id": "22222222-0000-0000-0000-000000000001"
                            }]
                        }
                    }))
                }),
            )
            .layer(axum::middleware::from_fn(network_wire_middleware));
        let mut request = Request::builder().uri("/");
        for (name, value) in headers {
            request = request.header(*name, *value);
        }
        // A plain Router is always ready, so it can be called without polling.
        let response = app
            .call(request.body(Body::empty()).unwrap())
            .await
            .unwrap();
        let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        serde_json::from_slice(&bytes).unwrap()
    }

    const DAEMON_ID: (&str, &str) = ("X-Daemon-ID", "375b4992-3f56-446f-ad5b-16002023b0dc");

    #[tokio::test]
    async fn daemons_up_to_the_last_network_release_are_sent_network_names() {
        let last = last_network_wire().to_string();
        for version in [Some("0.14.10"), Some(last.as_str()), None] {
            let mut headers = vec![DAEMON_ID];
            if let Some(v) = version {
                headers.push(("X-Daemon-Version", v));
            }
            let body = served_to(&headers).await;
            assert!(body["data"].get("site_id").is_none(), "{version:?}");
            assert_eq!(
                body["data"]["network_id"],
                "7eb98070-8e88-4434-b534-e3edc542498c"
            );
            assert_eq!(body["data"]["integration_targets"][0]["scope"], "Network");
        }
    }

    #[tokio::test]
    async fn newer_daemons_and_other_clients_are_sent_site_names() {
        let next = {
            let mut v = last_network_wire();
            v.patch += 1;
            v.to_string()
        };
        for headers in [vec![DAEMON_ID, ("X-Daemon-Version", next.as_str())], vec![]] {
            let body = served_to(&headers).await;
            assert_eq!(
                body["data"]["site_id"],
                "7eb98070-8e88-4434-b534-e3edc542498c"
            );
            assert!(body["data"].get("network_id").is_none());
            assert_eq!(body["data"]["integration_targets"][0]["scope"], "Site");
        }
    }

    /// A daemon built on site names reads what it is sent until its version passes
    /// `last_network_wire`: the round trip through both rewrites loses nothing.
    #[test]
    fn site_types_read_the_network_wire() {
        let subnet = Subnet::default();
        let mut wire = serde_json::to_value(&subnet).unwrap();
        rewrite_for_network_wire(&mut wire);
        assert!(wire.get("network_id").is_some());
        rewrite_from_network_wire(&mut wire);
        let read: Subnet = serde_json::from_value(wire).unwrap();
        assert_eq!(read.base.site_id, subnet.base.site_id);
    }
}

#[cfg(test)]
mod single_target_rescan_tests {
    use super::*;
    use crate::server::discovery::r#impl::types::DiscoveryType;
    use uuid::Uuid;

    fn rescan(targets: &[Uuid]) -> serde_json::Value {
        serde_json::json!({
            "discovery_type": {
                "type": "Rescan",
                "host_id": Uuid::nil(),
                "target_host_ids": targets,
                "ips": ["10.0.0.1", "10.0.0.2"],
            }
        })
    }

    /// The shape daemons at or below `last_single_target_rescan()` were built with.
    #[derive(serde::Deserialize)]
    #[serde(tag = "type")]
    enum OldDiscoveryType {
        Rescan { target_host_id: Uuid },
    }

    #[test]
    fn an_old_daemon_reads_a_multi_host_rescan() {
        let targets = [Uuid::new_v4(), Uuid::new_v4()];
        let mut wire = rescan(&targets);
        rewrite_for_single_target_rescan(&mut wire);
        let OldDiscoveryType::Rescan { target_host_id } =
            serde_json::from_value(wire["discovery_type"].clone()).unwrap();
        assert_eq!(target_host_id, targets[0]);
    }

    #[test]
    fn a_current_daemon_reads_the_rewritten_rescan_with_every_target() {
        let targets = [Uuid::new_v4(), Uuid::new_v4()];
        let mut wire = rescan(&targets);
        rewrite_for_single_target_rescan(&mut wire);
        rewrite_from_server_wire(&mut wire);
        let parsed: DiscoveryType = serde_json::from_value(wire["discovery_type"].clone()).unwrap();
        assert_eq!(parsed.rescan_target_host_ids(), Some(&targets[..]));
    }
}
