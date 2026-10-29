//! Binding validation, conflict detection, and deduplication helpers.
use super::*;

impl ServiceService {
    /// Validate that all bindings reference ports/interfaces that belong to the service's host.
    /// Returns Ok(()) if all bindings are valid, Err with ValidationError if any are invalid.
    pub(crate) async fn validate_bindings_belong_to_host(
        &self,
        host_id: &Uuid,
        bindings: &[Binding],
    ) -> Result<()> {
        if bindings.is_empty() {
            return Ok(());
        }

        let host_service = self
            .host_service
            .get()
            .expect("host_service not initialized");

        // Get all ports and ip_addresses for this host
        let host_ports = host_service.get_ports_for_host(host_id).await?;
        let host_interfaces = host_service.get_ip_addresses_for_host(host_id).await?;

        let valid_port_ids: std::collections::HashSet<Uuid> =
            host_ports.iter().map(|p| p.id).collect();
        let valid_ip_address_ids: std::collections::HashSet<Uuid> =
            host_interfaces.iter().map(|i| i.id).collect();

        for binding in bindings {
            match &binding.base.binding_type {
                BindingType::IPAddress { ip_address_id } => {
                    if !valid_ip_address_ids.contains(ip_address_id) {
                        return Err(ValidationError::new(format!(
                            "IP address binding references ip_address {} which does not belong to this host",
                            ip_address_id
                        )).into());
                    }
                }
                BindingType::Port {
                    port_id,
                    ip_address_id,
                } => {
                    if !valid_port_ids.contains(port_id) {
                        return Err(ValidationError::new(format!(
                            "Port binding references port {} which does not belong to this host",
                            port_id
                        ))
                        .into());
                    }
                    if let Some(iface_id) = ip_address_id
                        && !valid_ip_address_ids.contains(iface_id)
                    {
                        return Err(ValidationError::new(format!(
                                "Port binding references ip_address {} which does not belong to this host",
                                iface_id
                            )).into());
                    }
                }
            }
        }

        Ok(())
    }

    /// Check if a new binding is already covered by existing bindings.
    /// A Port binding with a specific interface is covered if there's already
    /// a Port binding for the same port with ip_address_id = None (all ip_addresses).
    pub(crate) fn is_binding_covered_by_existing(
        new_binding: &Binding,
        existing_bindings: &[Binding],
    ) -> bool {
        match &new_binding.base.binding_type {
            // A Port binding with a specific interface is covered by an "all ip_addresses" binding for the same port
            BindingType::Port {
                port_id,
                ip_address_id: Some(_),
            } => existing_bindings.iter().any(|existing| {
                matches!(
                    &existing.base.binding_type,
                    BindingType::Port {
                        port_id: existing_port_id,
                        ip_address_id: None,
                    } if existing_port_id == port_id
                )
            }),
            // Other binding types are not covered by anything else
            _ => false,
        }
    }

