//! Lists paged through `list_page`, against a database: each group's count covers every page,
//! filters narrow on the server, and no row outside the caller's sites or organization appears.

use chrono::Utc;
use uuid::Uuid;

use crate::server::shared::handlers::list::list_page;
use crate::server::shared::storage::traits::Storage;
use crate::server::shared::types::api::GroupCount;
use crate::server::vlans::handlers::{VlanFilterQuery, VlanOrderField};
use crate::server::vlans::r#impl::base::{Vlan, VlanBase};

use super::{list_scope, organization, site, subnet, test_state, user};
use crate::server::shared::storage::traits::Storable;

fn counts(groups: Option<&[GroupCount]>) -> Vec<(String, u64)> {
    groups
        .expect("a grouped list counts its groups")
        .iter()
        .map(|g| (g.value.clone().unwrap_or_default(), g.count))
        .collect()
}

fn vlan(site_id: Uuid, organization_id: Uuid, number: u16) -> Vlan {
    let now = Utc::now();
    Vlan {
        id: Uuid::new_v4(),
        created_at: now,
        updated_at: now,
        valid_from: now,
        last_seen_at: now,
        base: VlanBase {
            vlan_number: number,
            name: format!("vlan-{number}"),
            site_id,
            organization_id,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[tokio::test]
async fn a_grouped_vlan_page_counts_every_group_and_stays_in_the_callers_sites() {
    let (storage, state, _container) = test_state().await;

    let org = organization();
    storage.organizations.create(&org).await.unwrap();
    let (office, lab) = (site(&org.id), site(&org.id));
    storage.sites.create(&office).await.unwrap();
    storage.sites.create(&lab).await.unwrap();

    let other_org = organization();
    storage.organizations.create(&other_org).await.unwrap();
    let foreign = site(&other_org.id);
    storage.sites.create(&foreign).await.unwrap();

    for number in [10, 20, 30] {
        storage
            .vlans
            .create(&vlan(office.id, org.id, number))
            .await
            .unwrap();
    }
    let lab_vlan = vlan(lab.id, org.id, 40);
    storage.vlans.create(&lab_vlan).await.unwrap();
    storage
        .vlans
        .create(&vlan(foreign.id, other_org.id, 50))
        .await
        .unwrap();

    let sites = [office.id, lab.id];
    let grouped = VlanFilterQuery {
        group_by: Some(VlanOrderField::SiteId),
        order_by: Some(VlanOrderField::VlanNumber),
        limit: Some(2),
        offset: Some(1),
        ..Default::default()
    };
    let page = list_page::<Vlan>(&state, &list_scope(&sites, org.id), &grouped)
        .await
        .unwrap();

    let pagination = &page.meta.pagination;
    assert_eq!(
        pagination.total_count, 4,
        "the other org's VLAN is not counted"
    );
    let groups = counts(pagination.group_counts.as_deref());
    assert_eq!(
        groups.iter().map(|(_, count)| count).sum::<u64>(),
        pagination.total_count,
        "the groups cover every matching row, not the page"
    );
    let office_count = groups
        .iter()
        .find(|(key, _)| *key == office.id.to_string())
        .map(|(_, count)| *count);
    assert_eq!(
        office_count,
        Some(3),
        "a group cut by the page keeps its full size"
    );
    assert_eq!(page.data.len(), 2);

    // A site the caller can't see narrows to nothing rather than widening the list.
    let foreign_site = VlanFilterQuery {
        site_ids: Some(vec![foreign.id]),
        ..Default::default()
    };
    let page = list_page::<Vlan>(&state, &list_scope(&sites, org.id), &foreign_site)
        .await
        .unwrap();
    assert_eq!(page.meta.pagination.total_count, 0);

    // A subnet filter narrows through the link table.
    let lab_subnet = subnet(&lab.id);
    storage.subnets.create(&lab_subnet).await.unwrap();
    storage
        .subnet_vlan
        .link(&lab_subnet.id, &lab_vlan.id)
        .await
        .unwrap();
    let on_subnet = VlanFilterQuery {
        subnet_ids: Some(vec![lab_subnet.id]),
        ..Default::default()
    };
    let page = list_page::<Vlan>(&state, &list_scope(&sites, org.id), &on_subnet)
        .await
        .unwrap();
    let ids: Vec<Uuid> = page.data.iter().map(|v| v.id).collect();
    assert_eq!(ids, vec![lab_vlan.id]);
}

/// A user's API keys list, its CSV export and its filter options hold that user's keys only. The
/// export once read through the organization-wide default and returned every member's keys.
#[tokio::test]
async fn user_api_keys_list_only_the_callers_own() {
    use crate::server::shared::handlers::traits::{CrudHandlers, ListScope, field_values_filter};
    use crate::server::shared::services::traits::CrudService;
    use crate::server::user_api_keys::handlers::{UserApiKeyFilterQuery, UserApiKeyOrderField};
    use crate::server::user_api_keys::r#impl::base::{UserApiKey, UserApiKeyBase};
    use crate::server::users::r#impl::permissions::UserOrgPermissions;

    let (storage, state, _container) = test_state().await;
    let org = organization();
    storage.organizations.create(&org).await.unwrap();
    let net = site(&org.id);
    storage.sites.create(&net).await.unwrap();
    let (mut me, mut colleague) = (user(&org.id), user(&org.id));
    me.base.email = email_address::EmailAddress::new_unchecked("me@porto.example");
    colleague.base.email = email_address::EmailAddress::new_unchecked("colleague@porto.example");
    storage.users.create(&me).await.unwrap();
    storage.users.create(&colleague).await.unwrap();

    let key = |owner: Uuid, name: &str| {
        UserApiKey::new(UserApiKeyBase {
            name: name.to_string(),
            user_id: owner,
            organization_id: org.id,
            key: Uuid::new_v4().to_string(),
            is_enabled: true,
            ..Default::default()
        })
    };
    let mine = key(me.id, "ci");
    storage.user_api_keys.create(&mine).await.unwrap();
    storage
        .user_api_keys
        .create(&key(colleague.id, "backup"))
        .await
        .unwrap();

    let scope = ListScope {
        user_id: Some(me.id),
        permissions: Some(UserOrgPermissions::Admin),
        ..list_scope(&[net.id], org.id)
    };
    let query = UserApiKeyFilterQuery::default();

    let page = list_page::<UserApiKey>(&state, &scope, &query)
        .await
        .unwrap();
    let ids: Vec<Uuid> = page.data.iter().map(|k| k.id).collect();
    assert_eq!(ids, vec![mine.id]);

    let exported = state
        .services
        .user_api_key_service
        .get_all(
            UserApiKey::list_filter(&state, &query, &scope)
                .await
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        exported.iter().map(|k| k.id).collect::<Vec<_>>(),
        vec![mine.id]
    );

    let options = state
        .services
        .user_api_key_service
        .count_by_group(
            field_values_filter::<UserApiKey>(&scope, &query, UserApiKeyOrderField::Name),
            "user_api_keys.name",
        )
        .await
        .unwrap();
    assert_eq!(options.iter().map(|g| g.count).sum::<u64>(), 1);
}

/// Credentials sort, group and filter on what their type's metadata says, through the SQL built
/// from that metadata: integration, beta, and the targets a type can be applied to. Another
/// organization's credentials never appear.
#[tokio::test]
async fn credentials_group_and_filter_on_their_types_metadata() {
    use crate::server::credentials::handlers::{CredentialFilterQuery, CredentialOrderField};
    use crate::server::credentials::r#impl::base::{Credential, CredentialBase};
    use crate::server::credentials::r#impl::types::Target;
    use crate::server::credentials::r#impl::types::{
        CredentialIntegration, CredentialTypeDiscriminants,
    };

    let (storage, state, _container) = test_state().await;
    let org = organization();
    storage.organizations.create(&org).await.unwrap();
    let other_org = organization();
    storage.organizations.create(&other_org).await.unwrap();

    let credential = |organization_id: Uuid, t: CredentialTypeDiscriminants| {
        Credential::new(CredentialBase {
            name: format!("{t}"),
            organization_id,
            credential_type: t.to_credential_type(),
            ..Default::default()
        })
    };
    let snmp = credential(org.id, CredentialTypeDiscriminants::SnmpV2c);
    let gnmi = credential(org.id, CredentialTypeDiscriminants::Gnmi);
    let docker = credential(org.id, CredentialTypeDiscriminants::DockerSocket);
    for c in [&snmp, &gnmi, &docker] {
        storage.credentials.create(c).await.unwrap();
    }
    storage
        .credentials
        .create(&credential(
            other_org.id,
            CredentialTypeDiscriminants::SnmpV3,
        ))
        .await
        .unwrap();
    let scope = list_scope(&[], org.id);

    let grouped = CredentialFilterQuery {
        group_by: Some(CredentialOrderField::Integration),
        ..Default::default()
    };
    let page = list_page::<Credential>(&state, &scope, &grouped)
        .await
        .unwrap();
    let mut groups = counts(page.meta.pagination.group_counts.as_deref());
    groups.sort();
    assert_eq!(
        groups,
        vec![
            ("Docker".to_string(), 1),
            ("Gnmi".to_string(), 1),
            ("Snmp".to_string(), 1)
        ],
        "grouped by the integration id the UI groups on, and only this organization's"
    );

    let ids = |query: CredentialFilterQuery| {
        let state = &state;
        let scope = &scope;
        async move {
            let mut ids: Vec<Uuid> = list_page::<Credential>(state, scope, &query)
                .await
                .unwrap()
                .data
                .iter()
                .map(|c| c.id)
                .collect();
            ids.sort();
            ids
        }
    };
    let mut snmp_and_gnmi = vec![snmp.id, gnmi.id];
    snmp_and_gnmi.sort();

    assert_eq!(
        ids(CredentialFilterQuery {
            integrations: Some(vec![CredentialIntegration::Docker]),
            ..Default::default()
        })
        .await,
        vec![docker.id]
    );
    assert_eq!(
        ids(CredentialFilterQuery {
            beta: Some(vec![true]),
            ..Default::default()
        })
        .await,
        vec![gnmi.id]
    );
    // A socket reaches only the daemon's own host; SNMP and gNMI can broadcast to a site.
    assert_eq!(
        ids(CredentialFilterQuery {
            targets: Some(vec![Target::Site]),
            ..Default::default()
        })
        .await,
        snmp_and_gnmi
    );
}

/// Every filter the credentials list documents narrows the rows.
#[tokio::test]
async fn every_documented_credential_filter_narrows_the_rows() {
    use crate::server::credentials::handlers::CredentialFilterQuery;
    use crate::server::shared::handlers::query::narrowing_check::assert_every_param_narrows;

    let scope = list_scope(&[Uuid::new_v4()], Uuid::new_v4());
    let id = Uuid::new_v4().to_string();
    assert_every_param_narrows(
        |query: &CredentialFilterQuery| query.narrow(&scope).expect("filter builds"),
        &[
            ("type", "SnmpV2c"),
            ("integrations", "Docker"),
            ("beta", "true"),
            ("unofficial_api", "true"),
            ("targets", "Site"),
            ("daemon_os", "Windows"),
            ("site_ids", id.as_str()),
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

/// Daemons sort by version in version order and group and filter by the status the server
/// derives, in memory, with totals and group counts that cover every row rather than the page.
#[tokio::test]
async fn daemons_order_and_filter_on_what_code_derives() {
    use crate::server::daemons::handlers::{DaemonFilterQuery, DaemonOrderField};
    use crate::server::daemons::r#impl::api::DaemonHealth;
    use crate::server::daemons::r#impl::base::Daemon;
    use crate::server::shared::handlers::query::OrderDirection;

    let (storage, state, _container) = test_state().await;
    let org = organization();
    storage.organizations.create(&org).await.unwrap();
    let net = site(&org.id);
    storage.sites.create(&net).await.unwrap();

    let maintainer = user(&org.id);
    storage.users.create(&maintainer).await.unwrap();
    let current = env!("CARGO_PKG_VERSION");
    for (name, version, unreachable) in [
        ("ten", "0.17.10", false),
        ("nine", "0.17.9", false),
        ("current", current, false),
        ("lost", current, true),
    ] {
        let daemon_host = super::host(&net.id);
        storage.hosts.create(&daemon_host).await.unwrap();
        let mut d = super::daemon(&net.id, &daemon_host.id);
        d.base.user_id = maintainer.id;
        d.base.name = name.to_string();
        d.base.version = Some(version.parse().unwrap());
        d.base.is_unreachable = unreachable;
        storage.daemons.create(&d).await.unwrap();
    }
    let scope = list_scope(&[net.id], org.id);
    let names = |page: &crate::server::shared::types::api::PaginatedApiResponse<
        crate::server::daemons::r#impl::api::DaemonResponse,
    >| {
        page.data
            .iter()
            .map(|d| d.base.name.clone())
            .collect::<Vec<_>>()
    };

    // Whole order: nine, ten, current|lost by name. The page skips one, takes two.
    let by_version = DaemonFilterQuery {
        order_by: Some(DaemonOrderField::Version),
        order_direction: Some(OrderDirection::Asc),
        limit: Some(2),
        offset: Some(1),
        ..Default::default()
    };
    let page = list_page::<Daemon>(&state, &scope, &by_version)
        .await
        .unwrap();
    assert_eq!(page.meta.pagination.total_count, 4);
    assert_eq!(names(&page)[0], "ten", "0.17.10 follows 0.17.9");

    let unreachable = DaemonFilterQuery {
        statuses: Some(vec![DaemonHealth::Unreachable]),
        group_by: Some(DaemonOrderField::Status),
        ..Default::default()
    };
    let page = list_page::<Daemon>(&state, &scope, &unreachable)
        .await
        .unwrap();
    assert_eq!(names(&page), vec!["lost"]);
    assert_eq!(page.meta.pagination.total_count, 1);
    assert_eq!(
        counts(page.meta.pagination.group_counts.as_deref()),
        vec![("unreachable".to_string(), 1)]
    );
}

/// Every filter the daemons list documents narrows the rows. `statuses` is derived in code, so
/// it narrows in the list rather than in this filter.
#[tokio::test]
async fn every_documented_daemon_filter_narrows_the_rows() {
    use crate::server::daemons::handlers::DaemonFilterQuery;
    use crate::server::shared::handlers::query::narrowing_check::assert_every_param_narrows;

    let site = Uuid::new_v4();
    let scope = list_scope(&[site, Uuid::new_v4()], Uuid::new_v4());
    let id = Uuid::new_v4().to_string();
    let site_param = site.to_string();
    assert_every_param_narrows(
        |query: &DaemonFilterQuery| query.narrow(&scope).expect("filter builds"),
        &[
            ("site_ids", site_param.as_str()),
            ("os", "linux"),
            ("modes", "server_poll"),
            ("maintainer_ids", id.as_str()),
            ("versions", "0.17.9"),
            ("interfaced_subnet_ids", id.as_str()),
            ("tag_ids", id.as_str()),
            ("tag_sets", id.as_str()),
            ("search", "lab"),
        ],
        &[
            "statuses",
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
