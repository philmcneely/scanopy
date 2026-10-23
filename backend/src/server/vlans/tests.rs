use crate::server::shared::handlers::query::narrowing_check::assert_every_param_narrows;
use crate::server::vlans::handlers::VlanFilterQuery;
use crate::tests::list_scope;
use uuid::Uuid;

/// Every filter the VLANs list documents narrows the rows, through the one filter the list and
/// the CSV export build.
#[tokio::test]
async fn every_documented_vlan_filter_narrows_the_rows() {
    let (site, other_site, org) = (Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4());
    let id = Uuid::new_v4().to_string();
    let site_param = site.to_string();
    assert_every_param_narrows(
        |query: &VlanFilterQuery| {
            query
                .narrow(&list_scope(&[site, other_site], org))
                .expect("filter builds")
        },
        &[
            ("site_ids", site_param.as_str()),
            ("sources", "Discovery"),
            ("subnet_ids", id.as_str()),
            ("tag_ids", id.as_str()),
            ("tag_sets", id.as_str()),
            ("search", "mgmt"),
            ("at", "2026-01-01T00:00:00Z"),
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
