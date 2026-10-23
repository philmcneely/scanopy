<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		/** Unique ID for the input */
		id: string;
		/** Label text for the option (supports HTML for required indicators) */
		label: string;
		/** Help text shown below the label */
		helpText?: string;
		/** Whether the option is disabled */
		disabled?: boolean;
		/** Slot for the input element (checkbox or radio) */
		children: Snippet;
	}

	let { id, label, helpText = '', disabled = false, children }: Props = $props();
</script>

<label for={id} class="flex cursor-pointer items-start gap-2" class:disabled>
	<div class="mt-0.5 shrink-0">
		{@render children()}
	</div>
	<div class="flex flex-col">
		<!-- eslint-disable-next-line svelte/no-at-html-tags -- label content is sanitized -->
		<span class="text-sm text-primary">{@html label}</span>
		{#if helpText}
			<span class="text-xs text-tertiary">{helpText}</span>
		{/if}
	</div>
</label>

<style>
	.disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}
</style>
