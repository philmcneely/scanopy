//! Where hosts sit in their virtualization trees, for the host list's tree grouping and the
//! placement every host response carries.
//!
//! A host's parent is the host running the service that virtualizes it
//! (`hosts.virtualization_service_id` → `services.host_id`). Chains nest: a Proxmox node runs a
//! VM, which runs a Docker engine, which runs a macvlan container with an address of its own. The
//! walk follows live services and live hosts only.
use super::*;
use crate::server::shared::handlers::tree::{DEPTH_CAP, TreePosition, tree_position};
use crate::server::shared::storage::traits::Storage;

impl HostService {
    /// Fill each response's `virtualization_parent_host_id`, `virtualization_root_host_id` and
    /// `virtualization_depth`.
    pub(crate) async fn place_in_virtualization_trees(
        &self,
        responses: &mut [HostResponse],
    ) -> Result<()> {
        let positions = self
            .virtualization_tree_positions(
                responses
                    .iter()
                    .map(|r| (r.id, r.virtualization_service_id))
                    .collect(),
            )
            .await?;
        for response in responses.iter_mut() {
            let position = positions.get(&response.id);
            response.virtualization_parent_host_id = position.and_then(|p| p.parent_id);
            response.virtualization_root_host_id = position.map(|p| p.root_id);
            response.virtualization_depth = position.map(|p| p.depth).unwrap_or(0);
        }
        Ok(())
    }

    /// Where each of `host_ids` sits in its virtualization tree, for a list grouped by it. Reads
    /// only each host's virtualizing service, not the whole row.
    pub(crate) async fn tree_positions_of(
        &self,
        host_ids: &[Uuid],
    ) -> Result<HashMap<Uuid, TreePosition>> {
        let runs_under = self
            .storage()
            .get_ordered_values(
                StorableFilter::<Host>::new_from_entity_ids(host_ids),
                "hosts.id ASC",
                &["hosts.virtualization_service_id"],
            )
            .await?
            .into_iter()
            .map(|(id, values)| {
                let service = values
                    .into_iter()
                    .next()
                    .flatten()
                    .and_then(|v| v.parse().ok());
                (id, service)
            })
            .collect();
        self.virtualization_tree_positions(runs_under).await
    }

    /// Where each host sits, given each host and the service it runs under. A host in no tree has
    /// no entry.
    ///
    /// One services and one hosts query per level of ancestors (at most [`DEPTH_CAP`]), and one
    /// grouped count for whether a host with no parent runs anything.
    async fn virtualization_tree_positions(
        &self,
        hosts: Vec<(Uuid, Option<Uuid>)>,
    ) -> Result<HashMap<Uuid, TreePosition>> {
        if hosts.is_empty() {
            return Ok(HashMap::new());
        }
        let listed: Vec<Uuid> = hosts.iter().map(|(id, _)| *id).collect();

        // Every host met so far, with the service it runs under.
        let mut runs_under: HashMap<Uuid, Option<Uuid>> = hosts.into_iter().collect();
        // Resolved parent links: host → the live host it runs under.
        let mut parent_of: HashMap<Uuid, Uuid> = HashMap::new();
        // Which host each live service runs on, for every service looked up.
        let mut service_host: HashMap<Uuid, Uuid> = HashMap::new();

        let mut frontier: Vec<Uuid> = listed.clone();
        for _ in 0..DEPTH_CAP {
            let service_ids: Vec<Uuid> = frontier
                .iter()
                .filter_map(|id| runs_under.get(id).copied().flatten())
                .filter(|service_id| !service_host.contains_key(service_id))
                .collect::<HashSet<_>>()
                .into_iter()
                .collect();
            if !service_ids.is_empty() {
                let services = self
                    .service_service
                    .get_all(StorableFilter::<Service>::new_from_entity_ids(&service_ids).live())
                    .await?;
                service_host.extend(services.into_iter().map(|s| (s.id, s.base.host_id)));
            }

            let candidates: Vec<(Uuid, Uuid)> = frontier
                .iter()
                .filter_map(|id| {
                    let service_id = runs_under.get(id).copied().flatten()?;
                    let parent = *service_host.get(&service_id)?;
                    (parent != *id).then_some((*id, parent))
                })
                .collect();

            let unknown: Vec<Uuid> = candidates
                .iter()
                .map(|(_, parent)| *parent)
                .filter(|parent| !runs_under.contains_key(parent))
                .collect::<HashSet<_>>()
                .into_iter()
                .collect();
            let mut fetched = Vec::new();
            if !unknown.is_empty() {
                for parent in self
                    .get_all(StorableFilter::<Host>::new_from_entity_ids(&unknown).live())
                    .await?
                {
                    runs_under.insert(parent.id, parent.base.virtualization_service_id);
                    fetched.push(parent.id);
                }
            }

            // A parent only counts when it is a live host: one that is in `runs_under` was either
            // on the page (live list) or just fetched live.
            for (child, parent) in candidates {
                if runs_under.contains_key(&parent) {
                    parent_of.insert(child, parent);
                }
            }

            if fetched.is_empty() {
                break;
            }
            frontier = fetched;
        }

        let roots_to_check: Vec<Uuid> = listed
            .iter()
            .filter(|id| !parent_of.contains_key(id))
            .copied()
            .collect();
        let runs_others = self
            .hosts_running_other_hosts(&roots_to_check, &runs_under)
            .await?;

        Ok(listed
            .into_iter()
            .filter_map(|id| {
                tree_position(
                    id,
                    |id| parent_of.get(&id).copied(),
                    runs_others.contains(&id),
                )
                .map(|position| (id, position))
            })
            .collect())
    }

    /// Which of `host_ids` have another live host running under one of their live services.
    ///
    /// `runs_under` holds each of `host_ids`' own virtualizing service, so a host recorded as
    /// running under its own service is not counted as its own guest.
    async fn hosts_running_other_hosts(
        &self,
        host_ids: &[Uuid],
        runs_under: &HashMap<Uuid, Option<Uuid>>,
    ) -> Result<HashSet<Uuid>> {
        if host_ids.is_empty() {
            return Ok(HashSet::new());
        }

        let services = self
            .service_service
            .get_all(StorableFilter::<Service>::new_from_host_ids(host_ids).live())
            .await?;
        if services.is_empty() {
            return Ok(HashSet::new());
        }
        let host_of: HashMap<Uuid, Uuid> =
            services.iter().map(|s| (s.id, s.base.host_id)).collect();
        let service_ids: Vec<Uuid> = host_of.keys().copied().collect();

        // Scoped by `service_ids`, which are services on hosts the caller already holds. Not
        // narrowed by site: a guest makes its host a root wherever the guest sits.
        let guests_per_service = self
            .count_by_group(
                StorableFilter::<Host>::new()
                    .virtualization_service_in(&service_ids, false)
                    .live(),
                "hosts.virtualization_service_id::text",
            )
            .await?;

        let mut running = HashSet::new();
        for group in guests_per_service {
            let Some(service_id) = group.value.and_then(|v| v.parse::<Uuid>().ok()) else {
                continue;
            };
            let Some(host) = host_of.get(&service_id) else {
                continue;
            };
            let self_guest = runs_under.get(host).copied().flatten() == Some(service_id);
            if group.count > u64::from(self_guest) {
                running.insert(*host);
            }
        }
        Ok(running)
    }
}
