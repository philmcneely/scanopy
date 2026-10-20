import type { components } from '$lib/api/schema';
import { entities } from '$lib/shared/stores/metadata';
import { getTopologyIndex } from './entity-index';
import type { RenderableTopology, TopologyHost } from './types/base';

type DeviceRole = components['schemas']['DeviceRole'];

/**
 * What an entity draws as its glyph: a device as its host's role (a square tile), software as its
 * definition's logo (a round frame). Which entity is which is the backend's `glyph` metadata on the
 * entity type; this only resolves the role or definition for the one entity in hand.
 */
export type EntityGlyph =
	{ type: 'device'; role: DeviceRole } | { type: 'software'; definition: string };

/** The glyph for an entity of a given type, or null for a type that keeps its plain icon. */
export function entityGlyph(
	entityType: string,
	entity: object,
	topology: RenderableTopology
): EntityGlyph | null {
	const meta = entities.getMetadata(entityType) as { glyph?: 'Device' | 'Software' } | null;
	switch (meta?.glyph) {
		case 'Device': {
			// A device glyph is its machine's role: the host itself, or the host an address sits on.
			const hostId =
				'host_id' in entity ? (entity.host_id as string) : (entity as { id: string }).id;
			const host = getTopologyIndex(topology).hostsById.get(hostId) as TopologyHost | undefined;
			return { type: 'device', role: host?.device_role ?? 'Unidentified' };
		}
		case 'Software':
			return {
				type: 'software',
				definition: (entity as { service_definition: string }).service_definition
			};
		default:
			return null;
	}
}
