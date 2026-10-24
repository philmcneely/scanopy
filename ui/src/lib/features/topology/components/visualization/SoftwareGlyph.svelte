<script lang="ts">
	import { serviceDefinitions } from '$lib/shared/stores/metadata';

	/**
	 * Software: its service definition's logo in a neutral round frame. The circle is the shape the
	 * topology gives every entity whose `glyph` metadata is `Software`. The frame fills with the
	 * topology card's opaque background: the dark-mode surface token is 70% translucent, which let the
	 * tile behind a badge show through the logo. A logo gets a white backdrop only when its definition
	 * asks for one (`LogoIcon` adds it).
	 */
	let {
		definition,
		size = 'sm',
		framed = true
	}: {
		definition: string;
		size?: 'sm' | 'md' | 'lg';
		/** False inside a container header, whose own frame is already the shape. */
		framed?: boolean;
	} = $props();

	let Logo = $derived(serviceDefinitions.getIconComponent(definition));
	// Pixel size, passed explicitly: `LogoIcon` sizes its image from `size` and ignores a height
	// class. 70% of the frame is the largest square that sits inside the circle, so a square logo
	// (pfSense) fits as well as a round one (UniFi).
	let logoPx = $derived(size === 'lg' ? 28 : size === 'md' ? 17 : 14);
</script>

{#if !framed}
	<Logo size={16} class="text-primary" />
{:else}
	<span
		class="inline-flex shrink-0 items-center justify-center overflow-hidden rounded-full border border-gray-300 align-top dark:border-gray-600 {size ===
		'lg'
			? 'h-12 w-12 border-2'
			: size === 'md'
				? 'h-6 w-6'
				: 'h-5 w-5'}"
		style="background: var(--color-topology-node-bg);"
	>
		<!-- A logo draws in its own colours; a definition with no logo falls back to a glyph, drawn in the
	     text colour like a device's so the two read alike. -->
		<Logo size={logoPx} class="text-primary" />
	</span>
{/if}
