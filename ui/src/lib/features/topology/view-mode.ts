import type { components } from '$lib/api/schema';
import viewModesJson from '$lib/data/view-modes.json';
import type { TopologyOptions } from './types/base';

export type ViewMode = components['schemas']['ViewMode'];

type ViewModeMetadata = (typeof viewModesJson)[number];

/** The view modes a view offers, in the order the backend declares them. */
export function viewModesFor(view: string): ViewModeMetadata[] {
	return viewModesJson.filter((m) => m.metadata.views.includes(view));
}

/** The mode a view draws in: the stored choice if the view offers it, otherwise `Detailed`. */
export function effectiveViewMode(
	local: TopologyOptions['local'] | undefined,
	view: string
): ViewMode {
	const stored = local?.view_mode;
	if (stored && viewModesFor(view).some((m) => m.id === stored)) return stored;
	return 'Detailed';
}
