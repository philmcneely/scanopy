<script lang="ts">
	/**
	 * `PaidCrown` on a primary button, as an icon component: for an icon slot that takes a component
	 * and no props, such as a modal action's `icon`.
	 */
	import PaidCrown from './PaidCrown.svelte';

	let { size = 16, class: className = '' }: { size?: number; class?: string } = $props();
</script>

<PaidCrown {size} class={className} onPrimary />
