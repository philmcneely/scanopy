use crate::server::daemon_api_keys::handlers::DaemonApiKeyFilterQuery;
use crate::server::shared::handlers::query::narrowing_check::assert_every_param_narrows;
use crate::tests::list_scope;
use uuid::Uuid;

/// Every filter the daemon API keys list documents narrows the rows.
#[tokio::test]
async fn every_documented_daemon_api_key_filter_narrows_the_rows() {
    let (site, other_site, org) = (Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4());
    let scope = list_scope(&[site, other_site], org);
    let id = Uuid::new_v4().to_string();
    let site_param = site.to_string();
    assert_every_param_narrows(
        |query: &DaemonApiKeyFilterQuery| query.narrow(&scope).expect("filter builds"),
        &[
            ("site_ids", site_param.as_str()),
            ("ids", id.as_str()),
            ("unbound", "true"),
            ("is_enabled", "true"),
            ("tag_ids", id.as_str()),
            ("tag_sets", id.as_str()),
            ("search", "lab"),
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
