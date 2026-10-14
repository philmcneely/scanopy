/**
 * TanStack Query hooks for VLANs
 *
 * VLANs are populated by SNMP discovery, so there are no create or delete hooks.
 * Users edit a discovered VLAN's description through the update hook.
 */

import { createMutation, createQuery, useQueryClient } from '@tanstack/svelte-query';
import { queryKeys } from '$lib/api/query-client';
import { apiClient } from '$lib/api/client';
import { unwrapData } from '$lib/api/query-helpers';
import type { Vlan } from './types/base';
import { invalidateListPages, useListPageQuery, type ListPageParams } from '$lib/api/list-page';

/**
 * VLANs list. Called with no arguments this is the shared full-list cache, so
 * any narrowing argument must also change the query key — otherwise a filtered
 * fetch would overwrite the shared cache with a subset.
 */
export function useVlansQuery(atGetter?: () => string | undefined) {
	return createQuery(() => {
		const at = atGetter?.();
		return {
			queryKey: at ? [...queryKeys.vlans.all, 'asOf', at] : queryKeys.vlans.all,
			queryFn: async () => {
				return unwrapData(
					await apiClient.GET('/api/v1/vlans', {
						params: { query: { limit: 0, at } }
					})
				);
			}
		};
	});
}

/** One page of the VLANs list, kept apart from the full list other consumers read. */
export function useVlansListQuery(params: () => ListPageParams) {
	return useListPageQuery<Vlan>(
		queryKeys.vlans.all,
		({ query, querySerializer }) =>
			apiClient.GET('/api/v1/vlans', { params: { query }, querySerializer }),
		params
	);
}

/**
 * Mutation hook for updating a VLAN
 */
export function useUpdateVlanMutation() {
	const queryClient = useQueryClient();

	return createMutation(() => ({
		mutationFn: async (vlan: Vlan) => {
			return unwrapData(
				await apiClient.PUT('/api/v1/vlans/{id}', {
					params: { path: { id: vlan.id } },
					body: vlan
				})
			);
		},
		onSuccess: (updatedVlan: Vlan) => {
			queryClient.setQueryData<Vlan[]>(
				queryKeys.vlans.all,
				(old) => old?.map((v) => (v.id === updatedVlan.id ? updatedVlan : v)) ?? []
			);
			invalidateListPages(queryClient, queryKeys.vlans.all);
		}
	}));
}
