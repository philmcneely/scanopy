//! Global middleware that refuses a user session while one of the
//! session-wide gates holds:
//! - the user still has to accept the current terms
//!   ([`crate::server::auth::terms_gate`]);
//! - the user's org is on a license plan and they haven't verified their
//!   email ([`crate::server::auth::verification_gate`]).
//!
//! Only user sessions are gated. API keys, daemons and external services
//! pass through, and so do the routes the gates themselves need: auth
//! (current user, accept, verify, resend, email change, logout), config,
//! health, version, and the org read the app shell makes on every page.

use crate::server::{
    auth::{middleware::auth::AuthenticatedEntity, terms_gate, verification_gate},
    config::{AppState, DeploymentType, get_deployment_type},
    shared::types::api::ApiError,
};
use axum::{
    body::Body,
    extract::{FromRequestParts, State},
    http::{Method, Request},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::sync::Arc;

/// Requests a gated user can make.
fn allowed_while_gated(method: &Method, path: &str) -> bool {
    !path.starts_with("/api/")
        || path.starts_with("/api/auth/")
        || path == "/api/config"
        || path == "/api/health"
        || path == "/api/version"
        || (method == Method::GET && path == "/api/v1/organizations/current")
}

pub async fn session_gate_middleware(
    State(state): State<Arc<AppState>>,
    request: Request<Body>,
    next: Next,
) -> Response {
    // Both gates apply only on Scanopy Cloud.
    let deployment_type = get_deployment_type(&state.config);
    if deployment_type != DeploymentType::Cloud
        || allowed_while_gated(request.method(), request.uri().path())
    {
        return next.run(request).await;
    }

    let (mut parts, body) = request.into_parts();

    let Ok(AuthenticatedEntity::User {
        organization_id,
        email_verified,
        terms_accepted_at,
        ..
    }) = AuthenticatedEntity::from_request_parts(&mut parts, &state).await
    else {
        return next.run(Request::from_parts(parts, body)).await;
    };

    let organization =
        match super::cache::CachedOrganization::get_or_load(&mut parts, &state, &organization_id)
            .await
        {
            Ok(org) => org,
            Err(e) => return e.into_response(),
        };
    let plan = Some(&organization.base.plan);

    if terms_gate::acceptance_required(deployment_type, plan, terms_accepted_at) {
        return ApiError::terms_acceptance_required().into_response();
    }
    if verification_gate::verification_required(
        deployment_type,
        plan,
        email_verified,
        state.services.email_service.is_some(),
    ) {
        return ApiError::email_verification_required().into_response();
    }

    next.run(Request::from_parts(parts, body)).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_gate_leaves_open_only_what_the_gated_steps_need() {
        assert!(allowed_while_gated(&Method::POST, "/api/auth/me"));
        assert!(allowed_while_gated(&Method::POST, "/api/auth/accept-terms"));
        assert!(allowed_while_gated(
            &Method::POST,
            "/api/auth/resend-verification"
        ));
        assert!(allowed_while_gated(
            &Method::POST,
            "/api/auth/request-email-change"
        ));
        assert!(allowed_while_gated(&Method::POST, "/api/auth/logout"));
        assert!(allowed_while_gated(&Method::GET, "/api/config"));
        assert!(allowed_while_gated(
            &Method::GET,
            "/api/v1/organizations/current"
        ));
        assert!(allowed_while_gated(&Method::GET, "/index.html"));

        assert!(!allowed_while_gated(&Method::GET, "/api/v1/hosts"));
        assert!(!allowed_while_gated(&Method::GET, "/api/v1/shares"));
        assert!(!allowed_while_gated(
            &Method::GET,
            "/api/v1/licenses/keys/current"
        ));
        assert!(!allowed_while_gated(&Method::POST, "/api/billing/checkout"));
        let org = format!("/api/v1/organizations/{}", uuid::Uuid::nil());
        assert!(!allowed_while_gated(&Method::PUT, &org));
    }
}
