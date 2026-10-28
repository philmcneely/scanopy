<script lang="ts">
	export let title: string;
	export let items: { name: string; id: string }[];
</script>

{#if items.length > 0}
	<div>
		{#if title.length > 0}
			<h4 class="mb-2 font-medium text-secondary">
				{title} ({items.length})
			</h4>
		{/if}
		<ul class="card-static card space-y-2">
			{#each items as item (item.id)}
				<li class="flex items-center gap-2 text-sm text-secondary">
					<span>• {item.name}</span>
				</li>
			{/each}
		</ul>
	</div>
{/if}
