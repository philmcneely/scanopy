/**
 * Single owner of what an element card renders — and therefore of how tall it is.
 *
 * This computation used to live inside `ElementNode.svelte` as a `$derived.by`.
 * It never depended on component-local state: every input is either the node
 * itself or a module-level store, so it was always a pure function that merely
 * happened to live in a component.
 *
 * Moving it out matters because the render pipeline needs to reason about card
 * height *without* rendering the card. The measure pass currently mounts every
 * node just to read its height; to measure only one representative per distinct
 * card shape, something has to decide which cards share a shape. If that
 * decision were made by a second implementation, the two would drift and the
 * layout would silently corrupt. With one function, the shape key is derived
 * from the same result the component renders.
 *
 * Consequently: **anything that affects rendered height must be reflected in
 * `ElementRenderResult`.** Height-affecting markup driven by something outside
 * this function's inputs will not be visible to the shape key.
 */

import type { components } from '$lib/api/schema';
import type { Service } from '$lib/features/services/types/base';
import type {
	ElementInlineGroup,
	ElementRenderData,
	RenderableTopology,
	TopologyNode,
	TopologyOptions
} from './types/base';
import { elementEntity, nodeEntity, resolveElementNode } from './resolvers';
import { entityGlyph, type EntityGlyph } from './entity-glyph';
import { elementMarks, entityMark, type ElementMarks, type MarkValue } from './element-marks';
import type { Host, HostVirtualization } from '$lib/features/hosts/types/base';
import type { ServiceVirtualization } from '$lib/features/services/types/base';
import { hostDisplayName } from '$lib/features/hosts/host-display-name';
import { getTopologyIndex } from './entity-index';
import {
	containerNetworkTypes,
	deviceTypes,
	entities,
	guestStates,
	hostVirtualizations,
	proxmoxGuestTypes,
	serviceDefinitions,
	serviceVirtualizations,
	views
} from '$lib/shared/stores/metadata';
import { getFreshnessTag } from '$lib/shared/utils/freshness';
import { createColorHelper } from '$lib/shared/utils/styling';
import type { TooltipContent, TooltipRow, TooltipSection } from '$lib/shared/actions/tooltip';
import { formatInterfaceSpeed } from '$lib/shared/utils/formatting';
import type { Site } from '$lib/features/sites/types';
import { get } from 'svelte/store';
import { activeView, topologyOptions } from './queries';
import { hiddenEntityIds } from './interactions';
import { expandedInlineGroups, inlineGroupKey } from './collapse';
import { queryClient, queryKeys } from '$lib/api/query-client';
import {
	common_containers,
	common_host,
	common_ipAddress,
	common_manager,
	common_services,
	common_status,
	hosts_deviceFacts_composeProject,
	hosts_deviceFacts_guestType,
	hosts_deviceFacts_networkType,
	hosts_deviceType
} from '$lib/paraglide/messages';

type InlineGroup = components['schemas']['InlineGroup'];

/**
 * Whether the active view inlines services / ports on this element, and whether
 * the user has hidden either. These gate whole blocks of card content, so they
 * belong with the render data rather than being recomputed by the template.
 */
export interface ElementInlineFlags {
	/** Entity types the active view inlines on this element (e.g. Service, Port). */
	inlineEntities: string[];
	inlinesService: boolean;
	inlinesPort: boolean;
	serviceInlineHidden: boolean;
	portInlineHidden: boolean;
}

export interface ElementRenderResult {
	data: ElementRenderData | null;
	flags: ElementInlineFlags;
	/**
	 * Staleness pill, or null. Rendered in flow at the top of the card, so its
	 * presence changes the card's height — which is why it belongs here and not
	 * in the component.
	 */
	staleTag: ReturnType<typeof getFreshnessTag>;
	/** The colours the view paints on this card, each decoded by one of its filter values. */
	marks: ElementMarks;
	/**
	 * What the element is, drawn in both view modes: a device's device-type tile or software's logo, by
	 * its entity type's `glyph` metadata. Null for an element type that has none (a port).
	 */
	glyph: EntityGlyph | null;
	/**
	 * The icon card, when the view draws in the `Icons` view mode and the element has a glyph.
	 * Replaces every row of `data` on the card, so it decides the card's height in that mode.
	 */
	iconCard: ElementIconCard | null;
}

