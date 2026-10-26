<script lang="ts">
	/**
	 * The topology view on a plan without it (Free). Covers the viewer and opens the plan picker;
	 * the rest of the app stays usable. It shows from the first scan on: while that scan runs it
	 * lists the scan's progress, so the reason for the gate and the trial appear while the user
	 * watches, and a trial started then has the map ready when the scan finishes. Afterwards the
	 * org's own diagram sits shrouded behind it.
	 */
	import TopologyOverlay from './TopologyOverlay.svelte';
	import SessionPhaseProgress from '$lib/features/discovery/components/cards/SessionPhaseProgress.svelte';
	import InfoCard from '$lib/shared/components/data/InfoCard.svelte';
	import { useActiveSessionsQuery } from '$lib/features/discovery/queries';
	import { useOrganizationQuery } from '$lib/features/organizations/queries';
	import PaidCrownOnPrimary from '$lib/shared/components/PaidCrownOnPrimary.svelte';
	import { triggerUpgrade } from '$lib/features/billing/trigger-upgrade';
	import { canTrialTopology, topologyTrialDays } from '$lib/shared/utils/trial';
	import type { ModalAction } from '$lib/shared/components/layout/modalActions';
	import {
		billing_startDayFreeTrial,
		common_firstDiscoveryEmailHint,
		common_upgrade,
		topology_planRequiredBody,
		topology_planRequiredScanningBody,
		topology_planRequiredTitle
	} from '$lib/paraglide/messages';

	let { scanning = false }: { scanning?: boolean } = $props();

	const organizationQuery = useOrganizationQuery();
	const activeSessionsQuery = useActiveSessionsQuery();
	let sessions = $derived(scanning ? (activeSessionsQuery.data ?? []) : []);
	let trial = $derived(canTrialTopology(organizationQuery.data));

	let actions = $derived<ModalAction[]>([
		{
			role: 'primary',
			// The trial is the offer itself, so its length goes on the button rather than in a caption.
			label: trial ? billing_startDayFreeTrial({ days: topologyTrialDays() }) : common_upgrade(),
			icon: PaidCrownOnPrimary,
			onClick: () =>
				triggerUpgrade({
					feature: 'topology_view',
					source: 'topology_plan_required',
					surface: 'topology_tab',
					gate_type: 'plan_required'
				})
		}
	]);
</script>

<TopologyOverlay title={topology_planRequiredTitle()} size={scanning ? 'lg' : 'sm'} {actions}>
	<div class="space-y-4 p-6">
		<p class="text-sm text-secondary">
			{scanning ? topology_planRequiredScanningBody() : topology_planRequiredBody()}
		</p>
		{#if scanning}
			<!-- The scan's status in a panel of its own, apart from the pitch above. -->
			<InfoCard variant="compact">
				{#each sessions as session (session.session_id)}
					<SessionPhaseProgress {session} />
				{/each}
				<p class="text-xs text-tertiary">{common_firstDiscoveryEmailHint()}</p>
			</InfoCard>
		{/if}
	</div>
</TopologyOverlay>
