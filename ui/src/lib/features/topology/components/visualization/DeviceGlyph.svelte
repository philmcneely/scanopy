<script lang="ts">
	import type { components } from '$lib/api/schema';
	import { deviceRoles } from '$lib/shared/stores/metadata';

	/**
	 * A device: a neutral square outline holding its device role's glyph. The square is
	 * the shape the topology gives every entity whose `glyph` metadata is `Device`.
	 */
	let {
		role,
		size = 'sm',
		framed = true
	}: {
		role: components['schemas']['DeviceRole'];
		size?: 'sm' | 'lg';
		/** False inside a container header, whose own frame is already the shape. */
		framed?: boolean;
	} = $props();

	let Glyph = $derived(deviceRoles.getIconComponent(role));
	let colors = $derived(deviceRoles.getColorHelper(role));
</script>

{#if !framed}
	<!-- Bare, in the Host colour every role shares: how a host's glyph draws outside a frame. -->
	<Glyph class="h-5 w-5 flex-shrink-0 {colors.icon}" />
{:else}
	<!-- A neutral outline on the card surface, the glyph in the text colour: the shape tells a device
     from software and the glyph reads first. Colour fills turned a map into same-coloured blocks. -->
	<span
		class="inline-flex flex-shrink-0 items-center justify-center border-gray-300 dark:border-gray-600 {size ===
		'lg'
			? 'h-12 w-12 rounded-lg border-2'
			: 'h-5 w-5 rounded border'}"
		style="background: var(--color-bg-surface);"
	>
		<Glyph class="text-primary {size === 'lg' ? 'h-7 w-7' : 'h-3.5 w-3.5'}" />
	</span>
{/if}
