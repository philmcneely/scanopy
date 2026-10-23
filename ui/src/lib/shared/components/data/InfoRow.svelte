<script lang="ts">
	import { onMount } from 'svelte';
	import type { Snippet } from 'svelte';

	let {
		label,
		mono = false,
		children
	}: {
		label: string;
		mono?: boolean;
		children: Snippet;
	} = $props();

	let rowEl: HTMLDivElement;
	let wrapped = $state(false);

	function checkWrap() {
		if (!rowEl) return;
		const labelEl = rowEl.firstElementChild as HTMLElement;
		const valueEl = rowEl.lastElementChild as HTMLElement;
		if (labelEl && valueEl) {
			// Check if value starts below the label's bottom edge (actual wrap),
			// not just a baseline offset from different font sizes
			wrapped = valueEl.offsetTop >= labelEl.offsetTop + labelEl.offsetHeight;
		}
	}

	onMount(() => {
		// Measure after first paint
		requestAnimationFrame(checkWrap);
		const observer = new ResizeObserver(checkWrap);
		observer.observe(rowEl);
		return () => observer.disconnect();
	});
</script>

<div bind:this={rowEl} class="flex flex-wrap items-baseline justify-between gap-x-2 gap-y-0.5">
	<span class="shrink-0 text-sm text-secondary">{label}:</span>
	<span
		class="min-w-0 text-sm"
		class:font-mono={mono}
		class:text-xs={mono}
		style="text-align: {wrapped ? 'left' : 'right'}; color: var(--color-text-primary); {wrapped
			? 'border-left: 2px solid var(--color-border); padding-left: 8px;'
			: ''}"
	>
		{@render children()}
	</span>
</div>
