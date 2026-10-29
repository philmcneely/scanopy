//! Subnet-specific filters.
use super::*;

use crate::server::shared::types::metadata::HasId;
use crate::server::subnets::r#impl::types::SubnetType;

/// Generic over the row type so the shared query extractor can apply it; it reads subnet columns.
impl<T: Storable> StorableFilter<T> {
    /// SQL form of [`Subnet::is_user_managed`](crate::server::subnets::r#impl::base::Subnet::is_user_managed) — the subnets that belong to the
    /// inventory the user curates, which is what the management lists show and
    /// what the dashboard's subnet count counts.
    ///
    /// Keyed on provenance as well as category, so a subnet the user created is
    /// counted whatever category they gave it. Counting the fabricated rows while
    /// the lists omitted them is the discrepancy reported in GH #677.
    ///
    /// This mirrors [`Subnet::is_user_managed`](crate::server::subnets::r#impl::base::Subnet::is_user_managed) — the two must be changed
    /// together, the same convention [`Self::stale_by_site`] follows with
    /// [`DiscoveryTracked::freshness`](crate::server::shared::storage::snapshot::DiscoveryTracked::freshness).
    /// The category list itself is not restated here: it is bound from
    /// [`SubnetType::synthetic_categories`].
    pub fn user_managed(mut self) -> Self {
        let condition = self.user_managed_condition();
        self.conditions.push(condition);
        self
    }

    /// [`Self::user_managed`], plus `shared_site`'s seeded Internet and Remote ranges
    /// ([`SubnetType::seeded_per_site`]). Every site holds its own copy of each, so the list shows
    /// one site's copy, once, instead of one per site. With no site, the user-managed rows alone.
    pub fn user_managed_with_shared(mut self, shared_site: Option<Uuid>) -> Self {
        let Some(site_id) = shared_site else {
            return self.user_managed();
        };
        let user_managed = self.user_managed_condition();
        let site_col = self.qualify_column("site_id");
        let source_col = self.qualify_column("source");
        let type_col = self.qualify_column("subnet_type");

        self.values.push(SqlValue::Uuid(site_id));
        let site_idx = self.values.len();
        let placeholders = self.bind_types(SubnetType::seeded_per_site());

        self.conditions.push(format!(
            "({user_managed} OR ({site_col} = ${site_idx} AND {source_col}->>'type' = 'System' \
             AND {type_col} IN ({placeholders})))"
        ));
        self
    }

    fn user_managed_condition(&mut self) -> String {
        let source_col = self.qualify_column("source");
        let type_col = self.qualify_column("subnet_type");
        let placeholders = self.bind_types(SubnetType::synthetic_categories());
        format!("({source_col}->>'type' = 'Manual' OR {type_col} NOT IN ({placeholders}))")
    }

    /// Binds each type's id and returns the placeholders, comma-separated.
    fn bind_types(&mut self, types: &[SubnetType]) -> String {
        types
            .iter()
            .map(|subnet_type| {
                self.values
                    .push(SqlValue::String(subnet_type.id().to_string()));
                format!("${}", self.values.len())
            })
            .collect::<Vec<_>>()
            .join(", ")
    }
}
