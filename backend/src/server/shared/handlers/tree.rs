//! Lists whose rows nest: where a row sits in its tree, and the order a tree grouping lists rows
//! in.
//!
//! An entity names each row's parent (a host runs under the host whose service virtualizes it, a
//! subnet sits inside the most specific range containing it) through
//! [`PaginatedList::tree_positions`](super::list::PaginatedList::tree_positions); everything else
//! here is shared. The rule every tree follows:
//!
//! - walk up from a row through its parents, at most [`DEPTH_CAP`] steps, never revisiting a row
//!   (a cycle stops the walk where it would repeat);
//! - the root is the last row the walk reached;
//! - a row with no parent is a root only when some other row sits under it. A row that neither
//!   sits under nor holds anything is in no tree at all.

use uuid::Uuid;

/// The most ancestors a walk follows. Real chains are three deep; the cap is what keeps a cycle
/// the data should never hold from walking forever.
pub const DEPTH_CAP: usize = 8;

/// A row's place in its tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TreePosition {
    /// The row this one sits under, if any.
    pub parent_id: Option<Uuid>,
    /// The row at the top of this row's chain. Itself when it is a root.
    pub root_id: Uuid,
    /// How many ancestors sit above it. A root is 0.
    pub depth: u32,
}

impl TreePosition {
    /// The key a tree grouping files this row under: its root's id, `''` for a row in no tree.
    pub fn group_key(position: Option<&Self>) -> String {
        position.map(|p| p.root_id.to_string()).unwrap_or_default()
    }
}

/// Where `id` sits, or `None` when it is in no tree.
///
/// `parent_of` answers the row a row sits under, never the row itself. `has_children` says
/// whether another row sits under `id`.
pub fn tree_position(
    id: Uuid,
    parent_of: impl Fn(Uuid) -> Option<Uuid>,
    has_children: bool,
) -> Option<TreePosition> {
    let mut chain = vec![id];
    while chain.len() <= DEPTH_CAP {
        let current = *chain.last().expect("the chain starts with the row");
        match parent_of(current) {
            Some(parent) if !chain.contains(&parent) => chain.push(parent),
            _ => break,
        }
    }

    let depth = chain.len() - 1;
    if depth == 0 && !has_children {
        return None;
    }

    Some(TreePosition {
        parent_id: chain.get(1).copied(),
        root_id: *chain.last().expect("the chain starts with the row"),
        depth: depth as u32,
    })
}

