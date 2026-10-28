<script lang="ts">
	import { RefreshCw } from 'lucide-svelte';
	import AppBanner from './AppBanner.svelte';
	import { reloadForServerUpdate, serverUpdatePending } from '$lib/api/server-version';
	import { common_reload, serverUpdated_body } from '$lib/paraglide/messages';
</script>

<!-- Not dismissable: the tab is running a frontend older than the server, and reloads by itself
     once nothing holds unsaved input. -->
{#if $serverUpdatePending}
	<AppBanner variant="info" icon={RefreshCw} body={serverUpdated_body()}>
		{#snippet actions()}
			<button
				onclick={reloadForServerUpdate}
				class="ml-2 rounded-sm px-2 py-0.5 text-xs font-medium underline hover:no-underline"
			>
				{common_reload()}
			</button>
		{/snippet}
	</AppBanner>
{/if}
