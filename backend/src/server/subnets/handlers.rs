use crate::server::auth::middleware::auth::AuthenticatedEntity;
use crate::server::auth::middleware::permissions::{Authorized, IsDaemon, Member, Or, Viewer};
use crate::server::ip_addresses::r#impl::base::IPAddress;
use crate::server::openapi::tags as api_tags;
use crate::server::shared::attribution::AttributeSource;
use crate::server::shared::extractors::Query;
use crate::server::shared::handlers::list::list_page;
use crate::server::shared::handlers::ordering::OrderField;
use crate::server::shared::handlers::query::{
    FilterQueryExtractor, OrderDirection, PaginationParams, on_requested_sites,
};
use crate::server::shared::handlers::traits::{CrudHandlers, ListScope, update_handler};
use crate::server::shared::services::traits::CrudService;
use crate::server::shared::services::traits::EventBusService;
use crate::server::shared::storage::filter::StorableFilter;
use crate::server::shared::storage::traits::{Entity, Storable};
use crate::server::shared::types::api::{
    ApiError, ApiErrorResponse, ApiJson, ApiResponse, ApiResult, PaginatedApiResponse,
};
use crate::server::shared::types::entities::{EntitySource, EntitySourceDiscriminants};
use crate::server::shared::validation::validate_read_access;
use crate::server::sites::r#impl::Site;
use crate::server::{
    config::AppState,
    subnets::r#impl::base::{Subnet, SubnetCidr, SubnetName, SubnetTypeAttributed},
    subnets::r#impl::nesting::SubnetResponse,
};
use axum::extract::{Path, State};
use axum::response::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::IntoParams;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

// ============================================================================
// Subnet Ordering
// ============================================================================

/// Fields that subnets can be ordered/grouped by.
#[derive(
    Serialize, Deserialize, Debug, Clone, Copy, Default, utoipa::ToSchema, strum::EnumIter,
)]
#[serde(rename_all = "snake_case")]
pub enum SubnetOrderField {
    #[default]
    CreatedAt,
    Name,
    /// The range, in address order. Grouping by it nests each range under the ranges that
    /// contain it, one group per outermost range.
    Cidr,
    SubnetType,
    UpdatedAt,
    SiteId,
    /// Sort by when discovery last observed the subnet. Surfaces stale assets.
    LastSeenAt,
    /// How the subnet came to exist (`source.type`).
    Source,
    /// The container runtime that created a bridge subnet. Other subnets have none and sort last.
    ManagedBy,
    /// How full the range is, its nested ranges' addresses included. Derived in code, so it
    /// orders in memory.
    Utilization,
}

/// The range as an address, so `10.0.2.0/24` sorts before `10.0.16.0/20`. Stored as JSON text.
const CIDR_SQL: &str = "((subnets.cidr::jsonb #>> '{}')::cidr)";

impl OrderField for SubnetOrderField {
    fn to_sql(&self) -> &'static str {
        match self {
            Self::CreatedAt => "subnets.created_at",
            Self::Name => "subnets.name",
            Self::Cidr => CIDR_SQL,
            Self::SubnetType => "subnets.subnet_type",
            Self::UpdatedAt => "subnets.updated_at",
            Self::SiteId => "subnets.site_id",
            Self::LastSeenAt => "subnets.last_seen_at",
            Self::Source => "subnets.source->>'type'",
            Self::ManagedBy => "subnets.virtualization_service_id",
            // Never read: utilization is computed.
            Self::Utilization => "subnets.id",
        }
    }

    fn nulls_last(&self) -> bool {
        matches!(self, Self::ManagedBy)
    }
}

// ============================================================================
// Subnet Filter Query
// ============================================================================

