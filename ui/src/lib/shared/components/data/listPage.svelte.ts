import type { ListPageParams } from '$lib/api/list-page';
import { narrowsList } from './controls/listQuery';

/**
 * A server-paginated list's state: the page the table is on, its size, and the list's view as
 * the endpoint's params. `DataControls` sets these through `onPageChange` and `onQueryChange`;
 * the tab's list query reads them through `params`.
 */
export function listPageState(initialPageSize = 20) {
	let pageSize = $state(initialPageSize);
	let currentPage = $state(1);
	// Search, filters and ordering. Null until DataControls has restored the view; the query is
	// not held for it, since the table only mounts once a first page has loaded.
	let listQuery = $state<URLSearchParams | null>(null);

	return {
		/** The page and view to request. */
		params(enabled = true): ListPageParams {
			return {
				limit: pageSize,
				offset: (currentPage - 1) * pageSize,
				list: listQuery ?? undefined,
				enabled
			};
		},
		/** The view as last sent, for an export of the same rows. */
		get listQuery(): URLSearchParams {
			return listQuery ?? new URLSearchParams();
		},
		/** Whether the view narrows the rows, so an empty page means "no match", not "none yet". */
		get isNarrowed(): boolean {
			return listQuery !== null && narrowsList(listQuery);
		},
		onPageChange(page: number, size: number) {
			currentPage = page;
			pageSize = size;
		},
		onQueryChange(query: URLSearchParams) {
			listQuery = query;
		}
	};
}
