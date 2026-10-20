<script lang="ts">
	import type { components } from '$lib/api/schema';
	import { deviceRoles } from '$lib/shared/stores/metadata';

	/** A host's device role as text with its glyph: the hosts table's cell and the editor's fact. */
	let { role }: { role: components['schemas']['DeviceRole'] | null | undefined } = $props();

	let resolved = $derived(role ?? 'Unidentified');
	let RoleIcon = $derived(deviceRoles.getIconComponent(resolved));
</script>

<span class="inline-flex items-center gap-1.5 text-sm">
	<RoleIcon class="h-4 w-4 flex-shrink-0 {deviceRoles.getColorHelper(resolved).icon}" />
	{deviceRoles.getName(resolved)}
</span>
