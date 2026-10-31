<script lang="ts">
	/**
	 * The crown that marks what the plan lacks: a paid feature's control, its badge, and the way to
	 * the plans. Lucide's crown, filled. On the page it is amber with a darker outline, which keeps
	 * 3:1 against a light background; `onPrimary` draws it on a primary (blue) button.
	 */
	import { Crown } from '@lucide/svelte';

	let {
		size = 16,
		class: className = '',
		onPrimary = false
	}: { size?: number; class?: string; onPrimary?: boolean } = $props();
</script>

<Crown
	{size}
	aria-hidden="true"
	class="shrink-0 {onPrimary
		? 'fill-amber-400 text-amber-400'
		: 'fill-amber-500 text-amber-700 dark:fill-amber-400 dark:text-amber-400'} {className}"
/>
