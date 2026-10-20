<script lang="ts">
	import type { components } from '$lib/api/schema';
	import { deviceRoles } from '$lib/shared/stores/metadata';

	/**
	 * A device: a square outline in its device role's colour, holding the role's glyph. The square is
	 * the shape the topology gives every entity whose `glyph` metadata is `Device`.
	 */
	let {
		role,
		size = 'sm'
	}: {
		role: components['schemas']['DeviceRole'];
		size?: 'sm' | 'lg';
	} = $props();

	let Glyph = $derived(deviceRoles.getIconComponent(role));
	let colors = $derived(deviceRoles.getColorHelper(role));
</script>

<!-- An outline, not a fill: the colour sits in the border so the glyph, in the text colour, reads
     first. A filled tile turned a map of devices into a field of same-coloured blocks. -->
<span
	class="inline-flex flex-shrink-0 items-center justify-center {colors.border} {size === 'lg'
		? 'h-12 w-12 rounded-lg border-2'
		: 'h-5 w-5 rounded border'}"
	style="background: var(--color-bg-surface);"
>
	<Glyph class="text-primary {size === 'lg' ? 'h-7 w-7' : 'h-3.5 w-3.5'}" />
</span>
