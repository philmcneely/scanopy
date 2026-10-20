import { describe, it, expect } from 'vitest';
import { columnSectionKey, fieldsToColumns } from '$lib/shared/components/data/table/columns';
import { dataControlsTabs, tabColumns } from './entity-tabs';

/**
 * Every column a tab's column menu lists sits under a heading. For an entity column the heading is
 * its backend group: the column's registry field, or the entity's declared table column
 * (`EntityDiscriminants::table_columns`). Tag-group columns sit under their own section. A column added to a tab without
 * either fails here, naming the tab and the column, instead of landing in the menu under no heading.
 */
describe('column menu groups', () => {
	it('gives every listed column of every tab a group', () => {
		const ungrouped = dataControlsTabs().flatMap((tab) =>
			fieldsToColumns(tabColumns(tab))
				.filter((column) => !column.primary && columnSectionKey(column) === null)
				.map((column) => `${tab}: ${column.id}`)
		);
		expect(ungrouped).toEqual([]);
	});
});
