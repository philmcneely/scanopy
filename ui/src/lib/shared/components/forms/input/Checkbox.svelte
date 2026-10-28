<script lang="ts">
	import FormField from './FormField.svelte';
	import type { AnyFieldApi } from '@tanstack/svelte-form';

	interface Props {
		label: string;
		field: AnyFieldApi;
		id: string;
		helpText?: string;
		required?: boolean;
		disabled?: boolean;
	}

	let { label, field, id, helpText = '', required = false, disabled = false }: Props = $props();
</script>

<div class:disabled>
	<FormField {label} {field} {helpText} {id} inline={true} {required}>
		<input
			type="checkbox"
			{id}
			checked={field.state.value}
			{disabled}
			onchange={(e) => field.handleChange(e.currentTarget.checked)}
			class="checkbox-card h-4 w-4 focus:ring-1 focus:ring-blue-500 disabled:cursor-not-allowed disabled:opacity-50"
		/>
	</FormField>
</div>

<style>
	input[type='checkbox']:checked {
		background-color: rgb(37, 99, 235);
		border-color: rgb(37, 99, 235);
	}

	.disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.disabled :global(label) {
		cursor: not-allowed;
	}
</style>
