import { deviceRoles } from '$lib/shared/stores/metadata';
import type { IconComponent } from '$lib/shared/utils/types';
import type { Host } from './types/base';
import DeviceGlyph from '$lib/features/hosts/components/DeviceGlyph.svelte';

/**
 * The icon for a specific host: its device role's glyph, in the Host colour every role shares.
 *
 * Every surface that draws one host (a card, a chip, a hover card, a modal header) reads this, so
 * a host is the same picture in the list, the map and the editor. A host nothing identifies is
 * `Unidentified`, whose glyph is the generic Host icon, so a host known only by id looks the same.
 */
export function hostIcon(host: Pick<Host, 'device_role'> | null | undefined): IconComponent {
	return deviceRoles.getIconComponent(host?.device_role ?? 'Unidentified');
}

/**
 * The same icon as a tile: the role's glyph on its neutral square, for a host's card (the hover
 * card, a picker row, the inspector), where the icon sits in its own slot. Chips and inline
 * mentions use `hostIcon`, bare.
 */
export function hostTile(host: Pick<Host, 'device_role'> | null | undefined): IconComponent {
	const role = host?.device_role ?? 'Unidentified';
	// Pre-binds the role, as `createLogoIconComponent` pre-binds a logo. The slot's own sizing
	// classes are dropped: the tile has a fixed size and draws its glyph in the text colour.
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	return ($$payload: any, _$$props: any) => DeviceGlyph($$payload, { role, size: 'md' });
}
