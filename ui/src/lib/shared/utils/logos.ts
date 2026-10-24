/**
 * Logos, by the ids the backend's `Logo` enum gives them. `logos.json` holds each logo's served
 * path and what it is drawn on, so whatever draws a logo only names its id.
 */

import logos from '$lib/data/logos.json';
import { createLogoIconComponent } from '$lib/shared/utils/styling';
import type { IconComponent } from '$lib/shared/utils/types';

const byId = new Map(logos.map((logo) => [logo.id, logo]));

/** Where the UI serves the logo with this id, for an `<img>`; null when there is none. */
export function logoPath(id: string): string | null {
	return byId.get(id)?.path ?? null;
}

/** The logo with this id as an icon component, or `fallback` when there is none. */
export function logoIcon(id: string | null | undefined, fallback: IconComponent): IconComponent {
	const logo = id ? byId.get(id) : undefined;
	return logo ? createLogoIconComponent(logo.id, logo.path, logo.background === 'White') : fallback;
}
