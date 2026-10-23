<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		title?: string;
		subtitle?: string | null;
		/** Rich title content, rendered in place of `title` (e.g. a title with an entity chip). */
		heading?: Snippet;
		/** Rendered at the right of the title row — a category tag, a status. */
		aside?: Snippet;
	}

	let { title = '', subtitle = null, heading, aside }: Props = $props();
</script>

<div class="flex items-start justify-between gap-2 border-b border-(--color-border) pb-4">
	<div class="min-w-0">
		<h3 class="flex items-center gap-1.5 text-sm font-medium text-primary">
			{#if heading}
				{@render heading()}
			{:else}
				{title}
			{/if}
		</h3>
		{#if subtitle}
			<p class="text-sm text-secondary">{subtitle}</p>
		{/if}
	</div>
	{#if aside}
		{@render aside()}
	{/if}
</div>