/**
 * An icon card: the element's glyph, large, with a short label and the rest of the card's text on
 * hover.
 */
export interface ElementIconCard {
	glyph: EntityGlyph;
	/**
	 * The services running on what the card stands for, as definitions for a stack of round badges
	 * on a device glyph: one per definition, the one that decided the device type first, then those
	 * with a logo. At most `BADGE_STACK_MAX`; the tooltip lists every service.
	 */
	badgeDefinitions: string[];
	label: string;
	/** Hover content: the card's name, labelled rows for what the card no longer shows, then its services. */
	tooltip: TooltipContent;
	/**
	 * The manager groups inlined on a guest's card (a Docker engine on a VM with its containers),
	 * each drawn under the label in a dashed box with a row per member.
	 */
	groups: IconCardGroup[];
}

/** A manager group on an icon card: its heading and its members. */
export interface IconCardGroup {
	groupId: string;
	/** The manager service's definition, drawn beside the heading; null when it is not on this card. */
	headerDefinition: string | null;
	headerName: string;
	members: IconCardMember[];
}

/** A container (or other managed member) drawn as a row inside an icon card's group. */
export interface IconCardMember {
	id: string;
	glyph: EntityGlyph;
	/** Its name, on the member's row beside the glyph. */
	label: string;
	/** Its run state, from the view's Status mark; absent when the view paints none on it. */
	status: MarkValue | undefined;
	/** Hover content, as the card's own. */
	tooltip: TooltipContent;
}

export interface ElementRenderInputs {
	nodeId: string;
	node: TopologyNode;
	topology: RenderableTopology;
	activeView: string;
	options: TopologyOptions;
	/** Ids of entities hidden by any filter, of any type (`hiddenEntityIds`). */
	hiddenEntityIds: Set<string>;
	/** Manager boxes that are open, keyed by `inlineGroupKey`; every other box is collapsed. */
	expandedInlineGroups: Set<string>;
	/** Sites, for resolving each entity's staleness window. */
	sites: Site[];
}

type ViewElementConfig = {
	element_config?: {
		container_entity?: string;
		element_entities?: Array<{ entity_type: string; inline_entities: string[] }>;
	};
} | null;

function viewConfigFor(activeView: string): ViewElementConfig {
	return views.getMetadata(activeView) as ViewElementConfig;
}

/** Service categories the user hid via the metadata filter, for this view. */
function hiddenServiceCategories(options: TopologyOptions, activeView: string): string[] {
	const byView = (options.request.hide_metadata_values ?? {}) as Record<
		string,
		Record<string, Record<string, string[]>>
	>;
	return byView[activeView]?.['Service']?.['Category'] ?? [];
}

export function elementInlineFlags(
	inputs: Pick<ElementRenderInputs, 'activeView' | 'options'>,
	elementType: string | undefined
): ElementInlineFlags {
	const inlineEntities =
		viewConfigFor(inputs.activeView)?.element_config?.element_entities?.find(
			(e) => e.entity_type === elementType
		)?.inline_entities ?? [];

	// Entity types the user has hidden in this view via the filter panel's eye
	// toggle. (Element/container-level hiding is applied upstream via
	// tagHiddenNodeIds.)
	const hiddenEntities =
		((inputs.options.request.hide_entities ?? {}) as Record<string, string[]>)[inputs.activeView] ??
		[];

	return {
		inlineEntities,
		inlinesService: inlineEntities.includes('Service'),
		inlinesPort: inlineEntities.includes('Port'),
		serviceInlineHidden: hiddenEntities.includes('Service'),
		portInlineHidden: hiddenEntities.includes('Port')
	};
}

