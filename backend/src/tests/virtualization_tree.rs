//! The host list grouped by virtualization tree, through the generic tree path: each tree parent
//! leads its subtree, siblings follow the sort, and each group's count covers every page. The UI
//! matches a row to its group by the root each response carries against the `group_counts` key,
//! so the two must agree on every host.

use crate::server::hosts::handlers::{HostFilterQuery, HostOrderField};
use crate::server::hosts::r#impl::name::{HostName, HostNameSources};
use crate::server::shared::handlers::list::list_page;
use crate::server::shared::handlers::query::OrderDirection;
use crate::server::shared::storage::traits::Storage;
use crate::server::shared::types::api::GroupCount;

use super::{host, list_scope, organization, service, site, test_state};

fn counts(groups: &[GroupCount]) -> Vec<(String, u64)> {
    groups
        .iter()
        .map(|g| (g.value.clone().unwrap_or_default(), g.count))
        .collect()
}

#[tokio::test]
async fn a_tree_grouping_places_every_host_under_its_root() {
    let (storage, state, _container) = test_state().await;

    let org = organization();
    storage.organizations.create(&org).await.unwrap();
    let net = site(&org.id);
    storage.sites.create(&net).await.unwrap();

    let named = |name: &str| {
        let mut h = host(&net.id);
        h.base.name = HostName::manual(name.to_string());
        h
    };

    let node = named("pve-01");
    storage.hosts.create(&node).await.unwrap();
    let node_runtime = service(&net.id, &node.id);
    storage.services.create(&node_runtime).await.unwrap();

    let mut vm = named("docker-vm");
    vm.base.virtualization_service_id = Some(node_runtime.id);
    storage.hosts.create(&vm).await.unwrap();
    let vm_runtime = service(&net.id, &vm.id);
    storage.services.create(&vm_runtime).await.unwrap();

    let mut container = named("pihole");
    container.base.virtualization_service_id = Some(vm_runtime.id);
    storage.hosts.create(&container).await.unwrap();

    let mut sibling = named("app-vm");
    sibling.base.virtualization_service_id = Some(node_runtime.id);
    storage.hosts.create(&sibling).await.unwrap();

    let lone = named("printer");
    storage.hosts.create(&lone).await.unwrap();

    let query = HostFilterQuery {
        group_by: Some(HostOrderField::VirtualizedBy),
        include_children: Some(false),
        ..Default::default()
    };
    let page = list_page::<crate::server::hosts::r#impl::base::Host>(
        &state,
        &list_scope(&[net.id], org.id),
        &query,
    )
    .await
    .unwrap();

    let names: Vec<&str> = page.data.iter().map(|h| h.name.as_str()).collect();
    assert_eq!(
        names,
        vec!["printer", "pve-01", "app-vm", "docker-vm", "pihole"],
        "no tree first, then the tree parent first with each subtree under its parent, siblings by name"
    );

    let depths: Vec<u32> = page.data.iter().map(|h| h.virtualization_depth).collect();
    assert_eq!(depths, vec![0, 0, 1, 1, 2]);

    // The groups the responses' roots imply, in page order, against `group_counts`.
    let mut response_groups: Vec<(String, u64)> = Vec::new();
    for h in &page.data {
        let key = h
            .virtualization_root_host_id
            .map(|id| id.to_string())
            .unwrap_or_default();
        match response_groups.last_mut() {
            Some((last, count)) if *last == key => *count += 1,
            _ => response_groups.push((key, 1)),
        }
    }
    let group_counts = counts(
        page.meta
            .pagination
            .group_counts
            .as_deref()
            .expect("a grouped list counts its groups"),
    );
    assert_eq!(response_groups, group_counts);
    assert_eq!(
        group_counts,
        vec![(String::new(), 1), (node.id.to_string(), 4)]
    );
}

/// Sorted inside a tree grouping, each parent still leads its subtree and every set of siblings
/// follows the sort. A page cut through a tree keeps every group's full count.
#[tokio::test]
async fn a_sort_orders_siblings_inside_each_tree() {
    let (storage, state, _container) = test_state().await;

    let org = organization();
    storage.organizations.create(&org).await.unwrap();
    let net = site(&org.id);
    storage.sites.create(&net).await.unwrap();

    let named = |name: &str| {
        let mut h = host(&net.id);
        h.base.name = HostName::manual(name.to_string());
        h
    };

    let node = named("pve-01");
    storage.hosts.create(&node).await.unwrap();
    let node_runtime = service(&net.id, &node.id);
    storage.services.create(&node_runtime).await.unwrap();
    for guest in ["app-vm", "docker-vm"] {
        let mut vm = named(guest);
        vm.base.virtualization_service_id = Some(node_runtime.id);
        storage.hosts.create(&vm).await.unwrap();
    }
    for lone in ["printer", "camera"] {
        storage.hosts.create(&named(lone)).await.unwrap();
    }

    let query = HostFilterQuery {
        group_by: Some(HostOrderField::VirtualizedBy),
        order_by: Some(HostOrderField::Name),
        order_direction: Some(OrderDirection::Desc),
        limit: Some(3),
        offset: Some(1),
        include_children: Some(false),
        ..Default::default()
    };
    let page = list_page::<crate::server::hosts::r#impl::base::Host>(
        &state,
        &list_scope(&[net.id], org.id),
        &query,
    )
    .await
    .unwrap();

    assert_eq!(page.meta.pagination.total_count, 5);
    let names: Vec<&str> = page.data.iter().map(|h| h.name.as_str()).collect();
    // Whole order: printer, camera | pve-01, docker-vm, app-vm. The page skips one, takes three.
    assert_eq!(names, vec!["camera", "pve-01", "docker-vm"]);
    assert_eq!(
        counts(page.meta.pagination.group_counts.as_deref().unwrap()),
        vec![(String::new(), 2), (node.id.to_string(), 3)]
    );
}
