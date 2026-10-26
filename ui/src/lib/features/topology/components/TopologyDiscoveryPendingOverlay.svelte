<script lang="ts">
	/**
	 * The topology view on a plan without it (Free), before the first discovery
	 * has finished: nothing to map yet, so the scan's progress shows instead of
	 * the plan prompt, which takes over once discovery completes.
	 */
	import TopologyOverlay from './TopologyOverlay.svelte';
	import SessionProgress from '$lib/features/discovery/components/cards/SessionProgress.svelte';
	import DiscoveryEstimation from '$lib/features/discovery/components/DiscoveryEstimation.svelte';
	import { isFullScan } from '$lib/features/discovery/utils/fullScan';
	import { useActiveSessionsQuery } from '$lib/features/discovery/queries';
	import { topology_readyWhenDiscoveryCompletes } from '$lib/paraglide/messages';
	import { discoveryPhases } from '$lib/shared/stores/metadata';

	const activeSessionsQuery = useActiveSessionsQuery();
	let sessions = $derived(activeSessionsQuery.data ?? []);
</script>

<TopologyOverlay title={topology_readyWhenDiscoveryCompletes()} size="lg">
	<div class="space-y-4 p-6">
		{#each sessions as session (session.session_id)}
			<div class="space-y-1">
				<DiscoveryEstimation
					phase={session.phase}
					hosts_discovered={session.hosts_discovered}
					estimated_remaining_secs={session.estimated_remaining_secs}
					is_full_scan={isFullScan(session.discovery_type)}
				/>
				<div class="flex items-center gap-3">
					<span class="shrink-0 text-sm font-medium text-primary">
						{discoveryPhases.getName(session.phase)}
					</span>
					<div class="min-w-0 flex-1">
						<SessionProgress
							session_id={session.session_id}
							phase={session.phase}
							progress={session.progress}
						/>
					</div>
				</div>
			</div>
		{/each}
	</div>
</TopologyOverlay>