/**
 * Staleness pill for the entity this card depicts.
 *
 * Judged on the node's own entity with no inheritance from its host — the same
 * rule the inventory cards apply, so the two surfaces agree and the tooltip's
 * timestamp can never contradict the tag. Type names come from the entity
 * metadata fixture so they stay localized and in step with the backend.
 */
function resolveStaleTag(
	node: TopologyNode,
	topology: RenderableTopology,
	resolved: ReturnType<typeof resolveElementNode>,
	sites: Site[]
): ReturnType<typeof getFreshnessTag> {
	const subject = elementEntity(node, topology);
	if (!subject) return null;
	return getFreshnessTag(
		subject,
		sites.find((n) => n.id === subject.site_id),
		{ entityTypeLabel: entities.getName(resolved.elementType ?? 'Host') || undefined }
	);
}

export function buildElementRender(inputs: ElementRenderInputs): ElementRenderResult {
	const resolved = resolveElementNode(inputs.nodeId, inputs.node, inputs.topology);
	const content = buildElementContent(inputs, resolved);
	const subject = nodeEntity(inputs.node, inputs.topology);
	const glyph = subject ? entityGlyph(subject.entityType, subject.entity, inputs.topology) : null;
	const marks = elementMarks(inputs.activeView, inputs.node, inputs.sites, inputs.topology);
	return {
		...content,
		marks,
		glyph,
		iconCard: glyph ? buildIconCard(inputs, resolved, content.data, glyph, marks) : null
	};
}

/** The most service badges a device glyph stacks: five fan across the card without overlapping. */
const BADGE_STACK_MAX = 5;
/** A tooltip row before empty values are dropped. */
type MaybeRow = { label: string; value: string | null | undefined; dot?: string };

/** The run state row, its dot in the colour the view's Status mark paints that state. */
function statusRow(state: string | null | undefined, status: MarkValue | undefined): MaybeRow {
	return {
		label: common_status(),
		value: state ? guestStates.getName(state) : null,
		dot: status ? createColorHelper(status.color).rgb : undefined
	};
}

/**
 * What a guest host's tooltip adds about how it runs: its manager, the modifiers that set it apart
 * from a plain guest (a Proxmox guest's type, a container's macvlan or ipvlan network, its Compose
 * project) and its run state. The same facts the host modal's virtualization section lists.
 */
function hostGuestRows(
	virtualization: HostVirtualization | null | undefined,
	status: MarkValue | undefined
): MaybeRow[] {
	if (!virtualization) return [];
	const manager: MaybeRow = {
		label: common_manager(),
		value: hostVirtualizations.getName(virtualization.type)
	};
	switch (virtualization.type) {
		case 'Proxmox': {
			const { guest_type, state } = virtualization.details;
			return [
				manager,
				{
					label: hosts_deviceFacts_guestType(),
					value: guest_type ? proxmoxGuestTypes.getName(guest_type) : null
				},
				statusRow(state, status)
			];
		}
		case 'VCenter':
		case 'ESXi':
			return [manager];
		case 'Docker':
		case 'Podman': {
			const { network_type, compose_project, state } = virtualization.details;
			return [
				manager,
				{
					label: hosts_deviceFacts_networkType(),
					value: containerNetworkTypes.getName(network_type)
				},
				{ label: hosts_deviceFacts_composeProject(), value: compose_project },
				statusRow(state, status)
			];
		}
		case 'NetworkIdentity':
			return [];
	}
}

/** A containerized service's rows: its runtime, Compose project and run state. */
function serviceGuestRows(
	virtualization: ServiceVirtualization | null | undefined,
	status: MarkValue | undefined
): MaybeRow[] {
	if (!virtualization) return [];
	const { compose_project, state } = virtualization.details;
	return [
		{ label: common_manager(), value: serviceVirtualizations.getName(virtualization.type) },
		{ label: hosts_deviceFacts_composeProject(), value: compose_project },
		statusRow(state, status)
	];
}

