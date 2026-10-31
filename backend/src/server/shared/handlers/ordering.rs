//! Generic ordering support for entity queries.
//!
//! Provides the `OrderField` trait and `apply_ordering` function to eliminate
//! duplicated ordering logic across entity handlers.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::server::shared::{
    entities::EntityDiscriminants,
    storage::{filter::StorableFilter, traits::Storable},
};

use super::query::OrderDirection;

// ============================================================================
// OrderField Trait
// ============================================================================

/// The alias of the site a site-name sort joins.
const SITE_ALIAS: &str = "row_site";

/// Trait for order field enums that generate SQL ORDER BY expressions.
///
/// Implement this for entity-specific OrderField enums to enable generic
/// ordering via `apply_ordering()`.
pub trait OrderField: Clone + Copy + Default + Send + Sync + 'static {
    /// Returns the SQL ORDER BY expression for this field.
    ///
    /// The expression should be fully qualified with table name or alias.
    /// Examples: `"hosts.created_at"`, `"COALESCE(virt_service.name, '')"`
    fn to_sql(&self) -> &'static str;

    /// Returns the JOIN clause if this field requires one, None otherwise.
    ///
    /// Example: `"LEFT JOIN services AS virt_service ON ..."`
    fn join_sql(&self) -> Option<&'static str> {
        None
    }

    /// Whether rows with no value for this field sort last in both directions.
    ///
    /// Postgres puts NULLs last for ASC but first for DESC. A field whose NULL means "nothing to
    /// sort by" (e.g. a host with no MAC) returns true so those rows never lead a DESC listing.
    fn nulls_last(&self) -> bool {
        false
    }

    /// Whether this field is a site id that sorts by the site's name. `to_sql` stays the id, which
    /// keys the field's groups and filter values and breaks ties between sites sharing a name.
    fn sorts_by_site_name(&self) -> bool {
        false
    }

    /// The JOIN a site-name sort reads the name from. `None` for any other field.
    fn site_join(&self) -> Option<String> {
        self.sorts_by_site_name().then(|| {
            format!(
                "LEFT JOIN sites AS {SITE_ALIAS} ON {SITE_ALIAS}.id = {}",
                self.to_sql()
            )
        })
    }

    /// An expression a sort by this field leads with, ascending in either direction: rows it puts
    /// first stay first whichever way the field sorts. Not part of the grouping, where it would
    /// split a group.
    fn leading_sql(&self) -> Option<String> {
        None
    }

    /// The field's own ORDER BY term in the given direction.
    fn field_term(&self, dir: &str) -> String {
        if self.sorts_by_site_name() {
            format!("{SITE_ALIAS}.name {dir}, {} {dir}", self.to_sql())
        } else if self.nulls_last() {
            format!("{} {} NULLS LAST", self.to_sql(), dir)
        } else {
            format!("{} {}", self.to_sql(), dir)
        }
    }

    /// The field's ORDER BY term in the given direction.
    fn order_term(&self, dir: &str) -> String {
        match self.leading_sql() {
            Some(leading) => format!("{leading} ASC, {}", self.field_term(dir)),
            None => self.field_term(dir),
        }
    }

    /// The ORDER BY term that keeps this field's groups together.
    fn group_term(&self) -> String {
        self.field_term("ASC")
    }
}

/// The order field of an entity with no server-side ordering.
///
/// Its one variant is the order the generic `get_all_handler` already lists by, unqualified
/// because that handler's query carries no JOIN.
#[derive(
    Serialize, Deserialize, Debug, Clone, Copy, Default, utoipa::ToSchema, strum::EnumIter,
)]
#[serde(rename_all = "snake_case")]
pub enum NoOrderField {
    #[default]
    CreatedAt,
}

impl OrderField for NoOrderField {
    fn to_sql(&self) -> &'static str {
        match self {
            Self::CreatedAt => "created_at",
        }
    }
}

// ============================================================================
// Expressions derived from code
// ============================================================================

/// `CASE expr WHEN 'k' THEN 'v' … END`: a value derived in code (a credential type's integration,
/// say) as SQL a list can sort, group and filter on. Build `arms` by iterating the enum and calling
/// its exhaustive match, so the SQL can't drift from the code. Keys and values are identifiers
/// written in code, never request input, so they are inlined.
pub fn case_sql<'a>(expr: &str, arms: impl IntoIterator<Item = (&'a str, &'a str)>) -> String {
    let arms: String = arms
        .into_iter()
        .map(|(when, then)| format!(" WHEN '{when}' THEN '{then}'"))
        .collect();
    format!("(CASE {expr}{arms} END)")
}

