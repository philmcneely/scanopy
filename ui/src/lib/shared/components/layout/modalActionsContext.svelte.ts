import { getContext, setContext } from 'svelte';
import type { ModalAction } from './modalActions';

const CONTEXT_KEY = Symbol('modal-actions');

interface ModalActionsContext {
	/** Declares the footer actions; the returned function withdraws them. */
	declare: (actions: () => ModalAction[]) => () => void;
}

/** GenericModal's side of `useModalActions`. */
export function provideModalActions(context: ModalActionsContext) {
	setContext(CONTEXT_KEY, context);
}

/**
 * Declares the enclosing modal's footer actions from a component inside it, for a footer that
 * belongs to one tab or one form within the modal. Withdrawn when the component unmounts. Outside
 * a modal it does nothing.
 */
export function useModalActions(actions: () => ModalAction[]) {
	const context = getContext<ModalActionsContext | undefined>(CONTEXT_KEY);
	if (!context) return;
	$effect(() => context.declare(actions));
}