/** A titled tooltip: one row per value present, then the services under their own heading. */
function titledTooltip(title: string, rows: MaybeRow[], services: string[] = []): TooltipContent {
	const present: TooltipRow[] = rows.flatMap(({ label, value, dot }) =>
		value ? [{ label, value, dot }] : []
	);
	const sections: TooltipSection[] = [];
	if (present.length > 0) sections.push({ rows: present });
	if (services.length > 0) sections.push({ heading: common_services(), items: services });
	return { title, sections };
}

/**
 * A service's hover text: its name as the title, the host it runs on when the card does not
 * already say it, and how it runs when it is containerized. `status` is the view's Status mark on
 * the service, which colours its run state.
 */
export function serviceTooltip(
	service: Service,
	{ host, status }: { host?: Host | null; status?: MarkValue } = {}
): TooltipContent {
	return titledTooltip(service.name, [
		{ label: common_host(), value: host ? hostDisplayName(host) : null },
		...serviceGuestRows(service.virtualization_metadata, status)
	]);
}

/**
 * A host's hover text: its name as the title, then its address, device type and how it runs as a
 * guest, then its services. `status` is the view's Status mark on the host.
 */
export function hostTooltip(
	host: Host,
	{
		name,
		address,
		deviceType,
		services,
		status
	}: {
		name?: string;
		address?: string | null;
		deviceType?: string | null;
		services: Service[];
		status?: MarkValue;
	}
): TooltipContent {
	return titledTooltip(
		name ?? hostDisplayName(host),
		[
			{ label: common_ipAddress(), value: address },
			{ label: hosts_deviceType(), value: deviceType ? deviceTypes.getName(deviceType) : null },
			...hostGuestRows(host.virtualization_metadata, status)
		],
		services.map((s) => s.name)
	);
}

/**
 * The icon card for an element with a glyph, or null for one with nothing resolved, which keeps
 * the detailed card. Software is labelled with its name; a device with its address on an address
 * card, so two addresses on one host stay apart, and with its host's name otherwise.
 */
function buildIconCard(
	inputs: ElementRenderInputs,
	resolved: ReturnType<typeof resolveElementNode>,
	data: ElementRenderData | null,
	glyph: EntityGlyph,
	marks: ElementMarks
): ElementIconCard | null {
	if (!data) return null;
	const host = resolved.host;

	if (glyph.type === 'software') {
		const service = resolved.services[0];
		if (!service) return null;
		return {
			glyph,
			badgeDefinitions: [],
			label: service.name,
			tooltip: serviceTooltip(service, { host, status: marks.Status }),
			groups: []
		};
	}

	if (!host) return null;
	const deviceType = glyph.deviceType;
	// The badges are what runs on the thing the card stands for: the services bound to an address,
	// or a host's own services, the same list the detailed card draws. The service that decided
	// the device type leads, so the top badge agrees with the glyph; logos come before glyphs.
	const meta = (s: Service) =>
		serviceDefinitions.getMetadata(s.service_definition) as {
			device_type?: string | null;
			has_logo?: boolean;
		} | null;
	const rank = (s: Service) =>
		meta(s)?.device_type === deviceType ? 0 : meta(s)?.has_logo ? 1 : 2;
	const badgeDefinitions = [
		...new Set(
			[...data.services].sort((a, b) => rank(a) - rank(b)).map((s) => s.service_definition)
		)
	].slice(0, BADGE_STACK_MAX);
	const hostName = data.headerText ?? hostDisplayName(host);
	// The container subnet's placeholder address names nothing, so that card keeps the host name.
	const address = data.subtitleText ? resolved.ipAddress?.ip_address : null;
	return {
		glyph,
		badgeDefinitions,
		label: address ?? hostName,
		tooltip: hostTooltip(host, {
			name: hostName,
			address,
			deviceType,
			services: data.services,
			status: marks.Status
		}),
		groups: data.inlineGroups.map((g) => iconCardGroup(inputs, g))
	};
}

/**
 * One inlined manager group as an icon card draws it: every member as a one-line row, whatever the
 * detailed card's collapse state, since a row of glyph and name is already the collapsed form.
 */