/// `(expr IN ('a', …))`, a boolean derived in code (whether a credential type is in beta, say), with
/// the same rules as [`case_sql`]. `FALSE` when no value qualifies.
pub fn in_sql<'a>(expr: &str, values: impl IntoIterator<Item = &'a str>) -> String {
    let values: Vec<String> = values.into_iter().map(|v| format!("'{v}'")).collect();
    if values.is_empty() {
        "FALSE".to_string()
    } else {
        format!("({expr} IN ({}))", values.join(", "))
    }
}

// ============================================================================
// Generic apply_ordering Function
// ============================================================================

/// Apply ordering to a filter based on group_by, order_by, and direction.
///
/// This function handles:
/// - Adding JOINs required by order fields
/// - Avoiding duplicate JOINs when group_by and order_by use the same JOIN
/// - Building the ORDER BY clause with group_by first (always ASC) then order_by
/// - Ending a field ordering with the row id, so rows that tie on the field (every NULL, say)
///   keep one order across pages instead of repeating on one page and vanishing from the next
///
/// Returns: (modified_filter, order_by_sql)
pub fn apply_ordering<T, O>(
    group_by: Option<O>,
    order_by: Option<O>,
    direction: Option<OrderDirection>,
    mut filter: StorableFilter<T>,
    default_order: &str,
) -> (StorableFilter<T>, String)
where
    T: Storable,
    O: OrderField,
{
    let mut order_parts = Vec::new();
    // Each JOIN once: grouping and sorting by fields that read the same table share one.
    let mut joins: Vec<String> = Vec::new();
    for field in group_by.iter().chain(order_by.iter()) {
        for join in [field.join_sql().map(str::to_string), field.site_join()]
            .into_iter()
            .flatten()
        {
            if !joins.contains(&join) {
                joins.push(join);
            }
        }
    }
    for join in &joins {
        filter = filter.join(join);
    }

    // Primary: group_by field (always ASC to keep groups together)
    if let Some(group_field) = group_by {
        order_parts.push(group_field.group_term());
    }

    // Secondary: order_by field with specified direction
    if let Some(order_field) = order_by {
        let dir = direction.unwrap_or_default().to_sql();
        order_parts.push(order_field.order_term(dir));
    }

    let order_by_sql = if order_parts.is_empty() {
        default_order.to_string()
    } else {
        order_parts.push(format!("{}.id ASC", T::table_name()));
        order_parts.join(", ")
    };

    (filter, order_by_sql)
}

// ============================================================================
// Ordering by a tag group
// ============================================================================

/// Sorting or grouping a server-paged list by the tag each row holds from one tag group.
///
/// A tag group is data, not a column, so it can't be an [`OrderField`] variant: the group's id
/// travels as its own query parameter. The id is a parsed [`Uuid`] and the entity type an enum, so
/// both are written into the JOIN as literals.
#[derive(Debug, Clone, Copy, Default)]
pub struct TagGroupOrdering {
    /// Group rows by the tag they hold from this group, in the group's tag order.
    pub group_by: Option<Uuid>,
    /// Sort rows by the position of the tag they hold from this group.
    pub order_by: Option<Uuid>,
}

impl TagGroupOrdering {
    /// The expression a list grouped by a tag group is grouped on, and the key of each group in
    /// the response's `group_counts`: `<position padded to 10 digits>:<tag name>:<tag id>`. Leading
    /// with the position keeps groups in the tag group's order; an unordered group's tags have
    /// position 0, so they follow the name. Rows holding no tag of the group have no key and come
    /// last.
    pub const GROUP_SQL: &'static str = "tag_group_grouped.group_key";

    pub fn is_set(&self) -> bool {
        self.group_by.is_some() || self.order_by.is_some()
    }

    /// The JOIN exposing, as `alias`, the position and group key of the tag each row of `T`
    /// holds from `group_id`.
    fn join<T: Storable>(alias: &str, entity_type: EntityDiscriminants, group_id: Uuid) -> String {
        let table = T::table_name();
        // `entity_tags.entity_type` holds the JSON-encoded discriminant, as `SqlValue` binds it.
        let entity_type = serde_json::to_string(&entity_type).unwrap_or_default();
        format!(
            "LEFT JOIN LATERAL (SELECT COALESCE(t.position, 0) AS position, t.name, \
             lpad(COALESCE(t.position, 0)::text, 10, '0') || ':' || t.name || ':' || t.id::text \
             AS group_key \
             FROM entity_tags et JOIN tags t ON t.id = et.tag_id \
             WHERE et.entity_id = {table}.id AND et.entity_type = '{entity_type}' \
             AND et.valid_to IS NULL AND et.snapshot_id IS NULL \
             AND t.tag_group_id = '{group_id}' \
             ORDER BY t.position, t.name LIMIT 1) AS {alias} ON true"
        )
    }

