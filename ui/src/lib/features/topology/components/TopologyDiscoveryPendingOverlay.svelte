<script lang="ts">
	/**
	 * The topology view on a plan without it (Free), before the first discovery
	 * has finished: nothing to map yet, so the scan's progress shows instead of
	 * the plan prompt, which takes over once discovery completes.
	 */
	import TopologyOverlay from './TopologyOverlay.svelte';
	import SessionPhaseProgress from '$lib/features/discovery/components/cards/SessionPhaseProgress.svelte';
	import { useActiveSessionsQuery } from '$lib/features/discovery/queries';
	import { topology_readyWhenDiscoveryCompletes } from '$lib/paraglide/messages';

	const activeSessionsQuery = useActiveSessionsQuery();
	let sessions = $derived(activeSessionsQuery.data ?? []);
</script>

<TopologyOverlay title={topology_readyWhenDiscoveryCompletes()} size="lg">
	<div class="space-y-4 p-6">
		{#each sessions as session (session.session_id)}
			<SessionPhaseProgress {session} />
		{/each}
	</div>
</TopologyOverlay>
