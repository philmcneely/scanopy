import { describe, it, expect, vi } from 'vitest';
import {
	assignModalKeys,
	isTopmostModal,
	registerOpenModal,
	resolveModalKeydown,
	ENTER_KEY,
	ESCAPE_KEY,
	type ModalAction,
	type ModalKeydown
} from '$lib/shared/components/layout/modalActions';
import { DELETE_KEY } from '$lib/shared/components/data/controls/actionShortcuts';

const action = (role: ModalAction['role'], label: string, extra: Partial<ModalAction> = {}) =>
	({ role, label, onClick: vi.fn(), ...extra }) as ModalAction;

/** A stand-in for the element a key landed on, with what the resolver reads from it. */
const element = (tagName: string, extra: Record<string, unknown> = {}) =>
	({
		tagName,
		type: 'text',
		isContentEditable: false,
		getAttribute: (name: string) => (extra.attributes as Record<string, string>)?.[name] ?? null,
		...extra
	}) as unknown as EventTarget;

const keydown = (key: string, target: EventTarget, extra: Partial<ModalKeydown> = {}) =>
	({
		key,
		target,
		metaKey: false,
		ctrlKey: false,
		altKey: false,
		shiftKey: false,
		repeat: false,
		isComposing: false,
		defaultPrevented: false,
		...extra
	}) as ModalKeydown;

const textInput = element('INPUT');
const textarea = element('TEXTAREA');

/**
 * A form modal's actions, as an editor declares them: the primary action submits through
 * validation, so it reaches `handleSubmit` only when the form is valid.
 */
function formModal(valid: boolean, disabled = false) {
	const handleSubmit = vi.fn();
	const submitForm = async () => {
		if (valid) handleSubmit();
	};
	const keyed = assignModalKeys([
		action('cancel', 'Cancel'),
		action('primary', 'Save', { onClick: submitForm, disabled })
	]);
	return { keyed, handleSubmit };
}

/** Runs the action a keydown resolves to, as GenericModal does. */
async function press(event: ModalKeydown, keyed: ReturnType<typeof assignModalKeys>) {
	const result = resolveModalKeydown(event, keyed);
	await result.action?.onClick();
	return result;
}

describe('Enter on a form modal', () => {
	it('submits a valid form from a text field, as the button does', async () => {
		const { keyed, handleSubmit } = formModal(true);
		const result = await press(keydown('Enter', textInput), keyed);
		expect(result.preventDefault).toBe(true);
		expect(handleSubmit).toHaveBeenCalledTimes(1);
	});

	it('stops at validation on an invalid form, as a click does', async () => {
		const { keyed, handleSubmit } = formModal(false);
		await press(keydown('Enter', textInput), keyed);
		await keyed.find(({ key }) => key === ENTER_KEY)!.action.onClick();
		expect(handleSubmit).not.toHaveBeenCalled();
	});

	it('does nothing while the primary action is disabled, and still blocks implicit submit', async () => {
		const { keyed, handleSubmit } = formModal(true, true);
		const result = await press(keydown('Enter', textInput), keyed);
		expect(result).toEqual({ action: null, preventDefault: true });
		expect(handleSubmit).not.toHaveBeenCalled();
	});

	it('ignores a held key, so holding Enter submits once', () => {
		const { keyed } = formModal(true);
		expect(resolveModalKeydown(keydown('Enter', textInput, { repeat: true }), keyed).action).toBe(
			null
		);
	});
});

describe('Enter in a textarea', () => {
	const keyed = assignModalKeys([action('primary', 'Save')]);

	it('types a newline', () => {
		expect(resolveModalKeydown(keydown('Enter', textarea), keyed)).toEqual({
			action: null,
			preventDefault: false
		});
	});

	it.each([{ metaKey: true }, { ctrlKey: true }])('runs the primary action with %o', (mod) => {
		expect(resolveModalKeydown(keydown('Enter', textarea, mod), keyed).action?.label).toBe('Save');
	});
});

describe('Enter that belongs to a control', () => {
	const keyed = assignModalKeys([action('primary', 'Save')]);

	it('leaves an open combobox to pick its option', () => {
		const combobox = element('INPUT', { attributes: { 'aria-expanded': 'true' } });
		expect(resolveModalKeydown(keydown('Enter', combobox), keyed).action).toBe(null);
	});

	it('leaves a key a dropdown already handled', () => {
		const event = keydown('Enter', textInput, { defaultPrevented: true });
		expect(resolveModalKeydown(event, keyed).action).toBe(null);
	});

	it('leaves a focused button to press itself', () => {
		expect(resolveModalKeydown(keydown('Enter', element('BUTTON')), keyed).action).toBe(null);
	});

	it('runs the primary action with a closed combobox focused', () => {
		const combobox = element('INPUT', { attributes: { 'aria-expanded': 'false' } });
		expect(resolveModalKeydown(keydown('Enter', combobox), keyed).action?.label).toBe('Save');
	});
});

describe('keys for the other actions', () => {
	const keyed = assignModalKeys([
		action('danger', 'Delete'),
		action('secondary', 'Back'),
		action('cancel', 'Cancel'),
		action('primary', 'Save')
	]);
	const body = element('BODY');

	it('gives cancel Escape, danger Delete, and secondaries a letter', () => {
		expect(keyed.map(({ key }) => key)).toEqual([DELETE_KEY, 'B', ESCAPE_KEY, ENTER_KEY]);
	});

	it('runs a secondary on its letter outside a field', () => {
		expect(resolveModalKeydown(keydown('b', body), keyed).action?.label).toBe('Back');
	});

	it('never fires a letter or Delete while the user types in a field', () => {
		expect(resolveModalKeydown(keydown('b', textInput), keyed).action).toBe(null);
		expect(
			resolveModalKeydown(keydown('Backspace', textInput, { metaKey: true }), keyed).action
		).toBe(null);
	});

	it('runs danger only with the modifier held', () => {
		expect(resolveModalKeydown(keydown('Backspace', body), keyed).action).toBe(null);
		expect(resolveModalKeydown(keydown('Delete', body), keyed).action).toBe(null);
		expect(
			resolveModalKeydown(keydown('Backspace', body, { metaKey: true }), keyed).action?.label
		).toBe('Delete');
	});

	it('gives a second danger action no key rather than a bare letter', () => {
		const twoDangers = assignModalKeys([action('danger', 'Delete'), action('danger', 'Purge')]);
		expect(twoDangers.map(({ key }) => key)).toEqual([DELETE_KEY, null]);
		expect(resolveModalKeydown(keydown('p', body), twoDangers).action).toBe(null);
	});

	it('skips a disabled action', () => {
		const disabled = assignModalKeys([action('secondary', 'Back', { disabled: true })]);
		expect(resolveModalKeydown(keydown('b', body), disabled)).toEqual({
			action: null,
			preventDefault: false
		});
	});
});

describe('stacked modals', () => {
	it('lets only the topmost modal answer, handing back to the one beneath on close', () => {
		const editor = Symbol('editor');
		const confirm = Symbol('confirm');
		const closeEditor = registerOpenModal(editor);
		const closeConfirm = registerOpenModal(confirm);
		expect(isTopmostModal(confirm)).toBe(true);
		expect(isTopmostModal(editor)).toBe(false);
		closeConfirm();
		expect(isTopmostModal(editor)).toBe(true);
		closeEditor();
		expect(isTopmostModal(editor)).toBe(false);
	});
});
