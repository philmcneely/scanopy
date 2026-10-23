<script lang="ts" module>
	import type { EntityRef } from '$lib/shared/components/data/types';
	import type { Color } from '$lib/shared/utils/styling';
	import type { IconComponent } from '$lib/shared/utils/types';

	export interface EntityTagOption {
		id: string;
		entityRef: EntityRef;
		label: string;
		icon?: IconComponent | null;
		color?: Color;
	}
</script>

<script lang="ts">
	import { ChevronDown } from 'lucide-svelte';
	import EntityTag from '$lib/shared/components/data/EntityTag.svelte';
	import Popover from '$lib/shared/components/data/Popover.svelte';
	import { common_selectOption, common_noOptionsAvailable } from '$lib/paraglide/messages';

	let {
		options,
		selectedValue,
		onSelect,
		placeholder = undefined,
		disabled = false,
		error = null
	}: {
		options: EntityTagOption[];
		selectedValue: string | null;
		onSelect: (id: string) => void;
		placeholder?: string;
		disabled?: boolean;
		error?: string | null;
	} = $props();

	let isOpen = $state(false);
	let triggerEl: HTMLButtonElement | undefined = $state();

	let selected = $derived(options.find((o) => o.id === selectedValue));

	function toggle(e: MouseEvent) {
		e.preventDefault();
		e.stopPropagation();
		if (disabled || options.length === 0) return;
		isOpen = !isOpen;
	}

	function handleSelect(id: string) {
		onSelect(id);
		isOpen = false;
	}
</script>

<div class="relative">
	<button
		bind:this={triggerEl}
		type="button"
		onclick={toggle}
		class="select-trigger text-primary flex w-full items-center justify-between gap-2 rounded-md px-2 py-1.5
           {error ? 'border-red-500' : ''}
           {disabled || options.length === 0 ? 'cursor-not-allowed opacity-50' : ''}"
		disabled={disabled || options.length === 0}
	>
		{#if selected}
			<EntityTag
				entityRef={selected.entityRef}
				label={selected.label}
				icon={selected.icon ?? null}
				color={selected.color ?? 'Gray'}
				disableNavigate={true}
				disablePopover={true}
			/>
		{:else}
			<span class="text-secondary text-sm">
				{options.length === 0
					? common_noOptionsAvailable()
					: (placeholder ?? common_selectOption())}
			</span>
		{/if}
		<ChevronDown
			class="text-tertiary h-4 w-4 flex-shrink-0 transition-transform {isOpen ? 'rotate-180' : ''}"
		/>
	</button>
	{#if error}
		<div class="text-danger mt-1 text-xs">{error}</div>
	{/if}
</div>

<Popover
	triggerElement={triggerEl ?? null}
	{isOpen}
	role="presentation"
	align="start"
	matchTriggerWidth
	onClose={() => (isOpen = false)}
>
	<div class="flex flex-col items-start gap-1">
		{#each options as option (option.id)}
			<button
				type="button"
				onclick={() => handleSelect(option.id)}
				class="flex w-full rounded p-1 text-left transition-colors hover:bg-gray-100 dark:hover:bg-gray-800 {option.id ===
				selectedValue
					? 'ring-1 ring-blue-500'
					: ''}"
			>
				<EntityTag
					entityRef={option.entityRef}
					label={option.label}
					icon={option.icon ?? null}
					color={option.color ?? 'Gray'}
					disableNavigate={true}
					disablePopover={true}
				/>
			</button>
		{/each}
	</div>
</Popover>