    /// Validates that a binding doesn't conflict with existing bindings.
    /// Rules:
    /// - Interface binding conflicts with port bindings on same interface OR port bindings on all ip_addresses
    /// - Port binding (specific ip_address) conflicts with interface binding on same interface
    /// - Port binding (all ip_addresses) conflicts with ANY interface binding
    ///
    /// Returns None if valid, Some(error_message) if conflict found.
    pub(crate) fn validate_binding_no_conflict(
        new_binding: &BindingType,
        existing_bindings: &[Binding],
    ) -> Option<&'static str> {
        match new_binding {
            BindingType::IPAddress { ip_address_id } => {
                // Check for conflicting port bindings: same interface OR all-interfaces
                for existing in existing_bindings {
                    if let BindingType::Port {
                        ip_address_id: existing_iface,
                        ..
                    } = &existing.base.binding_type
                        && (*existing_iface == Some(*ip_address_id) || existing_iface.is_none())
                    {
                        return Some(
                            "Cannot add ip_address binding: service already has a port binding on this ip_address (or on all ip_addresses).",
                        );
                    }
                }
            }
            BindingType::Port {
                ip_address_id: Some(ip_address_id),
                ..
            } => {
                // Check for conflicting interface binding on same interface
                for existing in existing_bindings {
                    if let BindingType::IPAddress {
                        ip_address_id: existing_iface,
                    } = &existing.base.binding_type
                        && existing_iface == ip_address_id
                    {
                        return Some(
                            "Cannot add port binding: service already has an ip_address binding on this ip_address.",
                        );
                    }
                }
            }
            BindingType::Port {
                ip_address_id: None,
                ..
            } => {
                // Port binding on all ip_addresses: conflicts with ANY interface binding
                for existing in existing_bindings {
                    if matches!(existing.base.binding_type, BindingType::IPAddress { .. }) {
                        return Some(
                            "Cannot add port binding on all ip_addresses: service already has ip_address bindings.",
                        );
                    }
                }
            }
        }
        None
    }

    /// Deduplicate bindings in a list.
    /// - Removes exact duplicates (same binding_type)
    /// - When an all-interfaces port binding is present, removes specific-interface bindings for the same port
    pub(crate) fn deduplicate_bindings(bindings: Vec<Binding>) -> Vec<Binding> {
        use std::collections::HashSet;

        // First, collect all port_ids that have all-interfaces bindings
        let all_interface_port_ids: HashSet<Uuid> = bindings
            .iter()
            .filter_map(|b| {
                if let BindingType::Port {
                    port_id,
                    ip_address_id: None,
                } = &b.base.binding_type
                {
                    Some(*port_id)
                } else {
                    None
                }
            })
            .collect();

        // Track seen binding types for deduplication
        let mut seen_binding_types: HashSet<String> = HashSet::new();
        let mut result = Vec::new();

        for binding in bindings {
            // Skip specific-interface port bindings when an all-interfaces binding exists for the same port
            if let BindingType::Port {
                port_id,
                ip_address_id: Some(_),
            } = &binding.base.binding_type
                && all_interface_port_ids.contains(port_id)
            {
                continue;
            }

            // Create a key for deduplication based on binding type
            let key = format!("{:?}", binding.base.binding_type);
            if seen_binding_types.contains(&key) {
                continue;
            }

            seen_binding_types.insert(key);
            result.push(binding);
        }

        result
    }

    /// Merge a rediscovered service's bindings into the stored ones, returning how many changed.
    ///
    /// Additive: a stored binding the rediscovery did not see is kept. A port binding on an
    /// address replaces a stored binding to that whole address. Discovery binds the whole address
    /// only when a match named no port, so a match that now names ports is the more specific
    /// statement about the same service, and it reads the way a fresh match would. An eero matched
    /// on its MAC alone, then again with the port its definition later learned to bind, kept the
    /// address binding and dropped the port: the port then belonged to no service at all. Every
    /// other conflict keeps the stored binding.
    pub(crate) fn merge_rediscovered_bindings(
        existing: &mut Vec<Binding>,
        incoming: &[Binding],
    ) -> usize {
        let mut updates = 0;

        for binding in incoming {
            // Check if this binding is already covered by existing bindings
            // (e.g., a specific interface binding is covered by an "all ip_addresses" binding for the same port)
            if Self::is_binding_covered_by_existing(binding, existing) {
                continue;
            }

            if let BindingType::Port {
                ip_address_id: Some(ip_address_id),
                ..
            } = &binding.base.binding_type
            {
                let before = existing.len();
                existing.retain(|stored| {
                    !matches!(
                        &stored.base.binding_type,
                        BindingType::IPAddress { ip_address_id: stored_ip } if stored_ip == ip_address_id
                    )
                });
                updates += before - existing.len();
            }

            // Check for binding type conflicts (Interface vs Port on same ip_address)
            if let Some(conflict_msg) =
                Self::validate_binding_no_conflict(&binding.base.binding_type, existing)
            {
                tracing::warn!(
                    "Skipping binding {:?} - conflicts with existing binding: {}",
                    binding.base.binding_type,
                    conflict_msg
                );
                continue;
            }

            // If new binding is "all ip_addresses" port binding, remove specific interface bindings for same port
            // (the all-interfaces binding supersedes them)
            if let BindingType::Port {
                port_id,
                ip_address_id: None,
            } = &binding.base.binding_type
            {
                let before = existing.len();
                existing.retain(|stored| {
                    !matches!(
                        &stored.base.binding_type,
                        BindingType::Port {
                            port_id: stored_port_id,
                            ip_address_id: Some(_),
                        } if stored_port_id == port_id
                    )
                });
                updates += before - existing.len();
            }

            if !existing.contains(binding) {
                updates += 1;
                existing.push(*binding);
            }
        }

        updates
    }

    /// Validate all bindings in a list don't conflict with each other.
    /// Returns Ok(()) if all bindings are valid, Err with message if any conflict.
    pub(crate) fn validate_bindings_no_conflicts(bindings: &[Binding]) -> Result<()> {
        for (i, binding) in bindings.iter().enumerate() {
            // Check against all bindings before this one (to avoid duplicate checks)
            let preceding_bindings = &bindings[..i];
            if let Some(error_msg) =
                Self::validate_binding_no_conflict(&binding.base.binding_type, preceding_bindings)
            {
                return Err(ValidationError::new(error_msg).into());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn types(bindings: &[Binding]) -> Vec<BindingType> {
        bindings.iter().map(|b| b.base.binding_type).collect()
    }

    #[test]
    fn a_rediscovered_port_replaces_the_binding_to_its_whole_address() {
        let (ip, port) = (Uuid::new_v4(), Uuid::new_v4());
        let mut stored = vec![Binding::new_ip_address_serviceless(ip)];

        let updates = ServiceService::merge_rediscovered_bindings(
            &mut stored,
            &[Binding::new_port_serviceless(port, Some(ip))],
        );

        assert_eq!(
            types(&stored),
            vec![BindingType::Port {
                port_id: port,
                ip_address_id: Some(ip)
            }]
        );
        assert_eq!(updates, 2);
    }

    #[test]
    fn a_rediscovery_without_ports_keeps_the_stored_ports() {
        let (ip, port) = (Uuid::new_v4(), Uuid::new_v4());
        let mut stored = vec![Binding::new_port_serviceless(port, Some(ip))];

        let updates = ServiceService::merge_rediscovered_bindings(
            &mut stored,
            &[Binding::new_ip_address_serviceless(ip)],
        );

        assert_eq!(
            types(&stored),
            vec![BindingType::Port {
                port_id: port,
                ip_address_id: Some(ip)
            }]
        );
        assert_eq!(updates, 0);
    }

    #[test]
    fn a_port_on_one_address_leaves_the_binding_to_another() {
        let (ip, other_ip, port) = (Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4());
        let mut stored = vec![Binding::new_ip_address_serviceless(other_ip)];

        ServiceService::merge_rediscovered_bindings(
            &mut stored,
            &[Binding::new_port_serviceless(port, Some(ip))],
        );

        assert_eq!(
            types(&stored),
            vec![
                BindingType::IPAddress {
                    ip_address_id: other_ip
                },
                BindingType::Port {
                    port_id: port,
                    ip_address_id: Some(ip)
                },
            ]
        );
    }
}
