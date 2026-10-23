import { getFieldKey, isOrderableField, type FieldConfig, type TableDefaults } from '../types';

export type SortDirection = 'asc' | 'desc';

export interface SortState {
	field: string | null;
	direction: SortDirection;
}

/**
 * Where a click on `fieldKey` moves the sort.
 *
 * Clicking the active field flips direction; clicking any other field starts it
 * ascending. The result always names exactly one field, which is what lets the
 * table set a non-`none` `aria-sort` on exactly one header.
 */
export function nextSortState(current: SortState, fieldKey: string): SortState {
	if (current.field === fieldKey) {
		return { field: fieldKey, direction: current.direction === 'asc' ? 'desc' : 'asc' };
	}
	return { field: fieldKey, direction: 'asc' };
}

/**
 * Whether the user can sort by this field: the server orders it.
 *
 * The sort control and the table headers both read this, so a header can never
 * offer a sort the control doesn't, and vice versa.
 */
export function isSortableField<T>(field: FieldConfig<T>): boolean {
	return isOrderableField(field);
}

/**
 * Whether the user can group by this field.
 *
 * String and boolean orderable fields group, unless `groupable: false`. An array field never
 * groups: its value has no single bucket.
 */
export function isGroupableField<T>(field: FieldConfig<T>): boolean {
	if (field.type === 'array' || !isOrderableField(field)) return false;
	return (field.type === 'string' || field.type === 'boolean') && field.groupable !== false;
}

/** Fields offered in the sort control. */
export function sortableFields<T>(fields: FieldConfig<T>[]): FieldConfig<T>[] {
	return fields.filter(isSortableField);
}

/** Fields offered in the group-by control. */
export function groupableFields<T>(fields: FieldConfig<T>[]): FieldConfig<T>[] {
	return fields.filter(isGroupableField);
}

/**
 * Default group and sort keys a list cannot apply.
 *
 * A list groups and sorts only by rendered columns, so a default on a missing
 * field, a field that cannot group or sort, or a column hidden until the user
 * shows it would silently open the tab ungrouped or unsorted.
 */
export function defaultOrderingViolations<T>(
	fields: FieldConfig<T>[],
	defaults: TableDefaults
): string[] {
	const rendersByDefault = (field: FieldConfig<T>) =>
		!field.display?.hidden && !field.display?.hiddenByDefault;
	const fieldFor = (key: string) => fields.find((f) => getFieldKey(f) === key);

	const offenders: string[] = [];
	if (defaults.group) {
		const field = fieldFor(defaults.group);
		if (!field || !rendersByDefault(field) || !isGroupableField(field)) {
			offenders.push(`group "${defaults.group}"`);
		}
	}
	if (defaults.sort) {
		const field = fieldFor(defaults.sort.field);
		if (!field || !rendersByDefault(field) || !isSortableField(field)) {
			offenders.push(`sort "${defaults.sort.field}"`);
		}
	}
	return offenders;
}
