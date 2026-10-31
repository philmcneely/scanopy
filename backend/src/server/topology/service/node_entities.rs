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
    let ip_of_interface: HashMap<Uuid, Uuid> = data
        .interfaces
        .iter()
        .filter_map(|i| Some((i.interface.id, i.interface.base.ip_address_id?)))
        .collect();
    // What each port's neighbour rows name: the far host, and the addresses reached through it. A
    // row naming the far interface reaches only the address that interface holds; a row naming
    // just the host, with the far port unknown, reaches all of the host's addresses.
    let mut neighbour_entities: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
    for row in &data.neighbours {
        let reached: Vec<Uuid> = match row.neighbor {
            Neighbor::Host(host_id) => std::iter::once(host_id)
                .chain(ips_of_host.get(&host_id).into_iter().flatten().copied())
                .collect(),
            Neighbor::Interface(id) => host_of_interface
                .get(&id)
                .into_iter()
                .copied()
                .chain(ip_of_interface.get(&id).copied())
                .collect(),
        };
        neighbour_entities
            .entry(row.interface_id)
            .or_default()
            .extend(reached);
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
                    // A port stands for the address it holds, and for the devices reached through
                    // it: those its forwarding table learned and those its neighbour rows name,
                    // each with the addresses on the far side.
                    ElementEntityType::Interface { interface_id } => {
                        related.extend(ip_of_interface.get(interface_id));
                        for host in learned.get(interface_id).into_iter().flatten() {
                            related.push(host.host_id);
                            related.extend(&host.ip_address_ids);
                        }
                        related.extend(neighbour_entities.get(interface_id).into_iter().flatten());
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::{
        interface_neighbors::r#impl::base::InterfaceNeighborRow,
        interfaces::r#impl::base::{Interface, InterfaceBase},
        ip_addresses::r#impl::base::{IPAddress, IPAddressBase},
        topology::types::api::TopologyInterface,
    };

    const SWITCH: Uuid = Uuid::from_u128(1);
    const NAS: Uuid = Uuid::from_u128(2);

    fn interface(host_id: Uuid, ip_address_id: Option<Uuid>) -> Interface {
        Interface::new(InterfaceBase {
            host_id,
            ip_address_id,
            ..Default::default()
        })
    }

    fn cable(local: &Interface, far: &Interface) -> InterfaceNeighborRow {
        InterfaceNeighborRow {
            id: Uuid::new_v4(),
            interface_id: local.id,
            neighbor: Neighbor::Interface(far.id),
            neighbor_seen_at: None,
        }
    }

    /// A NAS with a storage bond and a management port, each cabled to its own switch port: an
    /// address search lights the interface holding it and the switch port facing that interface,
    /// not the port facing the NAS's other interface.
    #[test]
    fn an_address_belongs_to_its_interface_and_the_port_cabled_to_it() {
        let storage = IPAddress::new(IPAddressBase {
            host_id: NAS,
            ..Default::default()
        });
        let management = IPAddress::new(IPAddressBase {
            host_id: NAS,
            ..Default::default()
        });
        let bond = interface(NAS, Some(storage.id));
        let mgmt = interface(NAS, Some(management.id));
        let port_2 = interface(SWITCH, None);
        let port_7 = interface(SWITCH, None);
        let data = TopologyData {
            ip_addresses: vec![storage.clone(), management.clone()],
            neighbours: vec![cable(&port_2, &bond), cable(&port_7, &mgmt)],
            interfaces: [&bond, &mgmt, &port_2, &port_7]
                .into_iter()
                .map(|i| TopologyInterface {
                    interface: i.clone(),
                    filter_values: Default::default(),
                })
                .collect(),
            ..Default::default()
        };
        let element = |i: &Interface| {
            Node::element(
                i.id,
                Uuid::nil(),
                i.base.host_id,
                ElementEntityType::Interface { interface_id: i.id },
            )
        };
        let nodes: Vec<Node> = [&bond, &mgmt, &port_2, &port_7]
            .into_iter()
            .map(element)
            .collect();

        let map = node_entities(&nodes, &data);
        let lit = |address: &IPAddress| -> HashSet<Uuid> {
            map.iter()
                .filter(|(_, ids)| ids.contains(&address.id))
                .map(|(node, _)| *node)
                .collect()
        };
        assert_eq!(lit(&storage), HashSet::from([bond.id, port_2.id]));
        assert_eq!(lit(&management), HashSet::from([mgmt.id, port_7.id]));
    }
}
