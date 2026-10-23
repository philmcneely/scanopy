import {
	columnResizingFeature,
	columnSizingFeature,
	constructTable,
	rowSortingFeature,
	tableFeatures,
	type RowData,
	type Table,
	type TableOptions
} from '@tanstack/table-core';
import { storeReactivityBindings } from '@tanstack/table-core/store-reactivity-bindings';

/**
 * The table-core features entity tables use. Sorting is registered only so
 * columns report `getCanSort()` and the controlled `sorting` slice exists; rows
 * arrive sorted, and no sorted row model is registered.
 */
export const entityTableFeatures = tableFeatures({
	coreReactivityFeature: storeReactivityBindings(),
	rowSortingFeature,
	columnSizingFeature,
	columnResizingFeature
});

export type EntityTableFeatures = typeof entityTableFeatures;
export type EntityTableOptions<T extends RowData> = Omit<
	TableOptions<EntityTableFeatures, T>,
	'features'
>;

/**
 * Build a table-core instance with the entity table features.
 *
 * A slice the caller leaves out of `state` falls back to the feature default:
 * table-core v9 resolves each slice on its own, so a partial `state` is safe.
 */
export function constructEntityTable<T extends RowData>(
	options: EntityTableOptions<T>
): Table<EntityTableFeatures, T> {
	return constructTable({ ...options, features: entityTableFeatures });
}

/**
 * `@tanstack/table-core` as a derived view model.
 *
 * The adapter is deliberately thin, and the instance owns no state of its own:
 * `options.state` is authoritative, supplied by whoever already persists it.
 * Rows arrive filtered, sorted and paginated, and the table is given only the
 * core row model — so there is no code path here that could reorder or drop a
 * row, and no second source of truth to drift from the first.
 *
 * `getOptions` is re-read inside a `$derived`, so every rune it touches
 * invalidates the snapshot. `state` is spread there so its getters are read
 * inside the derived too. Re-applying options inside the derived rather than
 * an effect is what makes the first render correct: an effect runs after the
 * template has already read stale header groups.
 */
export function createSvelteTable<T extends RowData>(getOptions: () => EntityTableOptions<T>) {
	const table = constructEntityTable(getOptions());

	const snapshot = $derived.by(() => {
		const options = getOptions();
		table.setOptions((old) => ({
			...old,
			...options,
			state: { ...(options.state ?? {}) },
			features: entityTableFeatures
		}));
		return {
			headers: table.getHeaderGroups()[0]?.headers ?? [],
			rows: table.getRowModel().rows
		};
	});

	return {
		get headers() {
			return snapshot.headers;
		},
		get rows() {
			return snapshot.rows;
		},
		get table() {
			return table;
		}
	};
}
