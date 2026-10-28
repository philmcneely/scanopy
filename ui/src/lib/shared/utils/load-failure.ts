/** What a page needs from a query to tell a failed load from an empty one. */
export interface LoadableQuery {
	isError: boolean;
	data: unknown;
	refetch: () => unknown;
}

/**
 * Whether any of a page's queries failed with nothing to show. TanStack leaves `data` undefined
 * after a failed first load, which a page reading `data ?? []` would present as "nothing here".
 * A query that failed a refetch keeps its last data and is not counted: the page still has
 * something true to show.
 */
export function loadFailure(...queries: LoadableQuery[]): { failed: boolean; retry: () => void } {
	const failed = queries.filter((q) => q.isError && q.data === undefined);
	return {
		failed: failed.length > 0,
		retry: () => failed.forEach((q) => q.refetch())
	};
}