/// Query parameters for filtering and ordering subnets.
#[derive(Deserialize, Default, Debug, Clone, IntoParams)]
pub struct SubnetFilterQuery {
    /// Filter by specific entity IDs (for selective loading)
    pub ids: Option<Vec<Uuid>>,
    /// Filter by site ID. Repeat the parameter to pass several.
    #[serde(alias = "site_id")]
    pub site_ids: Option<Vec<Uuid>>,
    /// Only subnets of one of these types. Repeat for several.
    pub subnet_types: Option<Vec<String>>,
    /// Only subnets that came to exist one of these ways (`source.type`). Repeat for several.
    pub sources: Option<Vec<EntitySourceDiscriminants>>,
    /// Only bridge subnets created by one of these container runtimes. Repeat for several.
    pub managed_by: Option<Vec<Uuid>>,
    /// `true` returns only the subnets the user curates, leaving out the ones Scanopy fabricates
    /// (`Subnet::is_user_managed`).
    pub user_managed: Option<bool>,
    /// Filter by tag IDs (returns subnets that have ANY of the specified tags)
    pub tag_ids: Option<Vec<Uuid>>,
    /// Tag sets the subnet must hold one tag from each of. Each entry is a comma-separated list
    /// of tag ids. One entry per tag group the list is filtered by.
    pub tag_sets: Option<Vec<String>>,
    /// Primary ordering field (used for grouping). Always sorts ASC to keep groups together.
    pub group_by: Option<SubnetOrderField>,
    /// Secondary ordering field (sorting within groups or standalone sort).
    pub order_by: Option<SubnetOrderField>,
    /// Group by the tag each row holds from this tag group. Takes the place of `group_by`.
    pub group_by_tag_group: Option<Uuid>,
    /// Sort by the position of the tag each row holds from this tag group. Takes the place of
    /// `order_by`.
    pub order_by_tag_group: Option<Uuid>,
    /// Direction for order_by field (group_by always uses ASC).
    pub order_direction: Option<OrderDirection>,
    /// Maximum number of results to return (1-1000, default: 50). Use 0 for no limit.
    #[param(minimum = 0, maximum = 1000)]
    pub limit: Option<u32>,
    /// Number of results to skip. Default: 0.
    #[param(minimum = 0)]
    pub offset: Option<u32>,
    /// As-of timestamp (ISO 8601). When set, returns SCD2 state as of this
    /// instant (snapshot view) instead of live state.
    pub at: Option<chrono::DateTime<chrono::Utc>>,
    /// `true` returns only subnets discovery hasn't observed within their
    /// site's staleness window; `false` returns only those it has. Omit for
    /// both. Evaluated per row against the subnet's own site's window.
    pub stale: Option<bool>,
    /// Free-text search. Case-insensitive substring match against the subnet's
    /// name, CIDR and description.
    pub search: Option<String>,
}

impl FilterQueryExtractor for SubnetFilterQuery {
    fn apply_to_filter<T: Storable>(
        &self,
        filter: StorableFilter<T>,
        user_site_ids: &[Uuid],
        _user_organization_id: Uuid,
    ) -> StorableFilter<T> {
        let filter = match &self.ids {
            Some(ids) if !ids.is_empty() => filter.entity_ids(ids),
            _ => filter,
        };
        let filter = on_requested_sites(filter, self.site_ids.as_deref(), user_site_ids);
        let filter = match &self.subnet_types {
            Some(types) if !types.is_empty() => filter.text_column_in("subnet_type", types),
            _ => filter,
        };
        let filter = match &self.sources {
            Some(sources) if !sources.is_empty() => filter.source_type_in(sources),
            _ => filter,
        };
        let filter = match &self.managed_by {
            Some(ids) if !ids.is_empty() => filter.expr_text_in(
                &format!("{}.virtualization_service_id", T::table_name()),
                &ids.iter().map(Uuid::to_string).collect::<Vec<_>>(),
            ),
            _ => filter,
        };
        // Here so the field values, which share this extractor, count only the ranges listed.
        match self.user_managed {
            Some(true) => filter.user_managed(),
            _ => filter,
        }
    }

    fn pagination(&self) -> PaginationParams {
        PaginationParams {
            limit: self.limit,
            offset: self.offset,
        }
    }

    fn at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.at
    }
}