function iconCardGroup(inputs: ElementRenderInputs, group: ElementInlineGroup): IconCardGroup {
	const { activeView, sites, topology } = inputs;
	const services: IconCardMember[] = group.services.map((service) => {
		const status = entityMark(activeView, 'Status', 'Service', service, sites);
		return {
			id: service.id,
			glyph: { type: 'software', definition: service.service_definition },
			label: service.name,
			status,
			tooltip: serviceTooltip(service, { status })
		};
	});
	const hosts: IconCardMember[] = group.hosts.map(({ host, services: hostServices }) => {
		const glyph: EntityGlyph = entityGlyph('Host', host, topology) ?? {
			type: 'device',
			deviceType: 'Unidentified'
		};
		const status = entityMark(activeView, 'Status', 'Host', host, sites);
		return {
			id: host.id,
			glyph,
			label: hostDisplayName(host),
			status,
			tooltip: hostTooltip(host, {
				deviceType: glyph.type === 'device' ? glyph.deviceType : null,
				services: hostServices,
				status
			})
		};
	});
	return {
		groupId: group.groupId,
		headerDefinition: group.header?.service_definition ?? null,
		// The runtime's type ("Docker"), not its service's name ("Docker Daemon"), which the narrow
		// card truncates.
		headerName: group.header
			? serviceDefinitions.getName(group.header.service_definition)
			: common_containers(),
		members: [...services, ...hosts]
	};
}

