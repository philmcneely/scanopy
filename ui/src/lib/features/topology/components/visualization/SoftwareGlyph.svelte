<script lang="ts">
	import { serviceDefinitions } from '$lib/shared/stores/metadata';

	/**
	 * Software: its service definition's logo in a round frame. The circle is the shape the topology
	 * gives every entity whose `glyph` metadata is `Software`. The frame takes the card's own surface,
	 * so a logo gets a white backdrop only when its definition asks for one (`LogoIcon` adds it).
	 */
	let {
		definition,
		size = 'sm'
	}: {
		definition: string;
		size?: 'sm' | 'lg';
	} = $props();

	let Logo = $derived(serviceDefinitions.getIconComponent(definition));
	let colors = $derived(serviceDefinitions.getColorHelper(definition));
</script>

<span
	class="inline-flex flex-shrink-0 items-center justify-center rounded-full border {colors.border} {size ===
	'lg'
		? 'h-12 w-12 border-2'
		: 'h-5 w-5'}"
	style="background: var(--color-bg-surface);"
>
	<Logo class="{size === 'lg' ? 'h-7 w-7' : 'h-3.5 w-3.5'} {colors.icon}" />
</span>
