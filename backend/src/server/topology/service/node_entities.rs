//! The entities each node stands for beyond its own `entity_id`, for the topology search.
//!
//! A search match is an entity; the map draws nodes. Most entities are a node's own `entity_id`,
//! but some are drawn inside another node or not drawn at all in a view: an address is a fact of a
//! port it is reached through in L2, a service bound to all of a host's addresses sits on each of
//! its IP cards, a guest's member services are rows on its card. Computed once, generically by node
//! shape rather than per view, so the frontend selects nodes by id and decides nothing itself.

use std::collections::{HashMap, HashSet};

use uuid::Uuid;

use crate::server::{
    interface_neighbors::r#impl::base::Neighbor,
    topology::{
        service::fdb,
        types::{
            api::TopologyData,
            nodes::{ContainerType, ElementEntityType, Node, NodeType},
        },
    },
};

/// For every node with associations, the entity ids it also stands for.
pub fn node_entities(nodes: &[Node], data: &TopologyData) -> HashMap<Uuid, Vec<Uuid>> {
    let mut ips_of_host: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
    for ip in &data.ip_addresses {
        ips_of_host.entry(ip.base.host_id).or_default().push(ip.id);
    }
    let host_of_interface: HashMap<Uuid, Uuid> = data
        .interfaces
        .iter()
        .map(|i| (i.interface.id, i.interface.base.host_id))
        .collect();
    let mut neighbour_hosts: HashMap<Uuid, HashSet<Uuid>> = HashMap::new();
    for row in &data.neighbours {
        let host = match row.neighbor {
            Neighbor::Host(host_id) => Some(host_id),
            Neighbor::Interface(id) => host_of_interface.get(&id).copied(),
        };
        if let Some(host) = host {
            neighbour_hosts
                .entry(row.interface_id)
                .or_default()
                .insert(host);
        }
    }
    let learned = fdb::learned_hosts(
        data.interfaces.iter().map(|i| &i.interface),
        &data.ip_addresses,
    );
    // Services bound to every address of their host: a port binding with no address.
    let mut all_address_services: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
    let host_of_service: HashMap<Uuid, Uuid> = data
        .services
        .iter()
        .map(|s| (s.service.id, s.service.base.host_id))
        .collect();
    for binding in &data.bindings {
        if binding.ip_address_id().is_none()
            && let Some(host) = host_of_service.get(&binding.base.service_id)
        {
            let services = all_address_services.entry(*host).or_default();
            if !services.contains(&binding.base.service_id) {
                services.push(binding.base.service_id);
            }
        }
    }

    let mut out = HashMap::new();
    for node in nodes {
        let mut related: Vec<Uuid> = Vec::new();
        match &node.node_type {
            NodeType::Element {
                host_id,
                element,
                inline_groups,
                ..
            } => {
                match element {
                    // A port stands for the devices reached through it: those its forwarding
                    // table learned and those its neighbour rows name, with their addresses.
                    ElementEntityType::Interface { interface_id } => {
                        for host in learned.get(interface_id).into_iter().flatten() {
                            related.push(host.host_id);
                            related.extend(&host.ip_address_ids);
                        }
                        for host in neighbour_hosts.get(interface_id).into_iter().flatten() {
                            related.push(*host);
                            related.extend(ips_of_host.get(host).into_iter().flatten());
                        }
                    }
                    ElementEntityType::IPAddress { .. } => {
                        related.extend(all_address_services.get(host_id).into_iter().flatten());
                    }
                    ElementEntityType::Service {} => related.push(*host_id),
                    ElementEntityType::Host {} => {}
                }
                related.extend(inline_groups.iter().map(|g| g.entity_id));
            }
            // A device's box stands for its addresses, which L2 draws nowhere else.
            NodeType::Container {
                container_type: ContainerType::Host,
                entity_id: Some(host_id),
                ..
            } => related.extend(ips_of_host.get(host_id).into_iter().flatten()),
            NodeType::Container { .. } => {}
        }
        if !related.is_empty() {
            let mut seen = HashSet::new();
            related.retain(|id| seen.insert(*id));
            out.insert(node.id, related);
        }
    }
    out
}
