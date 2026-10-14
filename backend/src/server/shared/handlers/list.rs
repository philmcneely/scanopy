//! The paginated list behind every entity table: one page of rows in the list's order, with the
//! full size of every group the page cuts.
//!
//! An entity opts in by implementing [`PaginatedList`]; its list handler is then a call to
//! [`list_page`]. Filtering stays with [`CrudHandlers::list_filter`], so the CSV export and the
//! list read the same rows for the same query.

use std::collections::HashMap;
use std::fmt::Display;

use async_trait::async_trait;
use serde::Serialize;
use uuid::Uuid;

use crate::server::{
    config::AppState,
    shared::{
        entities::{ChangeTriggersTopologyStaleness, Entity as EntityEnum},
        handlers::{
            ordering::{OrderField, TagGroupOrdering},
            query::{FilterQueryExtractor, OrderDirection},
            traits::{CrudHandlers, ListScope},
            tree::{TreePosition, tree_order},
        },
        services::traits::CrudService,
        storage::{
            filter::StorableFilter,
            traits::{Entity, Storage},
        },
        types::api::{ApiResult, GroupCount, PaginatedApiResponse},
    },
};

/// How a list request orders its rows: the grouping, the sort inside it, and either one taken
/// from a tag group in their place.
#[derive(Debug, Clone, Copy, Default)]
pub struct ListOrdering<O> {
    pub group_by: Option<O>,
    pub order_by: Option<O>,
    pub direction: Option<OrderDirection>,
    pub tag_group: TagGroupOrdering,
}

impl<O: OrderField> ListOrdering<O> {
    /// The expression the list is grouped on, which is also the key of each entry in
    /// `group_counts`. `None` when the list isn't grouped.
    pub fn group_sql(&self) -> Option<&'static str> {
        match (self.tag_group.group_by, self.group_by) {
            (Some(_), _) => Some(TagGroupOrdering::GROUP_SQL),
            (None, Some(field)) => Some(field.to_sql()),
            (None, None) => None,
        }
    }

    /// The ORDER BY for this ordering, with the JOINs it reads from added to `filter`.
    pub fn apply<T: Entity>(
        &self,
        filter: StorableFilter<T>,
        default_order: &str,
    ) -> (StorableFilter<T>, String) {
        self.tag_group.apply(
            T::entity_type(),
            self.group_by,
            self.order_by,
            self.direction,
            filter,
            default_order,
        )
    }

    /// This ordering without its grouping: the sort alone.
    fn sort_only(&self) -> Self {
        Self {
            group_by: None,
            tag_group: TagGroupOrdering {
                group_by: None,
                order_by: self.tag_group.order_by,
            },
            ..*self
        }
    }
}

/// An entity whose list is paged, sorted, grouped and counted on the server.
#[async_trait]
pub trait PaginatedList: CrudHandlers + 'static
where
    Self: Display + ChangeTriggersTopologyStaleness<Self> + Default,
    EntityEnum: From<Self>,
{
    /// What one row of the list is sent as.
    type Item: Serialize + Send;

    /// How `query` orders the list.
    fn ordering(query: &Self::FilterQuery) -> ListOrdering<Self::OrderField>;

    /// The order of a list nobody sorted or grouped.
    fn default_order() -> String {
        format!("{}.created_at ASC", Self::table_name())
    }

    /// The rows of one page as the list sends them, in the order given.
    async fn items(
        state: &AppState,
        rows: Vec<Self>,
        query: &Self::FilterQuery,
    ) -> ApiResult<Vec<Self::Item>>;

    /// Whether grouping by `field` nests the rows as trees, one group per root.
    fn groups_as_tree(field: &Self::OrderField) -> bool {
        let _ = field;
        false
    }

    /// Where each of `ids` sits in its tree. A row in no tree has no entry. Called only for a list
    /// grouped by a field [`Self::groups_as_tree`] accepts.
    async fn tree_positions(
        state: &AppState,
        ids: &[Uuid],
    ) -> ApiResult<HashMap<Uuid, TreePosition>> {
        let _ = (state, ids);
        Ok(HashMap::new())
    }
}