// Generated handlers for most CRUD operations
mod generated {
    use super::*;
    crate::crud_delete_handler!(Subnet);
    crate::crud_bulk_delete_handler!(Subnet);
    crate::crud_export_csv_handler!(Subnet);
    crate::crud_get_field_values_handler!(Subnet);
}

pub fn create_router() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new()
        .routes(routes!(get_all_subnets, create_subnet))
        .routes(routes!(get_subnet_by_id, update_subnet, generated::delete))
        .routes(routes!(merge_subnet))
        .routes(routes!(generated::bulk_delete))
        .routes(routes!(generated::export_csv))
        .routes(routes!(generated::get_field_values))
}

/// Get subnet by ID
///
/// Returns the subnet with its utilization and the range it sits inside, if any.
#[utoipa::path(
    get,
    path = "/{id}",
    tag = Subnet::ENTITY_NAME_PLURAL,
    operation_id = "get_subnet_by_id",
    summary = "Get subnet by ID",
    params(("id" = Uuid, Path, description = "Subnet ID")),
    responses(
        (status = 200, description = "Subnet found", body = ApiResponse<SubnetResponse>),
        (status = 404, description = "Subnet not found", body = ApiErrorResponse),
    ),
    security(("user_api_key" = []), ("session" = []))
)]
async fn get_subnet_by_id(
    state: State<Arc<AppState>>,
    auth: Authorized<Viewer>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<SubnetResponse>>> {
    let site_ids = auth.site_ids();
    let organization_id = auth
        .organization_id()
        .ok_or_else(ApiError::organization_required)?;

    let service = &state.services.subnet_service;
    let subnet = service
        .get_by_id(&id)
        .await?
        // SCD2: closed historical copies are not addressable here, as in `get_by_id_handler`.
        .filter(|s| s.is_live_row())
        .ok_or_else(|| ApiError::entity_not_found::<Subnet>(id))?;

    validate_read_access(
        service.get_site_id(&subnet),
        service.get_organization_id(&subnet),
        &site_ids,
        organization_id,
    )?;

    let response = service
        .with_usage(vec![subnet], None)
        .await?
        .pop()
        .ok_or_else(|| ApiError::entity_not_found::<Subnet>(id))?;
    Ok(Json(ApiResponse::success(response)))
}

/// List subnets
///
/// Returns all subnets accessible to the authenticated user or daemon.
/// Daemons can only access subnets within their assigned site.
/// Supports pagination via `limit` and `offset` query parameters,
/// and ordering via `group_by`, `order_by`, and `order_direction`.
#[utoipa::path(
    get,
    path = "",
    tag = Subnet::ENTITY_NAME_PLURAL,
    operation_id = "list_subnets",
    summary = "List subnets",
    params(SubnetFilterQuery),
    responses(
        (status = 200, description = "List of subnets", body = PaginatedApiResponse<SubnetResponse>),
    ),
    security( ("user_api_key" = []),("session" = []), ("daemon_api_key" = []))
)]
async fn get_all_subnets(
    state: State<Arc<AppState>>,
    auth: Authorized<Or<Viewer, IsDaemon>>,
    Query(query): Query<SubnetFilterQuery>,
) -> ApiResult<Json<PaginatedApiResponse<SubnetResponse>>> {
    let site_ids = auth.site_ids();
    let organization_id = auth.organization_id();
    let entity = auth.into_entity();

    match entity {
        AuthenticatedEntity::Daemon { site_id, .. } => {
            // Daemons can only access subnets in their site
            // Return all results (no pagination applied). SCD2: live rows only —
            // daemons operate on current state and must never see closed copies.
            let filter = StorableFilter::<Subnet>::new_from_site_ids(&[site_id]).live();
            let service = Subnet::get_service(&state);
            let result = service.get_all(filter).await.map_err(|e| {
                tracing::error!(
                    error = %e,
                    site_id = %site_id,
                    "Failed to fetch subnets for daemon"
                );
                ApiError::internal_error(&e.to_string())
            })?;
            let total_count = result.len() as u64;
            let result = service.with_usage(result, None).await?;
            Ok(Json(PaginatedApiResponse::success(
                result,
                total_count,
                0,
                0,
            )))
        }
        _ => {
            // Users and API keys read one page of the list.
            let organization_id = organization_id.ok_or_else(ApiError::organization_required)?;
            let scope = ListScope {
                site_ids,
                organization_id,
                user_id: entity.user_id(),
                permissions: entity.permissions(),
            };
            Ok(Json(list_page::<Subnet>(&state, &scope, &query).await?))
        }
    }
}

