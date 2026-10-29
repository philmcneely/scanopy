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
import {
	ENTITY_COLLECTIONS,
	elementEntity,
	nodeEntity,
	resolveElementNode,
	topologyLookup
} from './resolvers';
import { entityGlyph, type EntityGlyph } from './entity-glyph';
import { elementMarks, entityMark, type ElementMarks, type MarkValue } from './element-marks';
import type { Host, HostVirtualization } from '$lib/features/hosts/types/base';
import type { ServiceVirtualization } from '$lib/features/services/types/base';
import { hostDisplayName } from '$lib/features/hosts/host-display-name';
import { hostOsLabel, type HostOs } from '$lib/features/hosts/host-os';
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
import { getConfirmationTag } from '$lib/shared/utils/confirmation';
import { createColorHelper } from '$lib/shared/utils/styling';
import type {
	TooltipContent,
	TooltipItem,
	TooltipRow,
	TooltipSection
} from '$lib/shared/actions/tooltip';
import { formatPort } from '$lib/shared/utils/formatting';
import {
	collectionLookup,
	entityDisplayText,
	entityFieldLabel,
	entityFields,
	fieldText,
	type EntityType
} from '$lib/shared/entity-fields';
import type { Site } from '$lib/features/sites/types';
import { get } from 'svelte/store';
import { activeView, hiddenEntityTypesIn, topologyOptions } from './queries';
import { hiddenEntityIds } from './interactions';
import { expandedInlineGroups, inlineGroupKey } from './collapse';
import { queryClient, queryKeys } from '$lib/api/query-client';
import {
	common_containers,
	common_host,
	common_ipAddress,
	common_operatingSystem,
	common_manager,
	common_name,
	common_services,
	common_status,
	common_virtualization,
	hosts_deviceFacts_composeProject,
	hosts_deviceFacts_guestType,
	hosts_deviceFacts_networkType,
	hosts_deviceType
} from '$lib/paraglide/messages';

type InlineGroup = components['schemas']['InlineGroup'];

/**
 * Whether the active view inlines services on this element, and whether the user has hidden them.
 * These decide whether the card carries services at all, so they belong with the render data rather
 * than being recomputed by the template.
 */
export interface ElementInlineFlags {
	/** Entity types the active view inlines on this element (e.g. Service). */
	inlineEntities: string[];
	inlinesService: boolean;
	serviceInlineHidden: boolean;
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
	 * What the element is: a device's device-type tile, software's logo or a port's icon, by its
	 * entity type's `glyph` metadata. Null for an element type that has none.
	 */
	glyph: EntityGlyph | null;
	/**
	 * The icon card, when the element has a glyph and resolves. It decides the card's height; null
	 * leaves a text fallback naming what failed to resolve.
	 */
	iconCard: ElementIconCard | null;
}

