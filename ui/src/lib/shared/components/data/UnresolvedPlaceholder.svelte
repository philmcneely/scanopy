<script lang="ts">
	import { HelpCircle } from '@lucide/svelte';

	interface Props {
		label: string;
	}

	let { label }: Props = $props();
</script>

<div
	class="flex items-center gap-3 rounded-lg border border-dashed border-gray-300 px-3 py-2 dark:border-gray-600"
>
	<div class="flex h-7 w-7 shrink-0 items-center justify-center">
		<HelpCircle class="h-5 w-5 text-tertiary" />
	</div>
	<span class="text-sm text-tertiary">{label}</span>
</div>
