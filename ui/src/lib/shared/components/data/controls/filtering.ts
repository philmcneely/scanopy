import { SvelteSet } from 'svelte/reactivity';
import { getFieldKey, type FieldConfig } from '../types';

/** One field's filter selection. Readonly set so both `Set` and `SvelteSet` fit. */
export interface FieldFilter {
	type: 'string' | 'boolean' | 'array';
	values: ReadonlySet<string>;
	showTrue?: boolean;
	showFalse?: boolean;
}

export type FilterState = Record<string, FieldFilter>;

/**
 * Which filters the parent applies server-side. The list holds one page, so a filter the server
 * does not apply narrows nothing.
 */
export interface ServerFilterMode {
	/** The server applies the tag filter from the list's query. */
	tags: boolean;
}

/** Whether the parent applies this field's filter. */
export function isHandledServerSide<T>(field: FieldConfig<T>, server: ServerFilterMode): boolean {
	if (getFieldKey(field) === 'tags' && server.tags) return true;
	return field.serverFiltered === true;
}

/**
 * Filterable fields the server does not filter by.
 *
 * The client holds one page while the count describes the whole match set, so every filter has to
 * be handled by the server that produced the count, or not offered at all.
 */
export function serverFilterViolations<T>(
	fields: FieldConfig<T>[],
	server: ServerFilterMode
): string[] {
	return fields
		.filter((field) => field.filterable === true && !isHandledServerSide(field, server))
		.map(getFieldKey);
}

/**
 * A filter with nothing selected, shaped for the field's type.
 *
 * `applyDefaults` seeds a string field from `filterDefaults`, which is what a
 * fresh mount wants and what "clear filters" deliberately does not: clearing
 * means clearing, including a default the product chose.
 */
export function blankFilter<T>(field: FieldConfig<T>, applyDefaults: boolean): FieldFilter {
	if (field.type === 'boolean') {
		return { type: 'boolean', values: new SvelteSet(), showTrue: true, showFalse: true };
	}
	if (field.type === 'array') {
		return { type: 'array', values: new SvelteSet() };
	}
	return {
		type: 'string',
		values: new SvelteSet(applyDefaults ? field.filterDefaults : undefined)
	};
}

/** A blank filter for every filterable field. */
export function blankFilterState<T>(fields: FieldConfig<T>[], applyDefaults: boolean): FilterState {
	const state: FilterState = {};

	for (const field of fields) {
		if (!field.filterable) continue;
		state[getFieldKey(field)] = blankFilter(field, applyDefaults);
	}

	return state;
}

/** The checked booleans as strings, as a boolean filter's query param carries them. */
export function booleanFilterValues(showTrue: boolean, showFalse: boolean): string[] {
	const values: string[] = [];
	if (showTrue) values.push('true');
	if (showFalse) values.push('false');
	return values;
}

/**
 * Toggle one value of a string or array filter, returning the new state.
 *
 * A pure transition returning fresh objects: the component reassigns rather
 * than mutating, so `$derived` work downstream re-runs. Returns null when the
 * key names no filter of a togglable type, which the caller reads as "nothing
 * to do".
 */
export function toggleValue(
	state: FilterState,
	fieldKey: string,
	value: string
): FilterState | null {
	const filter = state[fieldKey];
	if (!filter || (filter.type !== 'string' && filter.type !== 'array')) return null;

	const values = new SvelteSet(filter.values);
	if (values.has(value)) {
		values.delete(value);
	} else {
		values.add(value);
	}

	return { ...state, [fieldKey]: { ...filter, values } };
}

/**
 * Toggle one box of a boolean filter, returning the new state and the boxes
 * left checked.
 *
 * `normalizeEmpty` re-checks both boxes when the last one is unticked. That
 * state asks for an empty set, which a query string cannot express and nobody
 * wants — it renders an empty table under a non-zero count — so a filter the
 * server applies falls back to no constraint instead. The tags filter, which
 * has no boolean, is the only one the server may not apply.
 */
export function toggleBoolean(
	state: FilterState,
	fieldKey: string,
	box: 'showTrue' | 'showFalse',
	normalizeEmpty: boolean
): { state: FilterState; showTrue: boolean; showFalse: boolean } | null {
	const filter = state[fieldKey];
	if (!filter || filter.type !== 'boolean') return null;

	let showTrue = box === 'showTrue' ? !filter.showTrue : (filter.showTrue ?? false);
	let showFalse = box === 'showFalse' ? !filter.showFalse : (filter.showFalse ?? false);

	if (normalizeEmpty && !showTrue && !showFalse) {
		showTrue = true;
		showFalse = true;
	}

	return {
		state: { ...state, [fieldKey]: { ...filter, showTrue, showFalse } },
		showTrue,
		showFalse
	};
}

/** Whether one field's filter would currently narrow the list. */
export function isFieldFilterActive<T>(
	field: FieldConfig<T>,
	filter: FieldFilter | undefined
): boolean {
	if (!field.filterable || !filter) return false;
	if (field.type === 'boolean') return !filter.showTrue || !filter.showFalse;
	return filter.values.size > 0;
}

/** How many filters currently narrow the list: one per filtered column, plus Stale only. */
export function activeFilterCount<T>(
	fields: FieldConfig<T>[],
	filterState: FilterState,
	staleOnly: boolean
): number {
	const columns = fields.filter((field) =>
		isFieldFilterActive(field, filterState[getFieldKey(field)])
	).length;
	return columns + (staleOnly ? 1 : 0);
}

/** Whether any filter would currently narrow the list. */
export function hasActiveFilters<T>(
	fields: FieldConfig<T>[],
	filterState: FilterState,
	staleOnly: boolean
): boolean {
	return activeFilterCount(fields, filterState, staleOnly) > 0;
}
