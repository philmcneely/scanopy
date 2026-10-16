/**
 * TanStack Query hooks for VLANs
 *
 * VLANs are populated by SNMP discovery, so there is no create hook. Users edit a
 * discovered VLAN's description, or delete it.
 */

import { createMutation, createQuery, useQueryClient } from '@tanstack/svelte-query';
import { queryKeys } from '$lib/api/query-client';
import { apiClient } from '$lib/api/client';
import { requireSuccess, unwrapData } from '$lib/api/query-helpers';
import type { Vlan } from './types/base';

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
		}
	}));
}

/**
 * Mutation hook for deleting a VLAN
 */
export function useDeleteVlanMutation() {
	const queryClient = useQueryClient();

	return createMutation(() => ({
		mutationFn: async (id: string) => {
			requireSuccess(
				await apiClient.DELETE('/api/v1/vlans/{id}', {
					params: { path: { id } }
				})
			);
			return id;
		},
		onSuccess: (id: string) => {
			queryClient.setQueryData<Vlan[]>(
				queryKeys.vlans.all,
				(old) => old?.filter((v) => v.id !== id) ?? []
			);
		}
	}));
}

/**
 * Mutation hook for bulk deleting VLANs
 */
export function useBulkDeleteVlansMutation() {
	const queryClient = useQueryClient();

	return createMutation(() => ({
		mutationFn: async (ids: string[]) => {
			requireSuccess(await apiClient.POST('/api/v1/vlans/bulk-delete', { body: ids }));
			return ids;
		},
		onSuccess: (ids: string[]) => {
			queryClient.setQueryData<Vlan[]>(
				queryKeys.vlans.all,
				(old) => old?.filter((v) => !ids.includes(v.id)) ?? []
			);
		}
	}));
}
