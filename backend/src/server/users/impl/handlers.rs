use crate::server::{
    config::AppState,
    shared::{
        handlers::{
            ordering::{NoOrderField, in_sql},
            query::NoFilterQuery,
            traits::{CrudHandlers, ListScope},
        },
        storage::filter::StorableFilter,
    },
    users::{
        r#impl::{base::User, permissions::UserOrgPermissions},
        service::UserService,
    },
};
use strum::IntoEnumIterator;

impl CrudHandlers for User {
    type Service = UserService;
    type FilterQuery = NoFilterQuery;
    type OrderField = NoOrderField;

    fn get_service(state: &AppState) -> &Self::Service {
        &state.services.user_service
    }

    /// The users a caller may list, as [`User::is_listed_for`] decides: every user for an owner,
    /// otherwise the caller and the users below their role. The CSV export reads through this, so
    /// it can't return more of the organization than the users list shows.
    fn access_filter(scope: &ListScope) -> StorableFilter<Self> {
        let org = StorableFilter::new_from_org_id(&scope.organization_id);
        let (Some(viewer), Some(permissions)) = (scope.user_id, scope.permissions) else {
            return StorableFilter::new_from_entity_ids(&[]);
        };
        if permissions == UserOrgPermissions::Owner {
            return org;
        }
        let below: Vec<&str> = UserOrgPermissions::iter()
            .filter(|role| *role < permissions)
            .map(|role| role.as_str())
            .collect();
        // The viewer's id is a parsed `Uuid` from their session, and the roles are written in
        // code, so both are inlined.
        let listed = format!(
            "users.id = '{viewer}' OR {}",
            in_sql("users.permissions", below)
        );
        org.expr_text_in(&listed, &["true".to_string()])
    }
}
