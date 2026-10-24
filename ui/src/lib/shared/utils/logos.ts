/**
 * Logos, by the ids the backend's `Logo` enum gives them. `logos.json` holds each logo's served
 * path and what fills its frame, so whatever draws a logo only names its id.
 */

import logos from '$lib/data/logos.json';
import { createLogoIconComponent } from '$lib/shared/utils/styling';
import type { IconComponent } from '$lib/shared/utils/types';

type LogoEntry = (typeof logos)[number];

const byId = new Map<string, LogoEntry>(logos.map((logo) => [logo.id, logo]));

/** What fills a logo's frame: a colour (`Solid`), the logo itself (`Round`), or nothing. */
export function logoFrame(background: LogoEntry['background']): {
	fill: string | null;
	round: boolean;
} {
	if (background === 'Round') return { fill: null, round: true };
	if (typeof background === 'object' && 'Solid' in background) {
		return { fill: background.Solid, round: false };
	}
	return { fill: null, round: false };
}

/** Where the UI serves the logo with this id, for an `<img>`; null when there is none. */
export function logoPath(id: string): string | null {
	return byId.get(id)?.path ?? null;
}

/** The logo with this id as an icon component, or `fallback` when there is none. */
export function logoIcon(id: string | null | undefined, fallback: IconComponent): IconComponent {
	const logo = id ? byId.get(id) : undefined;
	return logo ? createLogoIconComponent(logo.id, logo.path, logoFrame(logo.background)) : fallback;
}
