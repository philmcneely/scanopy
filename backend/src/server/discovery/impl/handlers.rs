use crate::server::{
    config::AppState,
    discovery::handlers::DiscoveryFilterQuery,
    discovery::{r#impl::base::Discovery, service::DiscoveryService},
    shared::{
        handlers::{
            list::{ListOrdering, PaginatedList},
            query::narrow_by_tags_and_search,
            traits::{CrudHandlers, base_list_filter},
        },
        storage::filter::StorableFilter,
        types::api::ApiResult,
    },
};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
impl CrudHandlers for Discovery {
    type Service = DiscoveryService;
    type FilterQuery = DiscoveryFilterQuery;
    type OrderField = crate::server::discovery::handlers::DiscoveryOrderField;

    fn get_service(state: &AppState) -> &Self::Service {
        &state.services.discovery_service
    }

    fn search_order() -> Option<Self::OrderField> {
        Some(Self::OrderField::Name)
    }

    /// Access, the query's own fields and search, shared by the list and the CSV export.
    async fn list_filter(
        _state: &AppState,
        query: &DiscoveryFilterQuery,
        site_ids: &[Uuid],
        organization_id: Uuid,
    ) -> ApiResult<StorableFilter<Discovery>> {
        narrow_by_tags_and_search(
            base_list_filter::<Discovery>(site_ids, organization_id, query),
            None,
            None,
            query.search.as_deref(),
        )
    }
}

#[async_trait]
impl PaginatedList for Discovery {
    type Item = Discovery;

    fn ordering(query: &DiscoveryFilterQuery) -> ListOrdering<Self::OrderField> {
        ListOrdering {
            group_by: query.group_by,
            order_by: query.order_by,
            direction: query.order_direction,
            tag_group: Default::default(),
        }
    }

    fn default_order() -> String {
        "discovery.created_at DESC".to_string()
    }

    async fn items(
        _state: &AppState,
        rows: Vec<Discovery>,
        _query: &DiscoveryFilterQuery,
    ) -> ApiResult<Vec<Discovery>> {
        Ok(rows)
    }
}