/// A list grouped by tree, in display order: rows in no tree first, then each tree where its first
/// row falls in the sort; each parent first with its subtree directly under it, and every set of
/// siblings in the order `rows` arrive in, which is the sort. A row whose parent is not among
/// `rows` (filtered out) is placed as a root of its group.
///
/// `rows` is every matching row in sort order: its id, its group key and its parent.
pub fn tree_order(rows: &[(Uuid, String, Option<Uuid>)]) -> Vec<Uuid> {
    let present: std::collections::HashSet<Uuid> = rows.iter().map(|(id, _, _)| *id).collect();
    let mut children: std::collections::HashMap<Uuid, Vec<Uuid>> = Default::default();
    // Groups in the order their first row arrives, the no-tree group (`""`) ahead of every tree.
    let mut groups: Vec<&str> = vec![""];
    let mut listed: std::collections::HashSet<&str> = std::collections::HashSet::from([""]);
    let mut roots: std::collections::HashMap<&str, Vec<Uuid>> = Default::default();
    for (id, group, parent) in rows {
        if listed.insert(group.as_str()) {
            groups.push(group.as_str());
        }
        match parent.filter(|p| present.contains(p) && p != id) {
            Some(parent) => children.entry(parent).or_default().push(*id),
            None => roots.entry(group.as_str()).or_default().push(*id),
        }
    }

    let mut ordered = Vec::with_capacity(rows.len());
    let mut seen = std::collections::HashSet::new();
    for group_roots in groups.iter().filter_map(|group| roots.get(group)) {
        let mut stack: Vec<Uuid> = group_roots.iter().rev().copied().collect();
        while let Some(id) = stack.pop() {
            // A cycle the walk should never meet; stop rather than repeat a row.
            if !seen.insert(id) {
                continue;
            }
            ordered.push(id);
            if let Some(kids) = children.get(&id) {
                stack.extend(kids.iter().rev());
            }
        }
    }
    ordered
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn ids<const N: usize>() -> [Uuid; N] {
        std::array::from_fn(|_| Uuid::new_v4())
    }

    fn parents(edges: &[(Uuid, Uuid)]) -> impl Fn(Uuid) -> Option<Uuid> + '_ {
        let map: HashMap<Uuid, Uuid> = edges.iter().copied().collect();
        move |id| map.get(&id).copied()
    }

    #[test]
    fn a_three_level_chain_roots_every_host_at_the_top() {
        let [node, vm, container] = ids();
        let edges = [(vm, node), (container, vm)];

        assert_eq!(
            tree_position(container, parents(&edges), false),
            Some(TreePosition {
                parent_id: Some(vm),
                root_id: node,
                depth: 2
            })
        );
        assert_eq!(
            tree_position(vm, parents(&edges), true),
            Some(TreePosition {
                parent_id: Some(node),
                root_id: node,
                depth: 1
            })
        );
    }

    #[test]
    fn a_host_running_others_with_no_parent_is_its_own_root() {
        let [node] = ids();
        assert_eq!(
            tree_position(node, |_| None, true),
            Some(TreePosition {
                parent_id: None,
                root_id: node,
                depth: 0
            })
        );
    }

    #[test]
    fn a_host_that_neither_runs_under_nor_runs_anything_is_in_no_tree() {
        let [lone] = ids();
        assert_eq!(tree_position(lone, |_| None, false), None);
    }

    #[test]
    fn a_cycle_stops_where_it_would_repeat() {
        let [a, b] = ids();
        let edges = [(a, b), (b, a)];

        let position = tree_position(a, parents(&edges), true).expect("a has a parent");
        assert_eq!(position.root_id, b);
        assert_eq!(position.depth, 1);
    }

    #[test]
    fn a_chain_longer_than_the_cap_stops_at_the_cap() {
        let chain: Vec<Uuid> = (0..DEPTH_CAP + 4).map(|_| Uuid::new_v4()).collect();
        let edges: Vec<(Uuid, Uuid)> = chain.windows(2).map(|w| (w[0], w[1])).collect();

        let position = tree_position(chain[0], parents(&edges), false).expect("has parents");
        assert_eq!(position.depth as usize, DEPTH_CAP);
        assert_eq!(position.root_id, chain[DEPTH_CAP]);
    }

    #[test]
    fn siblings_follow_the_sort_and_parents_lead_their_subtrees() {
        let [node, vm_b, vm_a, container, lone_z, lone_y] = ids();
        let tree = node.to_string();
        // Sort order: lone_z, container, vm_b, lone_y, node, vm_a.
        let rows = vec![
            (lone_z, String::new(), None),
            (container, tree.clone(), Some(vm_a)),
            (vm_b, tree.clone(), Some(node)),
            (lone_y, String::new(), None),
            (node, tree.clone(), None),
            (vm_a, tree.clone(), Some(node)),
        ];
        assert_eq!(
            tree_order(&rows),
            vec![lone_z, lone_y, node, vm_b, vm_a, container],
            "hosts in no tree group first and keep the sort; inside the tree each parent leads and siblings keep the sort"
        );
    }

    #[test]
    fn a_row_whose_parent_is_filtered_out_heads_its_group() {
        let [node, vm, container] = ids();
        let tree = node.to_string();
        // `vm` is filtered out: `container` stands in its tree's group on its own.
        let rows = vec![(container, tree.clone(), Some(vm)), (node, tree, None)];
        assert_eq!(tree_order(&rows), vec![container, node]);
    }

    #[test]
    fn trees_list_where_their_rows_fall_in_the_sort_not_by_root_id() {
        // Root ids chosen so their text order is the reverse of the sort.
        let late = Uuid::from_u128(1);
        let early = Uuid::from_u128(2);
        let [early_kid, late_kid, lone] = ids();
        let rows = vec![
            (early, early.to_string(), None),
            (early_kid, early.to_string(), Some(early)),
            (late, late.to_string(), None),
            (late_kid, late.to_string(), Some(late)),
            (lone, String::new(), None),
        ];
        assert_eq!(
            tree_order(&rows),
            vec![lone, early, early_kid, late, late_kid],
            "rows in no tree lead; each tree follows where its first row sorts"
        );
    }
}
