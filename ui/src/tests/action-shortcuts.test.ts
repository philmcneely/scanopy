import { describe, it, expect } from 'vitest';
import {
	assignActionKeys,
	canDeleteRow,
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

describe('canDeleteRow', () => {
	const del = (extra: Partial<CardAction> = {}) =>
		action('Delete', { tone: 'danger', isDelete: true, ...extra });

	it('allows a row whose actions include an enabled delete', () => {
		expect(canDeleteRow([action('Edit'), del()])).toBe(true);
	});

	it('refuses a row with no delete, or a disabled one', () => {
		expect(canDeleteRow([action('Edit')])).toBe(false);
		expect(canDeleteRow([action('Edit'), del({ disabled: true })])).toBe(false);
	});

	it('refuses a row whose only destructive action is not its delete', () => {
		expect(canDeleteRow([action('Revoke', { tone: 'danger' })])).toBe(false);
	});
});