/// One page of `T`'s list for this caller and query, with `group_counts` when it is grouped.
pub async fn list_page<T>(
    state: &AppState,
    scope: &ListScope,
    query: &T::FilterQuery,
) -> ApiResult<PaginatedApiResponse<T::Item>>
where
    T: PaginatedList + ChangeTriggersTopologyStaleness<T> + Default,
    EntityEnum: From<T>,
{
    let filter = T::list_filter(state, query, scope).await?;
    let pagination = query.pagination();
    let limit = pagination.effective_limit();
    let offset = pagination.effective_offset();
    let ordering = T::ordering(query);

    let tree_grouped = ordering.tag_group.group_by.is_none()
        && ordering
            .group_by
            .is_some_and(|field| T::groups_as_tree(&field));

    let (rows, total_count, group_counts) = if tree_grouped {
        tree_page::<T>(state, filter, ordering, limit, offset).await?
    } else {
        let (filter, order) =
            ordering.apply(pagination.apply_to_filter(filter), &T::default_order());
        let service = T::get_service(state);
        // Each group's full size, not the slice of it on this page. Counted under the same
        // filter, JOINs included, since the group expression may read from one.
        let group_counts = match ordering.group_sql() {
            Some(group_sql) => Some(service.count_by_group(filter.clone(), group_sql).await?),
            None => None,
        };
        let page = service.get_paginated_ordered(filter, &order).await?;
        (page.items, page.total_count, group_counts)
    };

    let items = T::items(state, rows, query).await?;
    let response = PaginatedApiResponse::success(items, total_count, limit.unwrap_or(0), offset);
    Ok(match group_counts {
        Some(counts) => response.with_group_counts(counts),
        None => response,
    })
}

/// A page of a list grouped by tree. SQL can't place a row under a parent it hasn't read, so
/// the order is built here from every matching row's id, and the page is sliced from it: each
/// parent leads its subtree, and siblings follow the sort. With no sort, siblings follow the
/// order global search lists the entity in.
async fn tree_page<T>(
    state: &AppState,
    filter: StorableFilter<T>,
    ordering: ListOrdering<T::OrderField>,
    limit: Option<u32>,
    offset: u32,
) -> ApiResult<(Vec<T>, u64, Option<Vec<GroupCount>>)>
where
    T: PaginatedList + ChangeTriggersTopologyStaleness<T> + Default,
    EntityEnum: From<T>,
{
    let sort = ListOrdering {
        order_by: ordering.order_by.or_else(T::search_order),
        ..ordering.sort_only()
    };
    let (filter, order) = sort.apply(filter, &T::default_order());
    let service = T::get_service(state);
    let ids: Vec<Uuid> = service
        .storage()
        .get_ordered_values(filter, &order, &[])
        .await?
        .into_iter()
        .map(|(id, _)| id)
        .collect();

    let positions = T::tree_positions(state, &ids).await?;
    let rows: Vec<(Uuid, String, Option<Uuid>)> = ids
        .iter()
        .map(|id| {
            let position = positions.get(id);
            (
                *id,
                TreePosition::group_key(position),
                position.and_then(|p| p.parent_id),
            )
        })
        .collect();
    let ordered = tree_order(&rows);

    let key_of: HashMap<Uuid, &str> = rows
        .iter()
        .map(|(id, key, _)| (*id, key.as_str()))
        .collect();
    let mut group_counts: Vec<GroupCount> = Vec::new();
    for id in &ordered {
        let key = key_of.get(id).copied().unwrap_or_default();
        match group_counts.last_mut() {
            Some(last) if last.value.as_deref() == Some(key) => last.count += 1,
            _ => group_counts.push(GroupCount {
                value: Some(key.to_string()),
                count: 1,
            }),
        }
    }

    let page: Vec<Uuid> = ordered
        .into_iter()
        .skip(offset as usize)
        .take(limit.map_or(usize::MAX, |l| l as usize))
        .collect();
    let mut rows = service
        .get_all(StorableFilter::<T>::new_from_entity_ids(&page))
        .await?;
    let rank: HashMap<Uuid, usize> = page.iter().enumerate().map(|(i, id)| (*id, i)).collect();
    rows.sort_by_key(|row| rank.get(&row.id()).copied().unwrap_or(usize::MAX));

    Ok((rows, ids.len() as u64, Some(group_counts)))
}
