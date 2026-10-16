import { describe, it, expect } from 'vitest';
import {
	assignActionKeys,
	selectionActions,
	DELETE_KEY,
	RESERVED_ACTION_KEYS
} from '$lib/shared/components/data/controls/actionShortcuts';
import type { CardAction } from '$lib/shared/components/data/types';
import type { IconComponent } from '$lib/shared/utils/types';

const action = (label: string, extra: Partial<CardAction> = {}): CardAction => ({
	label,
	icon: null as unknown as IconComponent,
	onClick: () => {},
	...extra
});

const keys = (actions: CardAction[]) => assignActionKeys(actions).map((a) => a.key);

describe('assignActionKeys', () => {
	it('takes each label’s first free letter, skipping the list’s own keys', () => {
		expect(keys([action('Edit'), action('Rescan'), action('Consolidate')])).toEqual([
			'E',
			'R',
			'O'
		]);
		expect(RESERVED_ACTION_KEYS).toContain('C');
	});

	it('gives the delete action Delete, whatever its label', () => {
		expect(keys([action('Edit'), action('Remove', { isDelete: true })])).toEqual(['E', DELETE_KEY]);
	});

	it('gives a danger action that is not the delete a letter', () => {
		expect(
			keys([action('Upgrade', { tone: 'danger' }), action('Delete', { isDelete: true })])
		).toEqual(['U', DELETE_KEY]);
	});

	it('moves to a later letter when an earlier action took the first', () => {
		expect(keys([action('Edit'), action('Enable')])).toEqual(['E', 'N']);
	});

	it('leaves an action keyless when every letter is taken', () => {
		expect(keys([action('Tc')])).toEqual([null]);
	});
});

describe('selectionActions', () => {
	const bulk = () => {};
	const del = (extra: Partial<CardAction> = {}) =>
		action('Delete', { tone: 'danger', isDelete: true, onBulk: bulk, ...extra });
	const rescan = (extra: Partial<CardAction> = {}) => action('Rescan', { onBulk: bulk, ...extra });
	const ran: string[] = [];
	const runBulk = (a: CardAction) => () => void ran.push(a.label);
	const run = (perRow: CardAction[][]) => selectionActions(perRow, runBulk, 'blocked');

	it('shows one row’s actions as the row does, bulk ones running on the selection', () => {
		const [edit, remove] = run([[action('Edit'), del()]]);
		expect(edit).toMatchObject({ label: 'Edit' });
		expect(remove).toMatchObject({ label: 'Delete', tone: 'danger', disabled: false });
		remove.onClick();
		expect(ran).toContain('Delete');
	});

	it('keeps a disabled action disabled, with its tooltip', () => {
		const [remove] = run([[del({ disabled: true, tooltip: 'running' })]]);
		expect(remove).toMatchObject({ disabled: true, tooltip: 'running' });
	});

	it('offers every bulk action any selected row has, and only bulk ones, for several rows', () => {
		const actions = run([
			[action('Edit'), rescan(), del()],
			[action('Edit'), del()]
		]);
		expect(actions.map((a) => a.label)).toEqual(['Rescan', 'Delete']);
	});

	it('disables an action for several rows unless every row has it enabled', () => {
		const [rescanAll] = run([[rescan()], [action('Edit')]]);
		expect(rescanAll).toMatchObject({ disabled: true, tooltip: 'blocked' });
		const [removeAll] = run([[del()], [del({ disabled: true, tooltip: 'in use' })]]);
		expect(removeAll).toMatchObject({ disabled: true, tooltip: 'in use' });
		expect(run([[del()], [del()]])).toMatchObject([{ disabled: false }]);
	});

	it('offers nothing for several rows when none has a bulk action', () => {
		expect(run([[action('Edit')], [action('Revoke', { tone: 'danger' })]])).toEqual([]);
	});
});
