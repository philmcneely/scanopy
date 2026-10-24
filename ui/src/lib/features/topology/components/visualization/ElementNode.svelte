<script lang="ts">
	import { type NodeProps, useInternalNode } from '@xyflow/svelte';
	import NodeHandles from './NodeHandles.svelte';
	import { entities } from '$lib/shared/stores/metadata';
	import DeviceGlyph from '$lib/features/hosts/components/DeviceGlyph.svelte';
	import SoftwareGlyph from './SoftwareGlyph.svelte';
	import PortGlyph from './PortGlyph.svelte';
	import type { EntityGlyph } from '../../entity-glyph';
	import {
		selectedEdge as globalSelectedEdge,
		selectedNode as globalSelectedNode,
		topologyOptions,
		activeView
	} from '../../queries';
	import { useTopology, selectedTopologyId } from '../../context';
	import Tag from '$lib/shared/components/data/Tag.svelte';
	import { useSitesQuery } from '$lib/features/sites/queries';

	const sitesQuery = useSitesQuery();
	import type { TopologyNode, ElementRenderData, RenderableTopology } from '../../types/base';
	import { cardEntityForFilter, resolveTagTarget } from '../../resolvers';
	import { buildElementRender } from '../../element-render-data';
	import { getTopologyIndex } from '../../entity-index';
	import type { Writable } from 'svelte/store';
	import {
		matchesHoveredMetadata,
		inlineHostsMatching,
		UNTAGGED_SENTINEL
	} from '../../interactions';
	import { toggleInlineGroup } from '../../collapse';
	import * as sharedStores from '../../reactive-stores.svelte';
	import { createColorHelper } from '$lib/shared/utils/styling';
	import { getContext } from 'svelte';
	import type { Node, Edge } from '@xyflow/svelte';
	import { common_collapse, common_expand } from '$lib/paraglide/messages';
	import { tooltip } from '$lib/shared/actions/tooltip';
	import { hostOsIcon } from '$lib/features/hosts/host-os';
	import { ChevronDown, ChevronRight } from 'lucide-svelte';
	import { ELEMENT_HANDLE_SIZE_PX, ICON_ELEMENT_WIDTH_PX } from '../../pipeline/build-flow-nodes';

	// A service badge's size, and the icon card's width inside its px-2 padding: the badge fan
	// spreads within it.
	const BADGE_PX = 24;
	// The large device tile. The top service badge is centred on its bottom-right corner.
	const TILE_PX = 48;
	const CARD_INNER_PX = ICON_ELEMENT_WIDTH_PX - 16;
	import { stateFill as markedStateFill, type MarkValue } from '../../element-marks';

	let { id, data, width }: NodeProps = $props();

	// Shared, refcounted views over the module-level stores — see
	// `reactive-stores.svelte.ts`. One subscription serves every node component,
	// and it tears down with the component instead of leaking on every unmount.
	// `.current` falls back to `get(store)` outside a tracking context, so the
	// first value can't be missed — which is what the previous hand-rolled
	// subscriptions existed to work around.
	let isExportingValue = $derived(sharedStores.exporting.current);

	/**
	 * Draw as a box rather than as contents — decided once in the viewer and shared.
	 *
	 * See `detailSimplified`: the decision needs the size of the whole graph, which a card cannot
	 * see, and one refcounted subscription is cheaper than a viewport read per node.
	 */
	let simplified = $derived(sharedStores.simplified.current);

	/**
	 * The height a simplified card has to hold, taken from what SvelteFlow measured.
	 *
	 * Not the `height` prop: `build-flow-nodes.ts:247` sets it to `undefined` for every element on
	 * purpose, because `NodeWrapper` only applies a height style when it is present and an element
	 * card is meant to size to its content. That leaves the card itself as the height source — so an
	 * empty one collapses, ELK's next input shrinks, and the whole graph reflows on zoom. Measured
	 * at 5,936 nodes: the fitted zoom moved 0.0108 -> 0.0135 before this pin existed, which is the
	 * layout shrinking by a quarter.
	 *
	 * Pinning to `measured.height` is stable rather than circular: the card renders at exactly the
	 * height SvelteFlow already recorded, so `updateNodeInternals` sees no drift.
	 */
	let pinnedHeight = $derived(useInternalNode(id).current?.measured?.height);
	let hiddenEntities = $derived(sharedStores.hiddenEntities.current);
	let expandedInlineGroupKeys = $derived(sharedStores.expandedInlineGroupKeys.current);
	let searchHiddenNodes = $derived(sharedStores.searchHiddenNodes.current);
	let connectedNodes = $derived(sharedStores.connectedNodes.current);
	let edgeHandles = $derived(sharedStores.edgeHandles.current);
	let highlightedNewNodes = $derived(sharedStores.highlightedNewNodes.current);
	let multiSelectedNodes = $derived(sharedStores.multiSelectedNodes.current);
	let currentHoveredTag = $derived(sharedStores.currentHoveredTag.current);
	let currentHoveredMetadata = $derived(sharedStores.currentHoveredMetadata.current);

	const topo = useTopology();
	const topoStore = topo.fromContext ? topo.store : null;
	let topology = $derived(
		topoStore
			? $topoStore
			: (topo.query?.data?.find((t) => t.id === $selectedTopologyId) as
					RenderableTopology | undefined)
	);

	// Try to get selection from context (for share/embed pages), fallback to global store
	const selectedNodeContext = getContext<Writable<Node | null> | undefined>('selectedNode');
	const selectedEdgeContext = getContext<Writable<Edge | null> | undefined>('selectedEdge');
	let selectedNode = $derived(
		selectedNodeContext ? $selectedNodeContext : $globalSelectedNode
	) as Node | null;
	let selectedEdge = $derived(
		selectedEdgeContext ? $selectedEdgeContext : $globalSelectedEdge
	) as Edge | null;

	// Sites are still needed locally: the Staleness filter value reads the site window.
	let sitesData = $derived(sitesQuery.data ?? []);
	const siteFor = (entity: { site_id?: string } | undefined | null) =>
		sitesData.find((n) => n.id === entity?.site_id);

	let effectiveWidth = $derived(width ? width : 0);

	// All card content — and therefore card height — is computed by one function
	// in `element-render-data.ts`, shared with the render pipeline so it can reason
	// about card shape without mounting the card. Do not recompute any of this
	// locally; add to that function instead.
	let elementRender = $derived(
		topology
			? buildElementRender({
					nodeId: id,
					node: data as TopologyNode,
					topology,
					activeView: $activeView,
					options: $topologyOptions,
					hiddenEntityIds: hiddenEntities,
					expandedInlineGroups: expandedInlineGroupKeys,
					sites: sitesData
				})
			: null
	);
	let nodeRenderData: ElementRenderData | null = $derived(elementRender?.data ?? null);
	let inlineForThisElement = $derived(elementRender?.flags.inlineEntities ?? []);
	// Staleness pill — computed with the rest of the card content because it
	// renders in flow and therefore affects card height.
	let staleTag = $derived(elementRender?.staleTag ?? null);
	// The icon card: a device, software or port glyph above a label.
	let iconCard = $derived(elementRender?.iconCard ?? null);
	/**
	 * The fill a reduced card paints itself, from the one state signal it has.
	 *
	 * An element is never `hidden` or `labelled` — it has no name worth showing at this size and it
	 * is the graph's texture, so it always keeps its box. What varies is the colour.
	 */
	let marks = $derived(elementRender?.marks ?? {});
	let stateFill = $derived(markedStateFill(marks));

	let isNewNode = $derived(nodeRenderData ? highlightedNewNodes.has(id) : false);

	let isNodeSelected = $derived(
		selectedNode?.id === nodeRenderData?.ip_address_id ||
			multiSelectedNodes.some((n) => n.id === nodeRenderData?.ip_address_id)
	);

	// Fade signals "focus elsewhere" (search, selection) — not filter state.
	// Filter hides (tag / metadata / entity-hide) remove nodes structurally
	// upstream of this component, so a rendered card is never filter-hidden.
	let shouldFadeOut = $derived.by(() => {
		if (isExportingValue) return false;

		// Search highlight: fade non-matching nodes.
		if (searchHiddenNodes.has(id)) {
			return true;
		}

		// Selection focus: fade unconnected nodes.
		if (!selectedNode && !selectedEdge && multiSelectedNodes.length < 2) return false;
		if (!nodeRenderData) return false;
		return !connectedNodes.has(id);
	});

	let nodeOpacity = $derived(shouldFadeOut ? 0.3 : 1);

	const discoveryColorHelper = entities.getColorHelper('Discovery');

	// How does the hovered entity type relate to this card?
	//   'element' — the card IS the hovered entity type (Service element in
	//     Workloads/Application; IPAddress element in L3; Host VM in
	//     Workloads; Interface in L2).
	//   'inline'  — the hovered entity type is declared inline on this card
	//     (Service on an L3 IPAddress or a Workloads Host VM element).
	//   null      — no relationship; this card is unaffected by the hover.
	// Element cards get a card-border treatment; inline cards get a card
	// glow. This replaces the former Host-specific branch and makes hover
	// behaviour view-agnostic.
	let hoveredRelationship = $derived.by((): 'element' | 'inline' | null => {
		if (!currentHoveredTag) return null;
		const elType = nodeRenderData?.elementType;
		if (elType && currentHoveredTag.entityType === elType) return 'element';
		if (inlineForThisElement.includes(currentHoveredTag.entityType)) return 'inline';
		return null;
	});

	// Tags carried by this card's taggable entity (for tag-scoped ring): its own,
	// or its host's for the untaggable IPAddress and Interface cards.
	function cardEntityTags(): string[] {
		if (!topology) return [];
		const target = resolveTagTarget(data as TopologyNode);
		if (!target) return [];
		return getTopologyIndex(topology).entityTags.get(target.entityId) ?? [];
	}

	// Member hosts in this card's manager boxes that carry the hovered Host filter value.
	let matchingInlineHosts = $derived(
		inlineHostsMatching(nodeRenderData?.inlineGroups ?? [], currentHoveredMetadata, siteFor)
	);

	// Metadata hover context — mirrors `hoveredRelationship` + tag ring/pulse
	// but driven by `hoveredMetadata`. Element-mode when this card's own
	// entity carries the hovered value, inline-mode when an inline entity matches.
	// Host metadata also bubbles up to IPAddress/Interface cards (same
	// fallback as cardEntityTags).
	let metadataHoverContext = $derived.by(
		(): {
			mode: 'element' | 'inline';
			color: string;
		} | null => {
			if (!currentHoveredMetadata || !topology) return null;
			const hovered = currentHoveredMetadata;
			const { color } = hovered;

			// The card's own entity for each entity the hovered filter covers — any element type,
			// through the same `cardEntityForFilter` the view's element marks read.
			for (const entityType of hovered.entityTypes) {
				const cardEntity = cardEntityForFilter(data as TopologyNode, topology, entityType);
				if (
					cardEntity &&
					matchesHoveredMetadata(cardEntity, entityType, hovered, siteFor(cardEntity))
				) {
					return { mode: 'element', color };
				}
			}

			if (matchingInlineHosts.size > 0) return { mode: 'inline', color };

			for (const service of nodeRenderData?.services ?? []) {
				if (matchesHoveredMetadata(service, 'Service', hovered, siteFor(service)))
					return { mode: 'inline', color };
			}
			return null;
		}
	);

	// Card border for element-role hover. Subdued gray when entity-wide
	// (tagId null); tag-coloured when tag-scoped AND the card's entity
	// actually carries the hovered tag. Metadata-scoped hover uses the
	// same border treatment when this card's own entity matches.
	let tagHoverRingStyle = $derived.by(() => {
		if (metadataHoverContext?.mode === 'element') {
			const ch = createColorHelper(
				metadataHoverContext.color as Parameters<typeof createColorHelper>[0]
			);
			return `box-shadow: 0 0 0 3px ${ch.rgb};`;
		}
		if (hoveredRelationship !== 'element' || !currentHoveredTag) return '';
		const { tagId, color } = currentHoveredTag;
		if (tagId === null) {
			return 'box-shadow: 0 0 0 2px rgb(156, 163, 175);';
		}
		const tags = cardEntityTags();
		const hasTag = tagId === UNTAGGED_SENTINEL ? tags.length === 0 : tags.includes(tagId);
		if (!hasTag || !color) return '';
		const colorHelper = createColorHelper(color as Parameters<typeof createColorHelper>[0]);
		return `box-shadow: 0 0 0 3px ${colorHelper.rgb};`;
	});

	// Card glow for tag-scoped inline hover (and legacy category hover).
	// Entity-wide inline hover leaves the card alone: every card that inlines
	// the hovered entity would glow otherwise, which isn't useful
	// discrimination.
	let serviceHoverShadowStyle = $derived.by(() => {
		if (!nodeRenderData?.showServices) return '';
		const services = nodeRenderData.services;
		if (
			hoveredRelationship === 'inline' &&
			currentHoveredTag &&
			currentHoveredTag.tagId !== null &&
			currentHoveredTag.color
		) {
			const { tagId, color, entityType } = currentHoveredTag;
			// Only fire for entity types that carry tags today (Service). New
			// taggable inline entities would extend via the same per-row tag
			// lookup. For now, no generic registry to resolve "tags of an
			// arbitrary inline entity instance on this card" exists.
			if (entityType === 'Service') {
				for (const service of services) {
					const isUntagged = service.tags.length === 0;
					const hasTag = tagId === UNTAGGED_SENTINEL ? isUntagged : service.tags.includes(tagId);
					if (hasTag) {
						const ch = createColorHelper(color as Parameters<typeof createColorHelper>[0]);
						return `--pulse-color: ${ch.rgb};`;
					}
				}
			}
		}
		if (metadataHoverContext?.mode === 'inline') {
			const ch = createColorHelper(
				metadataHoverContext.color as Parameters<typeof createColorHelper>[0]
			);
			return `--pulse-color: ${ch.rgb};`;
		}
		return '';
	});

	// Entity-wide hover matching THIS element's own type — drives the
	// dotted-underline on the header text. Inline-role hover has its own
	// visual treatment (card glow) and doesn't underline the header.
	let isEntityTypeHover = $derived(
		hoveredRelationship === 'element' && currentHoveredTag?.tagId === null
	);

	let cardClass = $derived(`card ${isNodeSelected ? 'card-selected' : ''}`);

	// Handle styling lived here; `NodeHandles` now renders the geometry statically.
