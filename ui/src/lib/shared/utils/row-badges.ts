/**
 * The icon-only status badges a list row carries ahead of its name: Unconfirmed and Stale.
 *
 * Shown whatever columns the table displays, so a row says it is stale or awaiting confirmation
 * even with Last seen and Source hidden. Each is the shared tag (`getConfirmationTag`,
 * `getFreshnessTag`) with its label dropped; the tooltip carries the meaning.
 */

import type { Site } from '$lib/features/sites/types';
import type { TagProps } from '$lib/shared/components/data/types';
import type { EntityDiscriminants } from '$lib/api/entities';
import { entities } from '$lib/shared/stores/metadata';
import { getConfirmationTag, type ConfirmationSubject } from '$lib/shared/utils/confirmation';
import { getFreshnessTag, type FreshnessSubject } from '$lib/shared/utils/freshness';

/** The row badges for `entity`, Unconfirmed first. */
export function rowStatusBadges<
	T extends FreshnessSubject & ConfirmationSubject & { site_id?: string | null }
>(sites: () => Site[], entityType: EntityDiscriminants): (entity: T) => TagProps[] {
	return (entity) => {
		const site = sites().find((n) => n.id === entity.site_id);
		return [
			getConfirmationTag(entity),
			getFreshnessTag(entity, site, { entityTypeLabel: entities.getName(entityType) || undefined })
		]
			.filter((tag): tag is TagProps => tag !== null)
			.map((tag) => ({ ...tag, label: undefined }));
	};
}
