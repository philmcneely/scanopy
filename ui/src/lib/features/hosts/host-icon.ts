import { deviceRoles } from '$lib/shared/stores/metadata';
import type { IconComponent } from '$lib/shared/utils/types';
import type { Host } from './types/base';

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