</script>

<!-- One row's three cells, for a grid of a 20px glyph, the name and a trailing status dot (in the
     colour of the view's Status mark, when it paints one): a member of a manager box. -->
{#snippet rowCells(glyph: EntityGlyph, label: string, status: MarkValue | undefined)}
	<span class="flex">
		{#if glyph.type === 'device'}
			<DeviceGlyph deviceType={glyph.deviceType} />
		{:else if glyph.type === 'software'}
			<SoftwareGlyph definition={glyph.definition} />
		{:else}
			<PortGlyph />
		{/if}
	</span>
	<span class="truncate text-[11px] leading-tight font-medium text-secondary">
		{label}
	</span>
	<span class="flex justify-center">
		{#if status}
			<span class="h-2 w-2 rounded-full" style="background: {createColorHelper(status.color).rgb};"
			></span>
		{/if}
	</span>
{/snippet}

{#if nodeRenderData}
	<div
		class={`${cardClass} ${isNewNode ? 'animate-pulse-highlight' : ''} ${serviceHoverShadowStyle ? 'animate-pulse-highlight-once' : ''} ${isEntityTypeHover ? 'entity-type-hover-active' : ''}`}
		style={`width: ${effectiveWidth}px; height: ${simplified && pinnedHeight !== undefined ? `${pinnedHeight}px` : '100%'}; ${simplified ? `background: ${stateFill};` : ''} display: flex; flex-direction: column; padding: 0; opacity: ${nodeOpacity}; transition: opacity 0.2s ease-in-out, box-shadow 0.15s ease-in-out; ${isNewNode ? `--pulse-color: ${discoveryColorHelper.rgb};` : ''} ${serviceHoverShadowStyle} ${tagHoverRingStyle}`}
	>
		<!--
			Below the zoom threshold the card is its own box and nothing else — see `shouldSimplify`.
			The height is pinned to the node's own so the box the layout was built around is
			unchanged; without that the empty card collapses and ELK's next input shifts, which
			would make the graph reflow as you zoom.
		-->
		{#if !simplified && iconCard}
			<!-- Icon card. The glyph comes first in DOM order: the diagram exports take each
			     node's first icon as its shape icon. -->
			<div class="group/icon-card flex flex-col items-center gap-1.5 px-2 py-2">
				<!-- The card's tooltip sits on the tile and label only: a group member below carries its
				     own, and nested tooltips would show both at once. It shows above the whole card, so
				     it never covers the status indicator. -->
				<div
					class="flex w-full flex-col items-center gap-1.5"
					use:tooltip={{ anchorClosest: '.svelte-flow__node', content: iconCard.tooltip }}
				>
					<!-- my-1.5: the service badges (bottom-right), OS badge (bottom-left) and status dot (top-left) are
					     centred on the tile's corners and overhang it by half their size, more than the card's
					     gap and padding. On every icon card, so heights match. -->
					<div class="relative my-1.5">
						{#if iconCard.glyph.type === 'device'}
							<DeviceGlyph deviceType={iconCard.glyph.deviceType} size="lg" />
						{:else if iconCard.glyph.type === 'software'}
							<SoftwareGlyph definition={iconCard.glyph.definition} size="lg" />
						{:else}
							<PortGlyph size="lg" />
						{/if}
						{#if marks.Status}
							<!-- A guest's run state or a port's link state, from the view's Status mark: a dot
							     in the state's colour, centred on the tile's top-left corner. The tooltip's
							     Status row names it. -->
							<span
								class="absolute -top-2 -left-2 z-10 flex h-4 w-4 items-center justify-center rounded-full border border-gray-300 dark:border-gray-600"
								style="background: var(--color-topology-node-bg);"
							>
								<span
									class="h-2.5 w-2.5 rounded-full"
									style="background: {createColorHelper(marks.Status.color).rgb};"
								></span>
							</span>
						{/if}
						{#if iconCard.os && iconCard.glyph.type === 'device'}
							<!-- The host's OS, centred on the tile's bottom-left corner, the size of a service
							     badge. Fanned service badges spread under the whole tile, so it fades out while
							     they do; a lone service badge stays on its corner and leaves it in place. The
							     tooltip's Operating system row names it. -->
							{@const OsLogo = hostOsIcon(iconCard.os.family)}
							<span
								class="absolute -bottom-3 -left-3 flex h-6 w-6 items-center justify-center overflow-hidden rounded-full border border-gray-300 transition-opacity duration-150 dark:border-gray-600 {iconCard
									.badgeDefinitions.length > 1
									? 'group-hover/icon-card:opacity-0'
									: ''}"
								style="background: var(--color-topology-node-bg);"
							>
								<OsLogo size={15} class="text-primary" />
							</span>
						{/if}
						{#if iconCard.badgeDefinitions.length > 0}
							<!-- Round like every software glyph: this device, running these. The top badge is
							     centred on the tile's bottom-right corner; each one behind it steps up and left,
							     two layers at most, so more than one service reads as a stack. Hovering the
							     card fans them into a row centred under the tile, spaced to fit inside the
							     card. Offsets are from the tile's centre to each badge's left edge. -->
							{@const count = iconCard.badgeDefinitions.length}
							{@const pileX = TILE_PX / 2 - BADGE_PX / 2}
							<!-- A lone badge has nothing to fan out from: it stays on the corner on hover. -->
							{@const fanStep =
								count > 1 ? Math.min(BADGE_PX + 2, (CARD_INNER_PX - BADGE_PX) / (count - 1)) : 0}
							<!-- Every badge is a flex box, off the text baseline: an <img> logo and an <svg> glyph sit on a
							     baseline at different heights, which bent the fanned row into a descending curve. -->
							<span class="absolute -bottom-3 left-1/2 flex h-6 w-0">
								{#each iconCard.badgeDefinitions as definition, i (definition)}
									{@const layer = Math.min(i, 2) * 3}
									<span
										class="absolute top-(--stack-y) left-(--stack-x) flex leading-none transition-[left,top] duration-150 group-hover/icon-card:top-0 group-hover/icon-card:left-(--fan-x)"
										style="--stack-x: {pileX - layer}px; --stack-y: {-layer}px; --fan-x: {count > 1
											? (i - (count - 1) / 2) * fanStep - BADGE_PX / 2
											: pileX}px; z-index: {count - i};"
									>
										<SoftwareGlyph {definition} size="md" />
									</span>
								{/each}
							</span>
						{/if}
					</div>
					{#if staleTag}
						<div class="scale-90">
							<Tag {...staleTag} pill />
						</div>
					{/if}
					<div
						data-entity-header
						class="line-clamp-2 w-full text-center text-xs leading-tight font-medium wrap-break-word text-primary"
					>
						{iconCard.label}
					</div>
					{#if iconCard.sublabel}
						<div
							class="-mt-1 w-full truncate text-center text-xs leading-tight text-tertiary"
							title={iconCard.sublabel}
						>
							{iconCard.sublabel}
						</div>
					{/if}
				</div>
				<!-- Manager groups inlined on a guest, each in a dashed box. The heading and every member
				     row share three columns: a 20px glyph, the name, and a trailing slot that holds the
				     heading's member count and each row's run-state dot, so glyphs, names and dots line
				     up. The heading opens and closes the box; each row carries its own tooltip. -->
				{#each iconCard.groups as group (group.groupId)}
					<div
						class="grid w-full grid-cols-[1.25rem_minmax(0,1fr)_0.75rem] items-center gap-x-1.5 gap-y-1.5 rounded-md border border-dashed border-gray-300 p-1.5 dark:border-gray-600"
					>
						<button
							type="button"
							class="nopan col-span-3 grid cursor-pointer grid-cols-subgrid items-center text-left"
							aria-expanded={!group.collapsed}
							aria-label={group.collapsed ? common_expand() : common_collapse()}
							onclick={(e) => {
								e.stopPropagation();
								toggleInlineGroup(id, group.groupId);
							}}
						>
							<span class="flex justify-center">
								{#if group.headerDefinition}
									<SoftwareGlyph definition={group.headerDefinition} framed={false} />
								{/if}
							</span>
							<span
								class="flex min-w-0 items-center gap-0.5 text-[11px] leading-tight font-medium text-tertiary"
							>
								{#if group.collapsed}
									<ChevronRight class="h-3 w-3 shrink-0" />
								{:else}
									<ChevronDown class="h-3 w-3 shrink-0" />
								{/if}
								<span class="truncate">{group.headerName}</span>
							</span>
							<span class="text-center text-[10px] leading-none text-tertiary tabular-nums">
								{group.members.length}
							</span>
						</button>
						{#if !group.collapsed}
							{#each group.members as member (member.id)}
								<!-- A subgrid row, not `contents`: the tooltip places itself from the row's box. -->
								<div
									class="col-span-3 grid grid-cols-subgrid items-center"
									use:tooltip={{ content: member.tooltip }}
								>
									{@render rowCells(member.glyph, member.label, member.status)}
								</div>
							{/each}
						{/if}
					</div>
				{/each}
			</div>
		{:else if !simplified}
			<!-- An element that did not resolve to an icon card (a service whose record is gone): its
			     staleness, header and a line naming what is missing. -->
			{#if staleTag}
				<div class="flex shrink-0 justify-center px-2 pt-2">
					<div class="scale-90">
						<Tag {...staleTag} pill />
					</div>
				</div>
			{/if}
			{#if nodeRenderData.headerText}
				<div
					data-entity-header
					class="truncate px-2 pt-2 text-center text-xs leading-none font-medium text-tertiary"
				>
					{nodeRenderData.headerText}
				</div>
			{/if}
			{#if nodeRenderData.bodyText}
				<div
					class="truncate px-3 py-2 text-center text-xs leading-none text-secondary"
					title={nodeRenderData.bodyText}
				>
					{nodeRenderData.bodyText}
				</div>
			{/if}
		{/if}
	</div>
{/if}

<!-- Only the handles an edge on this node actually names; see `edgeHandlesByNode`. -->
{#if edgeHandles.get(id)}
	<NodeHandles size={ELEMENT_HANDLE_SIZE_PX} used={edgeHandles.get(id)!} />
{/if}
