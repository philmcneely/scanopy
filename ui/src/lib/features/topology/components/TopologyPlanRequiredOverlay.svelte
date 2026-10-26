<script lang="ts">
	/**
	 * The topology view on a plan without it (Free). Covers the viewer and opens
	 * the plan picker; the rest of the app stays usable.
	 */
	import TopologyOverlay from './TopologyOverlay.svelte';
	import { Network } from 'lucide-svelte';
	import { triggerUpgrade } from '$lib/features/billing/trigger-upgrade';
	import type { ModalAction } from '$lib/shared/components/layout/modalActions';
	import {
		billing_lapsedBannerCta,
		topology_planRequiredBody,
		topology_planRequiredTitle
	} from '$lib/paraglide/messages';

	let actions = $derived<ModalAction[]>([
		{
			role: 'primary',
			label: billing_lapsedBannerCta(),
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

<TopologyOverlay title={topology_planRequiredTitle()} size="sm" {actions}>
	<div class="flex flex-col items-center gap-4 p-6 text-center">
		<div class="rounded-full bg-blue-500/10 p-3">
			<Network class="h-8 w-8 text-blue-500" />
		</div>
		<p class="text-sm text-secondary">{topology_planRequiredBody()}</p>
	</div>
</TopologyOverlay>
