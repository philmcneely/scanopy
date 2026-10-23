<script lang="ts">
	import FormField from './FormField.svelte';
	import type { AnyFieldApi } from '@tanstack/svelte-form';

	interface Props {
		label: string;
		field: AnyFieldApi;
		id: string;
		required?: boolean;
		helpText?: string;
		disabled?: boolean;
	}

	let { label, field, id, required = false, helpText = '', disabled = false }: Props = $props();

	let hasErrors = $derived(field.state.meta.isTouched && field.state.meta.errors.length > 0);
</script>

<FormField {label} {field} {required} {helpText} {id}>
	<input
		{id}
		type="time"
		value={field.state.value ?? ''}
		onblur={() => field.handleBlur()}
		oninput={(e) => field.handleChange(e.currentTarget.value)}
		{disabled}
		class="time-picker input-field"
		class:input-field-error={hasErrors}
	/>
</FormField>

<style>
	:global(.time-picker) {
		color-scheme: dark;
	}

	:global(.time-picker::-webkit-calendar-picker-indicator) {
		cursor: pointer;
	}

	:global(.time-picker::-webkit-calendar-picker-indicator:hover) {
		filter: invert(1) opacity(1);
	}
</style>
