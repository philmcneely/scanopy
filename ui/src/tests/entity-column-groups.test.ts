import { describe, it, expect } from 'vitest';
import { fieldsToColumns } from '$lib/shared/components/data/table/columns';
import { dataControlsTabs, tabColumns } from './entity-tabs';

/**
 * Every column a tab's column menu lists sits under a group heading, and the group comes from the
 * backend: the column's registry field, or the entity's declared table column
 * (`EntityDiscriminants::table_columns`). A column added to a tab without either fails here, naming
 * the tab and the column, instead of landing in the menu under no heading.
 */
describe('column menu groups', () => {
	it('gives every listed column of every tab a group', () => {
		const ungrouped = dataControlsTabs().flatMap((tab) =>
			fieldsToColumns(tabColumns(tab))
				.filter((column) => !column.primary && !column.display.group)
				.map((column) => `${tab}: ${column.id}`)
		);
		expect(ungrouped).toEqual([]);
	});
});
