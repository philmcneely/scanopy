use crate::server::{
    auth::middleware::permissions::{Authorized, Viewer},
    config::AppState,
    live_updates::types::{LiveTopologyUpdate, LiveUpdate},
    shared::{
        services::traits::CrudService,
        sse::until_shutdown,
        types::api::{ApiError, ApiResult},
    },
};
use axum::{
    extract::State,
    response::{
        Sse,
        sse::{Event, KeepAlive},
    },
    routing::get,
};
use futures::Stream;
use std::{convert::Infallible, sync::Arc};
use tokio::sync::broadcast::error::RecvError;
use utoipa_axum::router::OpenApiRouter;

pub fn create_router() -> OpenApiRouter<Arc<AppState>> {
    // SSE endpoint (not well-supported by OpenAPI). `LiveUpdate` is registered in the spec's
    // components so the UI types its events from it.
    OpenApiRouter::new().route("/stream", get(live_updates_stream))
}

/// SSE stream of `LiveUpdate` events for the caller's sites: discovery progress, and topology
/// changes when the plan includes the topology view. The plan is read at connect, so the app
/// reconnects when the org's topology access changes.
async fn live_updates_stream(
    State(state): State<Arc<AppState>>,
    auth: Authorized<Viewer>,
) -> ApiResult<Sse<impl Stream<Item = Result<Event, Infallible>>>> {
    let organization_id = auth
        .organization_id()
        .ok_or_else(|| ApiError::internal_error("No organization"))?;
    let organization = state
        .services
        .organization_service
        .get_by_id(&organization_id)
        .await?
        .ok_or_else(|| ApiError::forbidden("Organization not found"))?;
    let topology_on_plan = !state.config.enforces_plan_limits()
        || organization.base.plan.features().includes_topology();

    let mut discovery_rx = state.services.discovery_service.subscribe();
    let mut topology_rx = state
        .services
        .topology_service
        .subscribe_live_topology_updates();
    let site_ids = auth.site_ids();

    let stream = async_stream::stream! {
        loop {
            let update = tokio::select! {
                received = discovery_rx.recv() => match received {
                    Ok(update) => LiveUpdate::Discovery(update),
                    Err(RecvError::Lagged(n)) => {
                        tracing::warn!("Live-update stream lagged by {} discovery messages", n);
                        continue;
                    }
                    Err(RecvError::Closed) => break,
                },
                received = topology_rx.recv() => match received {
                    Ok(site_id) => LiveUpdate::Topology(LiveTopologyUpdate { site_id }),
                    Err(RecvError::Lagged(n)) => {
                        tracing::warn!("Live-update stream lagged by {} topology messages", n);
                        continue;
                    }
                    Err(RecvError::Closed) => break,
                },
            };
            if update.is_visible_to(&site_ids, topology_on_plan) {
                let json = serde_json::to_string(&update).unwrap_or_default();
                yield Ok(Event::default().data(json));
            }
        }
    };

    Ok(Sse::new(until_shutdown(stream, &state.shutdown)).keep_alive(KeepAlive::default()))
}
