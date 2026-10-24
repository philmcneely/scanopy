<script lang="ts" module>
	/** Flow units the External title pill sits above its container's top edge. */
	export const EXTERNAL_TITLE_OFFSET = 40;
</script>

<script lang="ts">
	import { onMount } from 'svelte';
	import { Search } from 'lucide-svelte';
	// ChevronDown / ChevronRight replaced by the CSS `.caret` below — see the note on that rule.
	import Tag from '$lib/shared/components/data/Tag.svelte';
	import HiddenTagsChip from '$lib/shared/components/forms/selection/HiddenTagsChip.svelte';
	import { HIDDEN_TAGS_CHIP_WIDTH } from '$lib/shared/components/forms/selection/display-tags';
	import type { ColorStyle, Color } from '$lib/shared/utils/styling';
	import type { IconComponent } from '$lib/shared/utils/types';
	import type { TagProps } from '$lib/shared/components/data/types';
	import type { components } from '$lib/api/schema';
	import DeviceGlyph from '$lib/features/hosts/components/DeviceGlyph.svelte';
	import SoftwareGlyph from './SoftwareGlyph.svelte';
	import {
		topology_elementCount,
		topology_ungroupedCount,
		topology_searchContainerMatches
	} from '$lib/paraglide/messages';
	import { tooltip as tooltipAction, type TooltipContent } from '$lib/shared/actions/tooltip';

	/** A group container's pill: a category or tag the group collects, with its tooltip. */
	export type GroupPill = { label: string; color: Color; title?: string | TooltipContent };

	export type SubgroupRow = {
		/** Service definition whose logo marks the subgroup, drawn round as software. */
		logoDefinition: string | null;
		headerText: string;
		labels: GroupPill[];
		childCount: number;
		/** Pre-formatted entity-count summary, e.g. "5 services" or "3 services, 2 hosts". */
		childSummary: string;
		hideCount?: boolean;
		countOnly?: boolean;
	};

	let {
		isCollapsed,
		isCollapsible,
		headerText,
		iconComponent = null,
		logoDefinition = null,
		deviceType = null,
		fillIcon = false,
		colorHelper,
		groupLabels = [],
		childCount = 0,
		childSummary = '',
		ungroupedSummary = '',
		onToggleCollapse,
		variant,
		subgroupSummaries = [],
		tagHoverRingStyle = '',
		hideCount = false,
		countOnly = false,
		searchMatchCount = 0,
		searchHighlightRingStyle = '',
		staleTag = null,
		tooltip = null
	}: {
		isCollapsed: boolean;
		isCollapsible: boolean;
		headerText: string;
		iconComponent: IconComponent | null;
		/** Service definition whose logo marks the container, drawn round as software. */
		logoDefinition: string | null;
		/**
		 * The device type a container standing for a device draws, as the same square tile as its icon card,
		 * in place of its icon. Expanded or collapsed, the tile holds the header's identity slot.
		 */
		deviceType?: components['schemas']['DeviceType'] | null;
		fillIcon: boolean;
		colorHelper: ColorStyle;
		groupLabels: GroupPill[];
		childCount: number;
		/** Pre-formatted entity-count summary for the container's children, e.g. "5 services, 2 hosts". */
		childSummary: string;
		/** Pre-formatted summary for the ungrouped remainder (collapsed-root variant). */
		ungroupedSummary?: string;
		onToggleCollapse: (event: MouseEvent | KeyboardEvent) => void;
		variant: 'external' | 'inline' | 'collapsed-sub' | 'collapsed-root';
		subgroupSummaries?: SubgroupRow[];
		tagHoverRingStyle?: string;
		hideCount?: boolean;
		countOnly?: boolean;
		searchMatchCount?: number;
		searchHighlightRingStyle?: string;
		/** Staleness pill for the entity this container stands for, from `getFreshnessTag`. */
		staleTag?: TagProps | null;
		/** Hover text for the container's name: the entity it stands for, or what the group holds. */
		tooltip?: TooltipContent | null;
	} = $props();

	let subgroupTotal = $derived(subgroupSummaries.reduce((sum, s) => sum + s.childCount, 0));
	let ungroupedCount = $derived(childCount - subgroupTotal);

	// Tag truncation for inline variant: measure available space, show the (i) icon for overflow
	const TAG_GAP = 4;
	const MORE_WIDTH = HIDDEN_TAGS_CHIP_WIDTH;
	let inlineContainerEl: HTMLDivElement | undefined = $state(undefined);
	let inlineMeasureEl: HTMLDivElement | undefined = $state(undefined);
	// Derived from the label count by default; `calculateVisibleLabels`
	// reassigns this to a smaller value when measured width forces truncation.
	// Reassignment is reset on the next groupLabels change.
	let visibleLabelCount = $derived(groupLabels.length);

	function calculateVisibleLabels() {
		if (!inlineContainerEl || !inlineMeasureEl || groupLabels.length === 0) {
			visibleLabelCount = groupLabels.length;
			return;
		}

		const containerWidth = inlineContainerEl.offsetWidth;
		const tagEls = inlineMeasureEl.querySelectorAll('[data-tag]');
		const tagWidths: number[] = [];
		tagEls.forEach((el) => tagWidths.push((el as HTMLElement).offsetWidth));

		if (tagWidths.length === 0) {
			visibleLabelCount = groupLabels.length;
			return;
		}

		// Space used by non-tag elements (chevron, icon, header text)
		const fixedEls = inlineContainerEl.querySelectorAll('[data-fixed]');
		let fixedWidth = 0;
		fixedEls.forEach((el) => (fixedWidth += (el as HTMLElement).offsetWidth + TAG_GAP));

		let availableForTags = containerWidth - fixedWidth;
		let count = 0;
		let usedWidth = 0;

		for (let i = 0; i < tagWidths.length; i++) {
			const needsMore = i < tagWidths.length - 1;
			const extraWidth =
				(count > 0 ? TAG_GAP : 0) + tagWidths[i] + (needsMore ? TAG_GAP + MORE_WIDTH : 0);

			if (usedWidth + extraWidth <= availableForTags) {
				count++;
				usedWidth += (count > 1 ? TAG_GAP : 0) + tagWidths[i];
			} else {
				break;
			}
		}

		if (count < tagWidths.length && count > 1) {
			const totalWithMore = usedWidth + TAG_GAP + MORE_WIDTH;
			if (totalWithMore > availableForTags) {
				count--;
			}
		}

		visibleLabelCount = Math.max(count, 0);
	}

	let needsTruncation = $derived(
		(variant === 'inline' || variant === 'collapsed-sub') && groupLabels.length > 0
	);

	$effect(() => {
		if (needsTruncation && inlineContainerEl) {
			requestAnimationFrame(() => calculateVisibleLabels());
		}
	});

	onMount(() => {
		if (needsTruncation && inlineContainerEl) {
			const observer = new ResizeObserver(() => calculateVisibleLabels());
			observer.observe(inlineContainerEl);
			return () => observer.disconnect();
		}
	});

	let visibleLabels = $derived(groupLabels.slice(0, visibleLabelCount));
	let hiddenLabelCount = $derived(groupLabels.length - visibleLabelCount);