    /// [`apply_ordering`], with this tag-group grouping or sort in place of `group_by` or
    /// `order_by` where set.
    pub fn apply<T, O>(
        &self,
        entity_type: EntityDiscriminants,
        group_by: Option<O>,
        order_by: Option<O>,
        direction: Option<OrderDirection>,
        filter: StorableFilter<T>,
        default_order: &str,
    ) -> (StorableFilter<T>, String)
    where
        T: Storable,
        O: OrderField,
    {
        if !self.is_set() {
            return apply_ordering(group_by, order_by, direction, filter, default_order);
        }
        // Fields this ordering replaces are dropped before the field ordering is built.
        let group_by = group_by.filter(|_| self.group_by.is_none());
        let order_by = order_by.filter(|_| self.order_by.is_none());
        let (mut filter, field_order) =
            apply_ordering(group_by, order_by, direction, filter, default_order);

        let mut parts = Vec::new();
        if let Some(group_id) = self.group_by {
            filter = filter.join(&Self::join::<T>("tag_group_grouped", entity_type, group_id));
            parts.push(format!("{} ASC", Self::GROUP_SQL));
        }
        // A field ordering ends with the row id; a tag-group sort goes in before it. With no
        // field left, `apply_ordering` returned the default order, which this replaces.
        let id_term = format!("{}.id ASC", T::table_name());
        if group_by.is_some() || order_by.is_some() {
            let head = field_order
                .strip_suffix(&format!(", {id_term}"))
                .unwrap_or(&field_order);
            parts.push(head.to_string());
        }
        if let Some(group_id) = self.order_by {
            filter = filter.join(&Self::join::<T>("tag_group_sorted", entity_type, group_id));
            let dir = direction.unwrap_or_default().to_sql();
            // An unordered group's tags share position 0, so the name decides between them.
            parts.push(format!(
                "tag_group_sorted.position {dir} NULLS LAST, tag_group_sorted.name {dir}"
            ));
        }
        parts.push(id_term);
        (filter, parts.join(", "))
    }
}

#[cfg(test)]
mod tag_group_ordering_tests {
    use super::*;
    use crate::server::hosts::{handlers::HostOrderField, r#impl::base::Host};

    fn order(ordering: TagGroupOrdering, group_by: Option<HostOrderField>) -> (String, String) {
        let (filter, order) = ordering.apply(
            EntityDiscriminants::Host,
            group_by,
            None::<HostOrderField>,
            Some(OrderDirection::Desc),
            StorableFilter::<Host>::new_from_site_ids(&[]),
            "hosts.created_at ASC",
        );
        (filter.to_join_clause(), order)
    }

    #[test]
    fn grouping_and_sorting_by_site_join_the_site_once_and_order_by_its_name() {
        let (filter, order) = apply_ordering(
            Some(HostOrderField::SiteId),
            Some(HostOrderField::SiteId),
            Some(OrderDirection::Desc),
            StorableFilter::<Host>::new_from_site_ids(&[]),
            "",
        );
        assert_eq!(filter.to_join_clause().matches("JOIN sites").count(), 1);
        assert!(order.starts_with("row_site.name ASC, hosts.site_id ASC, row_site.name DESC"));
    }

    #[test]
    fn unset_leaves_the_field_ordering_alone() {
        let (joins, order) = order(TagGroupOrdering::default(), None);
        assert!(joins.is_empty());
        assert_eq!(order, "hosts.created_at ASC");
    }

    #[test]
    fn grouping_by_a_tag_group_leads_and_the_row_id_still_breaks_ties() {
        let group = Uuid::new_v4();
        let (joins, order) = order(
            TagGroupOrdering {
                group_by: Some(group),
                order_by: Some(group),
            },
            Some(HostOrderField::default()),
        );
        assert!(joins.contains(&group.to_string()));
        assert!(joins.contains("valid_to IS NULL"));
        let terms: Vec<&str> = order.split(", ").collect();
        assert_eq!(terms.first(), Some(&"tag_group_grouped.group_key ASC"));
        assert_eq!(terms.last(), Some(&"hosts.id ASC"));
        assert!(terms.contains(&"tag_group_sorted.position DESC NULLS LAST"));
        assert!(terms.contains(&"tag_group_sorted.name DESC"));
        // The host field it replaced is gone.
        assert_eq!(terms.len(), 4);
    }
}