/// Create subnet
#[utoipa::path(
    post,
    path = "",
    tag = Subnet::ENTITY_NAME_PLURAL,
    request_body = Subnet,
    responses(
        (status = 200, description = "Subnet created successfully", body = ApiResponse<Subnet>),
        (status = 400, description = "Invalid request", body = ApiErrorResponse),
    ),
    security( ("user_api_key" = []),("session" = []), ("daemon_api_key" = []))
)]
async fn create_subnet(
    state: State<Arc<AppState>>,
    auth: Authorized<Or<Member, IsDaemon>>,
    ApiJson(mut request): ApiJson<Subnet>,
) -> ApiResult<Json<ApiResponse<Subnet>>> {
    let site_ids = auth.site_ids();
    let entity = auth.into_entity();

    if let Err(err) = request.validate() {
        tracing::warn!(
            subnet_name = %request.base.name,
            subnet_cidr = %request.base.cidr,
            entity_id = %entity.entity_id().unwrap_or_default(),
            error = %err,
            "Subnet validation failed"
        );
        return Err(ApiError::bad_request(&format!(
            "Subnet validation failed: {}",
            err
        )));
    }

    let created = match &entity {
        AuthenticatedEntity::Daemon { site_id, .. } => {
            if *site_id == request.base.site_id {
                let service = Subnet::get_service(&state);
                let created = service.create(request, entity).await.map_err(|e| {
                    tracing::error!(
                        error = %e,
                        "Failed to create subnet"
                    );
                    ApiError::internal_error(&e.to_string())
                })?;
                Json(ApiResponse::success(created))
            } else {
                return Err(ApiError::entity_site_mismatch::<Subnet>());
            }
        }
        _ => {
            // User/API key - validate site access and create
            if !site_ids.contains(&request.base.site_id) {
                return Err(ApiError::entity_access_denied::<Site>(request.base.site_id));
            }
            // A range a person typed is an assertion, not a reading: `set_source` settles the
            // confidence ladder along with the source, so nothing a later scan reads displaces it.
            // Every other entity gets this from `create_handler`; subnets miss it because this
            // handler is written out longhand for the daemon case above, and never picked it up.
            // Stamped server-side rather than trusted from the body, so a client cannot claim
            // `Discovery` for a row it typed.
            request.set_source(EntitySource::Manual);
            let service = Subnet::get_service(&state);
            let created = service.create(request, entity).await.map_err(|e| {
                tracing::error!(error = %e, "Failed to create subnet");
                ApiError::internal_error(&e.to_string())
            })?;
            Json(ApiResponse::success(created))
        }
    };

    Ok(created)
}

