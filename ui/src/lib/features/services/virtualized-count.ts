import { apiClient } from '$lib/api/client';
import { unwrapEnvelope } from '$lib/api/query-helpers';

/**
 * How many services and hosts the services in `serviceIds` virtualize: a runtime's containers, a
 * hypervisor's VMs. Deleting a service deletes them too, so the confirmation says how many.
 */
export async function countVirtualized(serviceIds: string[]): Promise<number> {
	if (serviceIds.length === 0) return 0;
	const query = { virtualization_service_ids: serviceIds, limit: 1 };
	const [services, hosts] = await Promise.all([
		apiClient.GET('/api/v1/services', { params: { query } }),
		apiClient.GET('/api/v1/hosts', { params: { query } })
	]);
	return (
		(unwrapEnvelope(services).meta?.pagination?.total_count ?? 0) +
		(unwrapEnvelope(hosts).meta?.pagination?.total_count ?? 0)
	);
}

/** How many services and hosts the services on `hostId` virtualize, which deleting it deletes. */
export async function countVirtualizedOnHost(hostId: string): Promise<number> {
	const services = unwrapEnvelope(
		await apiClient.GET('/api/v1/services', { params: { query: { host_ids: [hostId], limit: 0 } } })
	).data;
	return countVirtualized(services.map((s) => s.id));
}
