/**
 * Keys for a row's actions while that one row is selected, when they show in the toolbar.
 *
 * The row's delete action takes Delete. Every other action takes the first letter of its label
 * that no earlier action and no list-wide key has claimed: Edit is E, Rescan R, and Consolidate
 * O, since C is Create. An action left with no free letter shows without a key.
 */
import type { ActionTone, CardAction } from '../types';

/** Single keys the list already uses: C for Create, T for the tag island. */
export const RESERVED_ACTION_KEYS = ['C', 'T'];

/** The `key` value the delete action takes; Backspace triggers it too. */
export const DELETE_KEY = 'Delete';

export interface KeyedAction {
	action: CardAction;
	/** A capital letter, `DELETE_KEY`, or null when none was free. */
	key: string | null;
}

/** Whether a row with these actions can be deleted: it has a delete action, and it is enabled. */
export function canDeleteRow(actions: CardAction[]): boolean {
	return actions.some((action) => action.isDelete && !action.disabled);
}

// Whole class names, so Tailwind's content scan keeps them.
const ICON_BUTTON_CLASS: Record<ActionTone, string> = {
	danger: 'btn-icon-danger',
	warning: 'btn-icon-warning',
	success: 'btn-icon-success',
	info: 'btn-icon-info'
};
const TOOLBAR_BUTTON_CLASS: Record<ActionTone, string> = {
	danger: 'btn-danger',
	warning: 'btn-warning',
	success: 'btn-success',
	info: 'btn-info'
};

/** The class of an action's icon button in a table row. */
export function iconButtonClass(tone: ActionTone | undefined): string {
	return tone ? ICON_BUTTON_CLASS[tone] : 'btn-icon';
}

/** The class of an action's labelled button in the selection toolbar. */
export function toolbarButtonClass(tone: ActionTone | undefined): string {
	return tone ? TOOLBAR_BUTTON_CLASS[tone] : 'btn-secondary';
}

export function assignActionKeys(
	actions: CardAction[],
	reserved: readonly string[] = RESERVED_ACTION_KEYS
): KeyedAction[] {
	const taken = new Set(reserved);
	return actions.map((action) => {
		if (action.isDelete && !taken.has(DELETE_KEY)) {
			taken.add(DELETE_KEY);
			return { action, key: DELETE_KEY };
		}
		const letter = [...action.label.toUpperCase()].find(
			(char) => char >= 'A' && char <= 'Z' && !taken.has(char)
		);
		if (letter) taken.add(letter);
		return { action, key: letter ?? null };
	});
}

/** Whether a keydown presses `key` (from `assignActionKeys`) with no modifier held. */
export function matchesActionKey(event: KeyboardEvent, key: string): boolean {
	if (event.metaKey || event.ctrlKey || event.altKey || event.shiftKey) return false;
	if (key === DELETE_KEY) return event.key === 'Delete' || event.key === 'Backspace';
	return event.key.toUpperCase() === key;
}