</script>

<!-- Hidden measurement container for tag widths (shared by inline + collapsed-sub) -->
{#if needsTruncation}
	<div
		bind:this={inlineMeasureEl}
		class="pointer-events-none invisible absolute flex items-center gap-1"
		aria-hidden="true"
	>
		{#each groupLabels as pill (pill.label)}
			<span data-tag><Tag label={pill.label} color={pill.color} title={pill.title ?? ''} /></span>
		{/each}
	</div>
{/if}

{#if variant === 'external'}
	<!-- External title pill: card above container -->
	<div
		class="nopan nodrag card absolute left-0 z-100 flex max-w-full cursor-pointer items-center gap-1 px-2 py-1 text-secondary shadow-lg backdrop-blur-xs"
		style:top="-{EXTERNAL_TITLE_OFFSET}px"
		role="button"
		tabindex={-1}
		onclick={onToggleCollapse}
		onkeydown={(e) => {
			if (e.key === 'Enter' || e.key === ' ') onToggleCollapse(e);
		}}
		onmousedown={(e) => e.stopPropagation()}
		onpointerdown={(e) => e.stopPropagation()}
	>
		{#if isCollapsible}
			<span class="caret text-secondary" class:caret-collapsed={isCollapsed}></span>
		{/if}

		{#if deviceType}
			<DeviceGlyph {deviceType} />
		{:else if logoDefinition}
			<SoftwareGlyph definition={logoDefinition} framed={false} />
		{:else if iconComponent}
			{@const IconComp = iconComponent}
			<IconComp class={`h-5 w-5 ${colorHelper.icon}`} fill={fillIcon ? 'currentColor' : 'none'} />
		{/if}

		{#if headerText}
			<span
				data-entity-header
				class="text-s min-w-0 truncate font-medium text-secondary"
				use:tooltipAction={{ content: tooltip }}
			>
				{headerText}
			</span>
		{/if}

		{#if staleTag}
			<Tag {...staleTag} pill />
		{/if}
	</div>
{:else if variant === 'inline'}
	<!-- Inline header: inside container top padding -->
	<div
		bind:this={inlineContainerEl}
		class="nopan nodrag absolute top-2 right-2 left-2 flex items-center gap-1 overflow-hidden rounded-t px-2 py-0.5 text-secondary"
	>
		{#if isCollapsible}
			<span data-fixed class="flex items-center"><span class="caret text-secondary"></span></span>
		{/if}
		{#if deviceType}
			<span data-fixed><DeviceGlyph {deviceType} /></span>
		{/if}
		{#if logoDefinition}
			<span data-fixed><SoftwareGlyph definition={logoDefinition} framed={false} /></span>
		{/if}
		{#if headerText}
			<span
				data-fixed
				data-entity-header
				class="min-w-0 truncate text-xs font-medium text-tertiary"
				use:tooltipAction={{ content: tooltip }}
			>
				{headerText}{groupLabels.length > 0 ? ':' : ''}
			</span>
		{/if}
		{#if staleTag}
			<span data-fixed class="shrink-0"><Tag {...staleTag} pill /></span>
		{/if}
		{#each visibleLabels as pill (pill.label)}
			<Tag label={pill.label} color={pill.color} title={pill.title ?? ''} />
		{/each}
		{#if hiddenLabelCount > 0}
			<HiddenTagsChip tags={groupLabels.slice(visibleLabelCount)} interactive />
		{/if}
	</div>
{:else if variant === 'collapsed-sub'}
	<!-- Collapsed subcontainer: compact inline with dashed border -->
	<div
		bind:this={inlineContainerEl}
		class="nopan nodrag flex cursor-pointer items-center gap-1 overflow-hidden rounded-lg border border-dashed border-gray-300 px-3 py-2 dark:border-gray-600"
		style="background: var(--color-topology-subgroup-bg); transition: box-shadow 0.15s ease-in-out; {searchHighlightRingStyle}"
		role="button"
		tabindex={-1}
		onclick={onToggleCollapse}
		onkeydown={(e) => {
			if (e.key === 'Enter' || e.key === ' ') onToggleCollapse(e);
		}}
	>
		{#if isCollapsible}
			<span data-fixed class="flex items-center"
				><span class="caret caret-collapsed text-secondary"></span></span
			>
		{/if}
		{#if deviceType}
			<span data-fixed><DeviceGlyph {deviceType} /></span>
		{:else if iconComponent}
			{@const IconComp = iconComponent}
			<span data-fixed
				><IconComp
					class={`h-3.5 w-3.5 shrink-0 ${colorHelper.icon}`}
					fill={fillIcon ? 'currentColor' : 'none'}
				/></span
			>
		{/if}
		{#if logoDefinition}
			<span data-fixed><SoftwareGlyph definition={logoDefinition} framed={false} /></span>
		{/if}
		{#if headerText}
			<span
				data-fixed
				data-entity-header
				class="text-xs font-medium whitespace-nowrap text-tertiary"
				use:tooltipAction={{ content: tooltip }}
			>
				{headerText}{groupLabels.length > 0 ? ':' : ''}
			</span>
		{/if}
		{#each visibleLabels.slice(0, 2) as pill (pill.label)}
			<Tag label={pill.label} color={pill.color} title={pill.title ?? ''} />
		{/each}
		{#if visibleLabels.length > 2}
			<HiddenTagsChip tags={groupLabels.slice(2)} interactive />
		{:else if hiddenLabelCount > 0}
			<HiddenTagsChip tags={groupLabels.slice(visibleLabelCount)} interactive />
		{/if}
		{#if searchMatchCount > 0}
			<span
				data-fixed
				class="ml-auto flex items-center gap-0.5 rounded-full bg-blue-500/20 px-1.5 py-0.5 text-[10px] leading-none whitespace-nowrap text-blue-400"
			>
				<Search class="h-2.5 w-2.5" />
				{searchMatchCount}
			</span>
		{/if}
		{#if !hideCount}
			<span
				data-fixed
				class="text-xs whitespace-nowrap text-tertiary"
				class:ml-auto={searchMatchCount === 0}
			>
				{#if countOnly}
					({childCount})
				{:else}
					({topology_elementCount({ summary: childSummary })})
				{/if}
			</span>
		{/if}
	</div>
{:else if variant === 'collapsed-root'}
	<!-- Collapsed root container: summary with subcontainer info -->
	<div
		class="rounded-xl text-center text-sm font-semibold shadow-lg"
		style="background: var(--color-topology-node-bg); position: relative; overflow: visible; transition: box-shadow 0.15s ease-in-out; border-top: 2px solid {colorHelper.rgb}; {tagHoverRingStyle}"
	>
		<div class="flex min-w-fit flex-col items-center gap-2 px-6 py-4 whitespace-nowrap">
			<!-- The container's name is not repeated here: a collapsed root container is named by
			     its external title pill, which stays visible above the box in both states. -->
			<span
				class="text-base font-medium text-secondary underline"
				use:tooltipAction={{ content: tooltip }}
			>
				{topology_elementCount({ summary: childSummary })}
			</span>
			{#if searchMatchCount > 0}
				<span
					class="flex items-center gap-1 rounded-full bg-blue-500/20 px-2 py-0.5 text-xs text-blue-400"
				>
					<Search class="h-3 w-3" />
					{topology_searchContainerMatches({ count: String(searchMatchCount) })}
				</span>
			{/if}
			{#if ungroupedCount > 0 && subgroupSummaries.length > 0}
				<span class="text-xs text-tertiary">
					{topology_ungroupedCount({ summary: ungroupedSummary })}
				</span>
			{/if}
			{#each subgroupSummaries as summary, i (i)}
				<div
					class="flex items-center gap-1 rounded-md border border-dashed border-gray-300 px-2 py-1 whitespace-nowrap dark:border-gray-600"
					style="background: var(--color-topology-subgroup-bg);"
				>
					{#if summary.logoDefinition}
						<SoftwareGlyph definition={summary.logoDefinition} framed={false} />
					{/if}
					{#if summary.headerText}
						<span class="text-xs text-tertiary"
							>{summary.headerText}{summary.labels.length > 0 ? ':' : ''}</span
						>
					{/if}
					{#each summary.labels.slice(0, 2) as pill, j (j)}
						<Tag label={pill.label} color={pill.color} title={pill.title ?? ''} />
					{/each}
					{#if summary.labels.length > 2}
						<HiddenTagsChip tags={summary.labels.slice(2)} interactive />
					{/if}
					{#if !summary.hideCount}
						<span class="text-xs text-tertiary">
							{#if summary.countOnly}
								({summary.childCount})
							{:else}
								({topology_elementCount({ summary: summary.childSummary })})
							{/if}
						</span>
					{/if}
				</div>
			{/each}
		</div>
	</div>
{/if}

<style>
	/*
	 * Collapse caret drawn in CSS rather than as an icon component.
	 *
	 * `ChevronDown`/`ChevronRight` each render an `<svg>` plus a `<path>` — two elements on every
	 * container header, 1,152 of them on the seeded graph, for a shape that is two borders. Blink
	 * accounts for a DOM node and its style and layout objects whether or not anything reads them,
	 * and at this node count that is the cheapest whole category to remove.
	 */
	.caret {
		display: inline-block;
		flex-shrink: 0;
		width: 0.35rem;
		height: 0.35rem;
		border-right: 1.5px solid currentColor;
		border-bottom: 1.5px solid currentColor;
		/*
		 * Rotated +45° the two borders draw a "v" that fills only the lower half of the box
		 * (its point sits ~0.35 of the box's height below centre), so it is lifted by that much
		 * to sit on the text's centre line. Rotated -45° the ">" spans the full height and is
		 * already centred.
		 */
		transform: translateY(-35%) rotate(45deg);
		transition: transform 150ms ease-in-out;
	}
	.caret-collapsed {
		transform: translateY(0) rotate(-45deg);
	}
</style>