function buildElementContent(
	inputs: ElementRenderInputs,
	resolved: ReturnType<typeof resolveElementNode>
): Omit<ElementRenderResult, 'marks' | 'glyph' | 'iconCard'> {
	const { nodeId, node, topology, activeView, options, hiddenEntityIds } = inputs;

	const flags = elementInlineFlags(inputs, resolved.elementType);

	const elementType = resolved.elementType ?? 'Interface';
	const host = resolved.host;
	const staleTag = resolveStaleTag(node, topology, resolved, inputs.sites);
	const ipAddress = resolved.ipAddress ?? null;
	const servicesForHost = resolved.services ?? [];

	// Service elements: simpler rendering — single service with host name.
	// Intentionally does NOT read the hidden-category set here — category/tag
	// fading is handled by shouldFadeOut via the hidden-services store, so
	// category toggles don't trigger a recomputation.
	if (elementType === 'Service') {
		const service = resolved.services[0];
		// Hide hostname in views where Host is the container — it's redundant
		const showHostname = viewConfigFor(activeView)?.element_config?.container_entity !== 'Host';
		return {
			flags,
			staleTag,
			data: {
				elementType,
				footerText: null,
				services: service ? [service] : [],
				hiddenOpenPorts: [],
				headerText: showHostname && host ? hostDisplayName(host) : null,
				bodyText: service ? null : 'Unknown Service',
				showServices: !!service,
				isCategoryHidden: false,
				ip_address_id: nodeId,
				inlineGroups: []
			} as ElementRenderData
		};
	}

	// Host elements: show host name with services
	if (elementType === 'Host') {
		if (!host || !resolved.hostId) return { data: null, flags, staleTag };

		const hiddenCategories = hiddenServiceCategories(options, activeView);

		// Services visible in card. Filter = structural remove: hidden services
		// are dropped from the list entirely, not faded. The OpenPorts-category
		// subset is routed to the collapsed "+N open ports" indicator below.
		const isShown = (s: Service) => {
			if (hiddenEntityIds.has(s.id)) return false;
			const category = serviceDefinitions.getCategory(s.service_definition);
			if (category === 'OpenPorts' && hiddenCategories.includes(category)) return false;
			return true;
		};
		const inlineGroups = buildInlineGroups(
			node,
			servicesForHost,
			topology,
			hiddenEntityIds,
			isShown,
			(groupId) => !inputs.expandedInlineGroups.has(inlineGroupKey(nodeId, groupId))
		);
		const groupedServiceIds = new Set(
			inlineGroups.flatMap((g) => [...(g.header ? [g.header] : []), ...g.services]).map((s) => s.id)
		);
		const servicesOnHost = servicesForHost.filter(
			(s) => isShown(s) && !groupedServiceIds.has(s.id)
		);

		// OpenPorts hidden by category → collapsed indicator.
		// (Tag-hidden services of any category are already removed above.)
		const hiddenOpenPorts = servicesForHost.filter((s) => {
			if (hiddenEntityIds.has(s.id)) return false;
			const category = serviceDefinitions.getCategory(s.service_definition);
			return category === 'OpenPorts' && hiddenCategories.includes(category);
		});

		// Service names and port lines hide independently. Render the services
		// block if the view declares EITHER inlined and the user hasn't hidden
		// it — so toggling Services off still leaves port lines visible.
		const showServices =
			((flags.inlinesService && !flags.serviceInlineHidden) ||
				(flags.inlinesPort && !flags.portInlineHidden)) &&
			(servicesOnHost.length !== 0 || hiddenOpenPorts.length !== 0 || inlineGroups.length !== 0);

		const hostLabel = node.header ?? hostDisplayName(host);

		return {
			flags,
			staleTag,
			data: {
				elementType,
				footerText: null,
				services: servicesOnHost,
				hiddenOpenPorts,
				headerText: hostLabel,
				bodyText: showServices ? null : hostLabel,
				showServices,
				ip_address_id: nodeId,
				inlineGroups
			} as ElementRenderData
		};
	}

	// Port elements: show port name + status/MAC info
	if (elementType === 'Interface') {
		const iface = resolved.snmpInterface;
		const speed = iface?.speed_bps ? formatInterfaceSpeed(iface.speed_bps) : null;

		return {
			flags,
			staleTag,
			data: {
				elementType,
				headerText: node.header ?? null,
				footerText: null,
				bodyText: null,
				showServices: false,
				services: [],
				hiddenOpenPorts: [],
				// The card's selection key, as on Host and Service cards; '' left L2 ports unringable.
				ip_address_id: nodeId,
				inlineGroups: [],
				portStatus: iface
					? {
							operStatus: iface.oper_status,
							speed,
							macAddress: iface.mac_address ?? null
						}
					: undefined
			} as ElementRenderData
		};
	}

	// IPAddress elements
	if (!host || !resolved.hostId) return { data: null, flags, staleTag };

	const hiddenCategories = hiddenServiceCategories(options, activeView);

	const isContainerSubnet = ipAddress
		? getTopologyIndex(topology).subnetsById.get(ipAddress.subnet_id)?.cidr === '0.0.0.0/0'
		: false;

	// All services bound to this interface (after tag filtering)
	const allServicesOnIPAddress = servicesForHost.filter((s) =>
		s.bindings.some(
			(b) => b.ip_address_id == null || (ipAddress && b.ip_address_id == ipAddress.id)
		)
	);

	// Filter = structural remove (see Host branch for context).
	const servicesOnIPAddress = allServicesOnIPAddress.filter((s) => {
		if (hiddenEntityIds.has(s.id)) return false;
		const category = serviceDefinitions.getCategory(s.service_definition);
		if (category === 'OpenPorts' && hiddenCategories.includes(category)) return false;
		return true;
	});

	const hiddenOpenPorts = allServicesOnIPAddress.filter((s) => {
		if (hiddenEntityIds.has(s.id)) return false;
		const category = serviceDefinitions.getCategory(s.service_definition);
		return category === 'OpenPorts' && hiddenCategories.includes(category);
	});

	const headerText: string | null = node.header ?? null;
	// Service names and port lines hide independently — see the Host branch.
	const showServices =
		((flags.inlinesService && !flags.serviceInlineHidden) ||
			(flags.inlinesPort && !flags.portInlineHidden)) &&
		(servicesOnIPAddress.length != 0 || hiddenOpenPorts.length != 0);

	const subtitleText =
		ipAddress && !isContainerSubnet
			? (ipAddress.name ? ipAddress.name + ': ' : '') + ipAddress.ip_address
			: null;

	return {
		flags,
		staleTag,
		data: {
			elementType,
			footerText: null,
			subtitleText,
			services: servicesOnIPAddress,
			hiddenOpenPorts,
			headerText,
			bodyText: showServices ? null : hostDisplayName(host),
			showServices,
			ip_address_id: resolved.ipAddressId ?? null,
			inlineGroups: []
		} as ElementRenderData
	};
}

