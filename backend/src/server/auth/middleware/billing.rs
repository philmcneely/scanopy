//! Billing middleware that checks subscription status for authenticated requests.
//!
//! This middleware examines each request and:
//! - For authenticated requests (user, API key, daemon): Checks billing status, returns 402 if not active
//! - For unauthenticated requests: Passes through (handler auth will reject if needed)
//!
//! Exemptions (always allowed regardless of billing):
//! - Community, Free, CommercialSelfHosted, and Demo plans
//! - Self-hosted instances (no stripe_secret configured)
//!
//! A plan without the topology view (Free) gets 402 on the topology and
//! snapshot routes.
//!
//! Orgs on a self-hosted license plan (SelfHostedStandard / SelfHostedPlus)
//! run Scanopy on their own servers, so on the cloud they only reach the
//! routes the Settings modal needs; everything else returns 403.
//!
//! A lapsed org (a license subscription ended, no paid plan chosen since)
//! keeps its plan and can read everything, but every mutating request outside
//! the Settings routes returns 402 until it chooses a paid plan. A lapsed cloud
//! plan lands on Free instead.

use crate::server::{
    auth::middleware::{auth::AuthenticatedEntity, cache::CachedSite},
    billing::types::{base::BillingPlan, features::Feature},
    config::AppState,
    shared::types::{api::ApiError, error_codes::ErrorCode, metadata::TypeMetadataProvider},
};
use axum::{
    body::Body,
    extract::{FromRequestParts, State},
    http::{Method, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::sync::Arc;
use uuid::Uuid;

/// Middleware that enforces billing requirements for authenticated requests.
///
/// Apply this middleware at the router level to groups of routes that require billing.
/// Checks billing for users (session), user API keys, and daemons.
/// Caches looked-up entities in request extensions for subsequent handlers.
pub async fn require_billing_for_users(
    State(state): State<Arc<AppState>>,
    request: Request<Body>,
    next: Next,
) -> Response {
    // Plan limits apply on cloud, and on a self-hosted server that enforces
    // them for testing.
    if !state.config.enforces_plan_limits() {
        return next.run(request).await;
    }

    // Split request to access parts for caching
    let (mut parts, body) = request.into_parts();

    // Extract authenticated entity (cached in extensions)
    let entity = AuthenticatedEntity::from_request_parts(&mut parts, &state)
        .await
        .ok();

    // Get organization ID based on auth type, caching site lookup for daemons
    let organization_id: Option<Uuid> = match &entity {
        Some(AuthenticatedEntity::User {
            organization_id, ..
        }) => Some(*organization_id),
        Some(AuthenticatedEntity::ApiKey {
            organization_id, ..
        }) => Some(*organization_id),
        Some(AuthenticatedEntity::Daemon { site_id, .. }) => {
            // Use cached site lookup
            match CachedSite::get_or_load(&mut parts, &state, site_id).await {
                Ok(site) => Some(site.base.organization_id),
                Err(_) => None,
            }
        }
        _ => None,
    };

    let Some(organization_id) = organization_id else {
        // Not authenticated or no org - pass through (handler auth will reject if needed)
        let request = Request::from_parts(parts, body);
        return next.run(request).await;
    };

    // Cached organization for the request — used by both this middleware
    // and any downstream feature checks.
    let organization =
        match super::cache::CachedOrganization::get_or_load(&mut parts, &state, &organization_id)
            .await
        {
            Ok(org) => org,
            Err(e) => return e.into_response(),
        };

    // Reassemble request with cached entities in extensions
    let request = Request::from_parts(parts, body);

    let plan = organization.base.plan;

    // A license plan, trialing, paid or not yet started, opens only the
    // Settings routes on Scanopy Cloud: its app runs on the org's own server.
    if plan.license_plan().is_some() {
        if settings_route(request.method(), request.uri().path()) {
            return next.run(request).await;
        }
        return ApiError::self_hosted_plan_locked().into_response();
    }
    // The topology view and its snapshots need a plan that includes them. A plan
    // without it can still read the diagram, which the app draws behind its
    // plan prompt.
    if topology_route(request.uri().path())
        && !plan.features().topology_view
        && !topology_preview_route(request.method(), request.uri().path())
    {
        return ApiError::coded(
            StatusCode::PAYMENT_REQUIRED,
            ErrorCode::BillingFeatureNotAvailable {
                feature: Feature::TopologyView.name().to_string(),
            },
        )
        .into_response();
    }
    if matches!(
        plan,
        BillingPlan::Community(_)
            | BillingPlan::Free(_)
            | BillingPlan::CommercialSelfHosted(_)
            | BillingPlan::Demo(_)
    ) {
        return next.run(request).await;
    }

    // Check subscription status. None = no subscription yet (must select a
    // plan); Cancelled = the subscription ended and the org kept its plan, so
    // it is read-only until it chooses a paid plan; everything else allows
    // the request through (Active / Trialing / PendingCancellation / PastDue
    // / Paused all keep features available — Paused is a Stripe collection
    // pause, not a feature lockout).
    if organization.is_lapsed() {
        if request.method().is_safe() || settings_route(request.method(), request.uri().path()) {
            return next.run(request).await;
        }
        return ApiError::billing_plan_lapsed().into_response();
    }
    match organization.base.plan_status {
        Some(_) => next.run(request).await,
        None => ApiError::billing_required().into_response(),
    }
}

/// The topology view's routes and the snapshots captured from it, which need
/// the topology view on the plan.
fn topology_route(path: &str) -> bool {
    ["/api/v1/topologies", "/api/v1/snapshots"]
        .iter()
        .any(|prefix| {
            path.strip_prefix(prefix)
                .is_some_and(|rest| rest.is_empty() || rest.starts_with('/'))
        })
}

/// The reads that draw the topology: the list, the graph data and one
/// topology. Open to a plan without the topology view, which sees the diagram
/// behind its plan prompt. The stream, saving options, exports and snapshots
/// stay closed to it.
fn topology_preview_route(method: &Method, path: &str) -> bool {
    const TOPOLOGY: &str = "/api/v1/topologies";
    *method == Method::GET
        && (path == TOPOLOGY
            || path
                .strip_prefix(TOPOLOGY)
                .and_then(|rest| rest.strip_prefix('/'))
                .is_some_and(|rest| !rest.is_empty() && !rest.contains('/') && rest != "stream"))
}

/// Billed routes the Settings modal calls, which stay open to an org on a
/// self-hosted license plan, to a lapsed org, and to a self-hosted server
/// with a locked license: reading, renaming, and deleting the org, and
/// editing the current user. Everything else Settings needs (auth, billing,
/// licenses, config) sits outside this middleware already.
pub(crate) fn settings_route(method: &Method, path: &str) -> bool {
    const ORGANIZATIONS: &str = "/api/v1/organizations";
    const CURRENT_ORGANIZATION: &str = "/api/v1/organizations/current";
    const USERS: &str = "/api/v1/users";
    let single_entity = |prefix: &str| {
        path.strip_prefix(prefix)
            .and_then(|rest| rest.strip_prefix('/'))
            .is_some_and(|id| !id.is_empty() && !id.contains('/'))
    };

    match *method {
        Method::GET => path == CURRENT_ORGANIZATION,
        Method::PUT => single_entity(ORGANIZATIONS) || single_entity(USERS),
        Method::DELETE => single_entity(ORGANIZATIONS),
        _ => false,
    }
}