/// Update subnet
///
/// Updates subnet properties. If the CIDR is being changed, validates that
/// all existing ip_addresses on this subnet have IPs within the new CIDR range.
#[utoipa::path(
    put,
    path = "/{id}",
    tag = Subnet::ENTITY_NAME_PLURAL,
    params(("id" = Uuid, Path, description = "Subnet ID")),
    request_body = Subnet,
    responses(
        (status = 200, description = "Subnet updated", body = ApiResponse<Subnet>),
        (status = 400, description = "CIDR change would orphan existing ip_addresses", body = ApiErrorResponse),
        (status = 404, description = "Subnet not found", body = ApiErrorResponse),
    ),
     security(("user_api_key" = []), ("session" = []))
)]
async fn update_subnet(
    State(state): State<Arc<AppState>>,
    auth: Authorized<Member>,
    Path(id): Path<Uuid>,
    ApiJson(mut subnet): ApiJson<Subnet>,
) -> ApiResult<Json<ApiResponse<Subnet>>> {
    // Check if CIDR is being changed
    let current = state
        .services
        .subnet_service
        .get_by_id(&id)
        .await
        .map_err(|e| ApiError::internal_error(&e.to_string()))?
        .ok_or_else(|| ApiError::entity_not_found::<Subnet>(id))?;

    // A name or type a person changed is theirs, for the reason the range below is. One sent back
    // unchanged keeps the stored source: saving a description is not renaming the subnet.
    if current.base.name.value() != subnet.base.name.value() {
        subnet.base.name =
            SubnetName::new(subnet.base.name.value().clone(), AttributeSource::Manual);
    }
    if current.base.subnet_type.value() != subnet.base.subnet_type.value() {
        subnet.base.subnet_type =
            SubnetTypeAttributed::new(*subnet.base.subnet_type.value(), AttributeSource::Manual);
    }

    if current.base.cidr != subnet.base.cidr {
        // Moving the range through this endpoint is a person's assertion, so it settles the
        // confidence ladder: `Confirmed` outranks anything a scan reads, which is what stops a
        // corrected range being re-corrected on the next discovery. This lives here rather than in
        // `preserve_immutable_fields` because here it is *true* — the route is `Authorized<Member>`
        // and no daemon can reach it — whereas storage sees every writer alike.
        subnet.base.cidr =
            SubnetCidr::new(subnet.base.cidr.value().clone(), AttributeSource::Manual);

        // CIDR is changing - validate that all existing ip_addresses are within the new CIDR
        let filter = StorableFilter::<IPAddress>::new_from_subnet_id(&id);
        let ip_addresses = state
            .services
            .ip_address_service
            .get_all(filter)
            .await
            .map_err(|e| ApiError::internal_error(&e.to_string()))?;

        for ip_address in &ip_addresses {
            if !subnet.base.cidr.contains(&ip_address.base.ip_address) {
                return Err(ApiError::ip_address_out_of_range(
                    &ip_address.base.ip_address.to_string(),
                    &subnet.base.cidr.to_string(),
                ));
            }
        }
    }

    // Delegate to generic handler
    update_handler::<Subnet>(State(state), auth, Path(id), ApiJson(subnet)).await
}

/// Request body for merging a subnet into the range that contains it.
#[derive(Debug, Deserialize, Serialize, utoipa::ToSchema)]
pub struct MergeSubnetRequest {
    /// The subnet to merge into. Must contain the range being merged.
    pub into: Uuid,
}

/// Merge subnet into its covering range
///
/// Moves every address to the covering subnet and removes this one. Offered for a range Scanopy
/// assumed that a later reading turned out to cover: discovery corrects such a range on its own
/// only where the answer is unambiguous, and folding several assumed ranges into one means deleting
/// rows, which is a person's call rather than a scan's.
#[utoipa::path(
    post,
    path = "/{id}/merge",
    tags = [Subnet::ENTITY_NAME_PLURAL, api_tags::INTERNAL],
    params(("id" = Uuid, Path, description = "Subnet ID to merge away")),
    request_body = MergeSubnetRequest,
    responses(
        (status = 200, description = "Subnet merged", body = ApiResponse<Subnet>),
        (status = 400, description = "The target does not contain this subnet", body = ApiErrorResponse),
        (status = 404, description = "Subnet not found", body = ApiErrorResponse),
    ),
     security(("user_api_key" = []), ("session" = []))
)]
async fn merge_subnet(
    State(state): State<Arc<AppState>>,
    auth: Authorized<Member>,
    Path(id): Path<Uuid>,
    ApiJson(request): ApiJson<MergeSubnetRequest>,
) -> ApiResult<Json<ApiResponse<Subnet>>> {
    let merged = state
        .services
        .subnet_service
        .merge_into(id, request.into, auth.entity.clone())
        .await
        .map_err(|e| ApiError::bad_request(&e.to_string()))?;

    Ok(Json(ApiResponse::success(merged)))
}