/**
 * The manager groups the backend inlined on a host card, resolved to entities.
 *
 * Each `inline_groups` entry says which entity type it names: service members come from the
 * card's own services, host members (macvlan containers, a guest's network identities) from the
 * topology's hosts, each carrying the services on that host. Hidden hosts and services drop out.
 */
export function buildInlineGroups(
	node: TopologyNode,
	servicesForHost: Service[],
	topology: RenderableTopology,
	hiddenEntityIds: Set<string>,
	isShown: (s: Service) => boolean,
	isCollapsed: (groupId: string) => boolean = () => false
): ElementInlineGroup[] {
	const entries = ((node as { inline_groups?: InlineGroup[] }).inline_groups ??
		[]) as InlineGroup[];
	if (entries.length === 0) return [];

	const index = getTopologyIndex(topology);
	const groups = new Map<string, ElementInlineGroup>();
	for (const entry of entries) {
		let group = groups.get(entry.group_id);
		if (!group) {
			group = {
				groupId: entry.group_id,
				collapsed: isCollapsed(entry.group_id),
				header: null,
				services: [],
				hosts: []
			};
			groups.set(entry.group_id, group);
		}
		if (entry.entity_type === 'Host') {
			const host = index.hostsById.get(entry.entity_id);
			if (!host || hiddenEntityIds.has(host.id)) continue;
			group.hosts.push({
				host,
				services: (index.servicesByHostId.get(host.id) ?? []).filter(isShown)
			});
			continue;
		}
		const service = servicesForHost.find((s) => s.id === entry.entity_id);
		if (!service || !isShown(service)) continue;
		if (entry.role === 'Header') group.header = service;
		else group.services.push(service);
	}

	return [...groups.values()].filter(
		(g) => g.header || g.services.length > 0 || g.hosts.length > 0
	);
}

/**
 * What the inspector's Services section shows for an element: the element's own services, and
 * the manager boxes its card draws (the same `buildInlineGroups` result, so the two agree on
 * members, hiding and service-less hosts).
 *
 * `ipAddressId` narrows the own services to those bound to that address, for an IP-address
 * element. A Host element passes null and keeps every service on the host.
 */
export function inspectorServiceSections(
	node: TopologyNode,
	services: Service[],
	topology: RenderableTopology,
	hiddenEntityIds: Set<string>,
	ipAddressId: string | null
): { own: Service[]; groups: ElementInlineGroup[] } {
	const isShown = (s: Service) => !hiddenEntityIds.has(s.id);
	const groups = buildInlineGroups(node, services, topology, hiddenEntityIds, isShown);
	const grouped = new Set(
		groups.flatMap((g) => [...(g.header ? [g.header] : []), ...g.services]).map((s) => s.id)
	);
	const own = services.filter(
		(s) =>
			isShown(s) &&
			!grouped.has(s.id) &&
			(ipAddressId === null ||
				s.bindings.some((b) => b.ip_address_id === ipAddressId || b.ip_address_id === null))
	);
	return { own, groups };
}

/**
 * A key identifying cards that render to the same height.
 *
 * Two element nodes whose keys match are assumed to measure identically, which
 * is what lets the pipeline mount one representative per key instead of every
 * node.
 *
 * Derived structurally from `ElementRenderResult` — counts, presence flags and
 * text-length buckets — rather than from a hand-picked list of fields, so a new
 * field added to the render data participates by default instead of being
 * silently ignored.
 *
 * Text contributes a *bucket*, not its content: cards are fixed-width, so a
 * long label wraps to another line while a short one does not. Bucketing by
 * rough length approximates the wrap point without making every distinct string
 * its own shape (which would defeat the sampling entirely).
 *
 * This is a heuristic, and the verification mode in the measure pass exists to
 * catch it being wrong: it re-measures nodes and reports any whose actual
 * height disagrees with the height predicted for their key.
 */
