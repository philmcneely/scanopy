<script lang="ts">
	import FormField from './FormField.svelte';
	import type { AnyFieldApi } from '@tanstack/svelte-form';

	interface Props {
		label: string;
		field: AnyFieldApi;
		id: string;
		placeholder?: string;
		required?: boolean;
		helpText?: string;
		type?: 'text' | 'email' | 'password' | 'number';
		disabled?: boolean;
		autofocus?: boolean;
		/** The field's accessible name when `label` is empty because a label is drawn beside it. */
		ariaLabel?: string;
	}

	let {
		label,
		field,
		id,
		placeholder = '',
		required = false,
		helpText = '',
		type = 'text',
		disabled = false,
		autofocus = false,
		ariaLabel = undefined
	}: Props = $props();

	let hasErrors = $derived(field.state.meta.isTouched && field.state.meta.errors.length > 0);

	function focusOnMount(node: HTMLInputElement) {
		if (autofocus) node.focus();
	}
</script>

<FormField {label} {field} {required} {helpText} {id}>
	<input
		use:focusOnMount
		{id}
		{type}
		value={field.state.value ?? ''}
		onblur={() => field.handleBlur()}
		oninput={(e) => {
			const value = e.currentTarget.value;
			// Convert to number for number inputs, otherwise keep as string
			field.handleChange(type === 'number' ? (value === '' ? '' : Number(value)) : value);
		}}
		{placeholder}
		aria-label={ariaLabel}
		{disabled}
		class="input-field"
		class:input-field-error={hasErrors}
	/>
</FormField>
