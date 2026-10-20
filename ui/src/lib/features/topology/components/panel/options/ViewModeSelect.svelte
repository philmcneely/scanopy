<script lang="ts">
	import RichSelect from '$lib/shared/components/forms/selection/RichSelect.svelte';
	import {
		SimpleOptionDisplay,
		type SimpleOption
	} from '$lib/shared/components/forms/selection/display/SimpleOptionDisplay';
	import { activeView, topologyOptions, updateTopologyOptions } from '../../../queries';
	import { effectiveViewMode, viewModesFor, type ViewMode } from '../../../view-mode';
	import { metaName, metaDescription } from '$lib/i18n/metadata';
	import { topology_viewMode, topology_viewModeHelp } from '$lib/paraglide/messages';

	let modes = $derived(viewModesFor($activeView));
	let options: SimpleOption[] = $derived(
		modes.map((mode) => ({
			value: mode.id,
			label: metaName('view_modes', mode.id, mode.name),
			description: metaDescription('view_modes', mode.id, mode.description)
		}))
	);

	let selected = $derived(effectiveViewMode($topologyOptions.local, $activeView));

	function handleSelect(value: string) {
		if (value === selected) return;
		updateTopologyOptions((opts) => ({
			...opts,
			local: { ...opts.local, view_mode: value as ViewMode }
		}));
	}
</script>

<!-- A view that offers only one mode has nothing to choose. -->
{#if modes.length > 1}
	<RichSelect
		label={topology_viewMode()}
		selectedValue={selected}
		{options}
		onSelect={handleSelect}
		displayComponent={SimpleOptionDisplay}
		helpText={topology_viewModeHelp()}
	/>
{/if}
