import type { Host, Interface } from './types/base';

/**
 * What the resolve modal shows for an unconfirmed host: who reported it, the scan that found it,
 * and the identifiers it is known by.
 *
 * A listed host is a primitive with its children stripped (`toHostPrimitive`), so its interfaces
 * come in separately, from the cache the Hosts list fills. Taking them as an argument rather than
 * reading an optional field off the host is what keeps a forwarding-table host's MAC from going
 * missing without anything noticing.
 */
export interface UnconfirmedEvidence {
	reportedByHostIds: string[];
	sourceDiscoveryId: string | null;
	chassisId: string | null;
	sysName: string | null;
	macs: string[];
}

export function unconfirmedEvidence(host: Host, interfaces: Interface[]): UnconfirmedEvidence {
	const macs = interfaces
		.filter((i) => i.host_id === host.id)
		.flatMap((i) => (i.mac_address ? [i.mac_address] : []));
	return {
		reportedByHostIds: host.reported_by_host_ids ?? [],
		sourceDiscoveryId: host.found_by_discovery_id ?? null,
		chassisId: host.chassis_id ?? null,
		// A neighbour that advertised no separate system name repeats its chassis ID.
		sysName: host.sys_name && host.sys_name !== host.chassis_id ? host.sys_name : null,
		macs: [...new Set(macs)]
	};
}