export function elementShapeKey(result: ElementRenderResult): string {
	const d = result.data;
	if (!d) return 'null';

	// An icon card renders only the glyph, an optional stale pill and a label clamped to two
	// lines of ~14 characters, whatever the detailed rows would have been.
	const icon = result.iconCard;
	if (icon) {
		return [
			'icon',
			d.elementType,
			Math.min(2, Math.ceil(icon.label.length / 14)),
			result.staleTag ? 1 : 0,
			// Each group box adds a heading and one single-line row per member.
			icon.groups.map((g) => g.members.length).join(',')
		].join('|');
	}

	// ~28 characters fit on a line at the fixed card width; bucket by line count
	// rather than raw length so near-identical labels share a key.
	const lines = (text: string | null | undefined): number =>
		text ? Math.ceil(text.length / 28) : 0;
	const serviceShape = (s: Service): string =>
		`${lines(s.name)}:${s.bindings.filter((b) => b.type === 'Port').length}:${s.service_definition ?? ''}`;

	const parts: (string | number)[] = [
		d.elementType,
		// The glyph sits in the header row beside the header text.
		result.glyph?.type ?? '-',
		lines(d.headerText),
		lines(d.subtitleText),
		lines(d.bodyText),
		lines(d.footerText),
		d.showServices ? 1 : 0,
		d.hiddenOpenPorts.length,
		result.staleTag ? 1 : 0,
		// Not just presence: the block renders status, speed and MAC on separate
		// lines, so two interface cards can both "have a port status" and still
		// differ in height. Verification caught exactly this — 1600 synthetic
		// interfaces without MACs measured 54px against 56 demo interfaces with
		// them at 70px, under one key.
		d.portStatus
			? `p${d.portStatus.operStatus ? 1 : 0}${d.portStatus.speed ? 1 : 0}${d.portStatus.macAddress ? 1 : 0}`
			: 'p-',
		// Each service renders its own row, and a row's height depends on its
		// name length and how many port lines it carries.
		result.flags.inlinesService && !result.flags.serviceInlineHidden ? 1 : 0,
		result.flags.inlinesPort && !result.flags.portInlineHidden ? 1 : 0,
		d.services.length,
		// `service_definition` decides whether the row renders a definition icon,
		// and an icon row is not the same height as a bare text row. Keying on the
		// id rather than on "has an icon" over-discriminates a little — two
		// different definitions that both render an icon get separate keys — which
		// is the safe direction: an extra key costs one more measured node, a
		// missed distinction lays cards out at the wrong height.
		...d.services.map(serviceShape),
		// Each group is a dashed box with a header row, its service rows, and per member host a
		// name row followed by that host's service rows.
		d.inlineGroups.length,
		// A collapsed box renders only its header row, so its members don't affect height.
		...d.inlineGroups.map((g) =>
			g.collapsed
				? `gc${g.header ? lines(g.header.name) : 0}`
				: `g${g.header ? lines(g.header.name) : 0}[${g.services.map(serviceShape).join(',')}]` +
					`[${g.hosts.map((h) => `${lines(hostDisplayName(h.host))}(${h.services.map(serviceShape).join(',')})`).join(',')}]`
		)
	];

	return parts.join('|');
}

/**
 * Assemble the non-node inputs from the current store state.
 *
 * The render pipeline needs to compute shape keys outside a component, where
 * the `$store` shorthand is unavailable. Reading them in one place keeps the
 * pipeline and the component agreed on what "current" means.
 */
export function currentElementRenderContext(): Omit<
	ElementRenderInputs,
	'nodeId' | 'node' | 'topology'
> {
	return {
		activeView: get(activeView),
		options: get(topologyOptions),
		hiddenEntityIds: get(hiddenEntityIds),
		expandedInlineGroups: get(expandedInlineGroups),
		sites: queryClient.getQueryData<Site[]>(queryKeys.sites.all) ?? []
	};
}
