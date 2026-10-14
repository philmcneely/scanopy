use crate::server::shared::handlers::query::narrowing_check::assert_every_param_narrows;
use crate::server::shared::handlers::traits::ListScope;
use crate::server::user_api_keys::handlers::UserApiKeyFilterQuery;
use crate::tests::list_scope;
use uuid::Uuid;

/// Every filter the user API keys list documents narrows the rows.
#[tokio::test]
async fn every_documented_user_api_key_filter_narrows_the_rows() {
    let (site, org, me) = (Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4());
    let scope = ListScope {
        user_id: Some(me),
        ..list_scope(&[site], org)
    };
    let id = Uuid::new_v4().to_string();
    assert_every_param_narrows(
        |query: &UserApiKeyFilterQuery| query.narrow(&scope).expect("filter builds"),
        &[
            ("permissions", "Admin"),
            ("site_ids", id.as_str()),
            ("is_enabled", "true"),
            ("tag_ids", id.as_str()),
            ("tag_sets", id.as_str()),
            ("search", "ci"),
        ],
        &[
            "limit",
            "offset",
            "group_by",
            "order_by",
            "order_direction",
            "group_by_tag_group",
            "order_by_tag_group",
        ],
    )
    .await;
}
