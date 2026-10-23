<script lang="ts">
	import { ExternalLink } from 'lucide-svelte';

	interface Props {
		text: string;
		href: string;
		linkText: string;
		class?: string;
	}

	let { text, href, linkText, class: className = '' }: Props = $props();

	let parts = $derived(text.split('%link%'));
</script>

<p class="text-xs text-tertiary {className}">
	{parts[0]}<a
		{href}
		target="_blank"
		rel="external noopener noreferrer"
		class="text-blue-500 hover:underline"
		>{linkText}<ExternalLink class="ml-0.5 inline h-3 w-3" /></a
	>{parts[1] ?? ''}
</p>