/**
 * An icon card: the element's glyph with a short label and the rest of the card's text on hover. A
 * device, software or port draws its glyph large above the label.
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
	/**
	 * A second, quieter line under the label: the host's hostname on an address card, so the card names
	 * the device as well as the address. Null when there is none or it repeats the label.
	 */
	sublabel: string | null;
	/** The host's operating system, drawn as a badge on the device glyph's bottom-left corner. */
	os: HostOs | null;
	/** Hover content: the card's name, labelled rows for what the card no longer shows, then its services. */
	tooltip: () => TooltipContent;
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
	/** The viewer collapsed the box: only its heading, with the member count, renders. */
	collapsed: boolean;
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
	tooltip: () => TooltipContent;
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
	const hiddenEntities = hiddenEntityTypesIn(inputs.options, inputs.activeView);

	return {
		inlineEntities,
		inlinesService: inlineEntities.includes('Service'),
		serviceInlineHidden: hiddenEntities.includes('Service')
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
	// One status pill per card. Unconfirmed says the device may not be what the map shows at all,
	// which outranks it not having been seen lately.
	return (
		getConfirmationTag(subject) ??
		getFreshnessTag(
			subject,
			sites.find((n) => n.id === subject.site_id),
			{
				entityTypeLabel: entities.getName(resolved.elementType ?? 'Host') || undefined
			}
		)
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
		case 'Libvirt':
			return [manager, statusRow(virtualization.details.state, status)];
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
		// An identity on its guest's network, not a guest of its own: no manager and no run state.
		case 'NetworkIdentity':
			return [
				{
					label: common_virtualization(),
					value: hostVirtualizations.getName(virtualization.type)
				}
			];
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
function titledTooltip(
	title: string,
	rows: MaybeRow[],
	services: (string | TooltipItem)[] = []
): TooltipContent {
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
 * A host's hover text: its name as the title, then its address and the address's name, device type
 * and how it runs as a guest, then its services. `status` is the view's Status mark on the host;
 * `serviceItem` lists each service (by its name, by default).
 */
export function hostTooltip(
	host: Host,
	{
		name,
		address,
		addressName,
		deviceType,
		services,
		status,
		serviceItem = (s) => s.name
	}: {
		name?: string;
		address?: string | null;
		addressName?: string | null;
		deviceType?: string | null;
		services: Service[];
		status?: MarkValue;
		serviceItem?: (service: Service) => string | TooltipItem;
	}
): TooltipContent {
	return titledTooltip(
		name ?? hostDisplayName(host),
		[
			{ label: common_ipAddress(), value: address },
			{ label: common_name(), value: addressName },
			{ label: hosts_deviceType(), value: deviceType ? deviceTypes.getName(deviceType) : null },
			{ label: common_operatingSystem(), value: host.os ? hostOsLabel(host.os) : null },
			...hostGuestRows(host.virtualization_metadata, status)
		],
		services.map(serviceItem)
	);
}

/** The interface fields a port card's tooltip lists, in order: what the card has no room for. */
const PORT_TOOLTIP_FIELDS = [
	'oper_status',
	'speed_bps',
	'mac_address',
	'native_vlan_id',
	'vlan_ids'
];

/**
 * A port's hover text: its name as the title, then its status, with a dot in the colour the view's
 * Status mark paints it, and its speed, MAC and VLANs. Labels and formatting are the backend field
 * registry's, as the inspector shows them.
 */
export function interfaceTooltip(
	iface: object,
	name: string,
	status: MarkValue | undefined
): TooltipContent {
	const fields = entityFields('Interface');
	const lookup = collectionLookup({});
	return titledTooltip(
		name,
		PORT_TOOLTIP_FIELDS.flatMap((id) => {
			const field = fields.find((f) => f.id === id);
			if (!field) return [];
			return [
				{
					label: entityFieldLabel('Interface', field),
					value: fieldText(field, iface, lookup),
					dot: id === 'oper_status' && status ? createColorHelper(status.color).rgb : undefined
				}
			];
		})
	);
}

/** A service with the ports it listens on at one address, set apart as the item's detail. */
function serviceWithPorts(
	service: Service,
	ipAddressId: string,
	topology: RenderableTopology
): TooltipItem {
	const portsById = getTopologyIndex(topology).portsById;
	const ports = service.bindings.flatMap((b) => {
		if (b.type !== 'Port' || !b.port_id) return [];
		if (b.ip_address_id != null && b.ip_address_id !== ipAddressId) return [];
		const port = portsById.get(b.port_id);
		return port ? [formatPort(port)] : [];
	});
	return { text: service.name, detail: ports.length > 0 ? ports.join(', ') : undefined };
}

/**
 * Any entity's hover text, by its type: a host, service or port the way its card shows it, and
 * any other entity (a subnet, a VLAN) as its summary fields, the ones the inspector shows by
 * default. `name` replaces the entity's own display text as the title. `status` is the view's
 * Status mark on the entity, read here so every caller colours run state the same way.
 */
export function entityTooltip(
	entityType: string,
	entity: object,
	{
		topology,
		view,
		sites,
		name
	}: { topology: RenderableTopology; view: string; sites: Site[]; name?: string }
): TooltipContent {
	const status = entityMark(view, 'Status', entityType, entity, sites);
	const index = getTopologyIndex(topology);
	switch (entityType) {
		case 'Host': {
			const host = entity as Host;
			const glyph = entityGlyph('Host', host, topology);
			return hostTooltip(host, {
				name,
				deviceType: glyph?.type === 'device' ? glyph.deviceType : null,
				services: (index.servicesByHostId.get(host.id) ?? []) as Service[],
				status
			});
		}
		case 'Service': {
			const service = entity as Service;
			return serviceTooltip(service, {
				host: index.hostsById.get(service.host_id) as Host | undefined,
				status
			});
		}
		case 'Interface':
			return interfaceTooltip(entity, name ?? entityDisplayText('Interface', entity), status);
		default: {
			const type = entityType as EntityType;
			const title = name ?? entityDisplayText(type, entity);
			const lookup = topologyLookup(topology);
			// The entity's own summary fields, then the related entities the topology carries (a
			// subnet's VLANs, the service managing it). A relation into a collection the topology
			// does not load would read as empty, so it is left out.
			const fields = entityFields(type);
			const reachable = (f: (typeof fields)[number]) =>
				f.path.every((hop) => hop.type === 'Embedded' || hop.entity in ENTITY_COLLECTIONS);
			const rows = fields
				.filter(
					(f) => (f.summary && f.path.length === 0) || (f.format.type === 'Entity' && reachable(f))
				)
				.map((f) => ({ label: entityFieldLabel(type, f), value: fieldText(f, entity, lookup) }))
				.filter((row) => row.value !== title);
			return titledTooltip(title, rows);
		}
	}
}

/**
 * The icon card for an element with a glyph, or null for one with nothing resolved, which keeps
 * the text fallback. Software is labelled with its name, a port with its own; a device with its
 * address on an address card, so two addresses on one host stay apart, and with its host's name
 * otherwise.
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

	if (glyph.type === 'port') {
		const iface = resolved.snmpInterface;
		if (!iface) return null;
		const label = data.headerText ?? iface.display_name ?? entities.getName('Interface');
		return {
			glyph,
			badgeDefinitions: [],
			label,
			sublabel: null,
			os: null,
			tooltip: () => interfaceTooltip(iface, label, marks.Status),
			groups: []
		};
	}

	if (glyph.type === 'software') {
		const service = resolved.services[0];
		if (!service) return null;
		return {
			glyph,
			badgeDefinitions: [],
			label: service.name,
			sublabel: null,
			os: null,
			tooltip: () => serviceTooltip(service, { host, status: marks.Status }),
			groups: []
		};
	}

	if (!host) return null;
	const deviceType = glyph.deviceType;
	// The badges are what runs on the thing the card stands for: the services bound to an address,
	// or a host's own services, the same list the inspector shows. The service that decided
	// the device type leads, so the top badge agrees with the glyph; logos come before glyphs.
	const meta = (s: Service) =>
		serviceDefinitions.getMetadata(s.service_definition) as {
			device_type?: string | null;
			logo?: string | null;
		} | null;
	const rank = (s: Service) => (meta(s)?.device_type === deviceType ? 0 : meta(s)?.logo ? 1 : 2);
	const badgeDefinitions = [
		...new Set(
			[...data.services].sort((a, b) => rank(a) - rank(b)).map((s) => s.service_definition)
		)
	].slice(0, BADGE_STACK_MAX);
	const hostName = data.headerText ?? hostDisplayName(host);
	// The container subnet's placeholder address names nothing, so that card keeps the host name.
	const ipAddress = data.namesAddress ? resolved.ipAddress : undefined;
	const address = ipAddress?.ip_address ?? null;
	return {
		glyph,
		badgeDefinitions,
		label: address ?? hostName,
		sublabel: address && host.hostname && host.hostname !== address ? host.hostname : null,
		os: host.os ?? null,
		tooltip: () =>
			hostTooltip(host, {
				name: hostName,
				address,
				addressName: ipAddress?.name,
				deviceType,
				services: data.services,
				status: marks.Status,
				// On an address card, each service with the ports it listens on there.
				serviceItem: ipAddress
					? (s) => serviceWithPorts(s, ipAddress.id, inputs.topology)
					: undefined
			}),
		groups: data.inlineGroups.map((g) => iconCardGroup(inputs, g))
	};
}

/**
 * One inlined manager group as an icon card draws it: a heading that opens and closes the box, and
 * every member as a one-line row while it is open.
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
			tooltip: () => serviceTooltip(service, { status })
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
			tooltip: () =>
				hostTooltip(host, {
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
		collapsed: group.collapsed,
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
				services: service ? [service] : [],
				headerText: showHostname && host ? hostDisplayName(host) : null,
				bodyText: service ? null : 'Unknown Service',
				showServices: !!service,
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
		// are dropped from the list entirely, not faded.
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

		const showServices =
			flags.inlinesService &&
			!flags.serviceInlineHidden &&
			(servicesOnHost.length !== 0 || inlineGroups.length !== 0);

		return {
			flags,
			staleTag,
			data: {
				elementType,
				services: servicesOnHost,
				headerText: node.header ?? hostDisplayName(host),
				bodyText: null,
				showServices,
				ip_address_id: nodeId,
				inlineGroups
			} as ElementRenderData
		};
	}

	// Port elements: the port's name; its status and the rest come from the interface itself.
	if (elementType === 'Interface') {
		return {
			flags,
			staleTag,
			data: {
				elementType,
				headerText: node.header ?? null,
				bodyText: null,
				showServices: false,
				services: [],
				// The card's selection key, as on Host and Service cards; '' left L2 ports unringable.
				ip_address_id: nodeId,
				inlineGroups: []
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

	const showServices =
		flags.inlinesService && !flags.serviceInlineHidden && servicesOnIPAddress.length !== 0;

	return {
		flags,
		staleTag,
		data: {
			elementType,
			namesAddress: Boolean(ipAddress) && !isContainerSubnet,
			services: servicesOnIPAddress,
			headerText: node.header ?? null,
			bodyText: null,
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

	const icon = result.iconCard;
	// An icon card renders only the glyph, an optional stale pill, a label clamped to two lines of
	// ~14 characters and an optional one-line sublabel.
	if (icon) {
		return [
			'icon',
			d.elementType,
			Math.min(2, Math.ceil(icon.label.length / 14)),
			icon.sublabel ? 1 : 0,
			result.staleTag ? 1 : 0,
			// Each group box adds a heading, and one single-line row per member while it is open.
			icon.groups.map((g) => (g.collapsed ? 'c' : g.members.length)).join(',')
		].join('|');
	}

	// The text fallback for an element that did not resolve: a header line and a body line, each
	// truncated to one line.
	return [
		'text',
		d.elementType,
		d.headerText ? 1 : 0,
		d.bodyText ? 1 : 0,
		result.staleTag ? 1 : 0
	].join('|');
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
