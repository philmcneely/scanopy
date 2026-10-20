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
	// Pixel size, passed explicitly: `LogoIcon` sizes its image from `size` and ignores a height
	// class. 70% of the frame is the largest square that sits inside the circle, so a square logo
	// (pfSense) fits as well as a round one (UniFi).
	let logoPx = $derived(size === 'lg' ? 28 : 14);
</script>

<span
	class="inline-flex flex-shrink-0 items-center justify-center overflow-hidden rounded-full border {colors.border} {size ===
	'lg'
		? 'h-12 w-12 border-2'
		: 'h-5 w-5'}"
	style="background: var(--color-bg-surface);"
>
	<Logo size={logoPx} class={colors.icon} />
</span>
