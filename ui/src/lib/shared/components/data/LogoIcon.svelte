<script lang="ts">
	/**
	 * A logo in the round frame topology cards give it: a ringed disc in the card colour, or in the
	 * logo's own background colour, with the logo inside it. A round logo fills the disc.
	 */
	let {
		size = 24,
		class: className = '',
		iconName,
		iconUrl,
		fill = null,
		round = false,
		framed = true
	}: {
		size?: number;
		class?: string;
		iconName: string;
		iconUrl: string;
		/** The colour that fills the frame: the logo's own background, or white behind a dark mark. */
		fill?: string | null;
		/** The logo is a disc: drawn at the frame's inner size so it fills it. */
		round?: boolean;
		/** False where the surrounding shape is already the frame (a container header): the bare logo. */
		framed?: boolean;
	} = $props();

	// 70% of the frame is the largest square that sits inside the circle, so a square logo fits as
	// well as a round one; the 1px ring takes the rest. Unframed, a logo with a background colour
	// sits on a small rounded square of it, 2px in from each side.
	let logoSize = $derived(
		!framed ? (fill ? size - 4 : size) : round ? size - 2 : Math.round(size * 0.7)
	);

	const fallbackIcon =
		'data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMjQiIGhlaWdodD0iMjQiIHZpZXdCb3g9IjAgMCAyNCAyNCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPGNpcmNsZSBjeD0iMTIiIGN5PSIxMiIgcj0iMTAiIHN0cm9rZT0iY3VycmVudENvbG9yIiBzdHJva2Utd2lkdGg9IjIiLz4KPHA=';

	let imgElement: HTMLImageElement | undefined = $state();

	function handleError() {
		if (imgElement) {
			imgElement.src = fallbackIcon;
		}
	}
</script>

<div
	class="inline-flex shrink-0 items-center justify-center {framed
		? 'overflow-hidden rounded-full border border-gray-300 dark:border-gray-600'
		: fill
			? 'rounded-sm'
			: ''} {className}"
	data-logo-fill={fill ?? undefined}
	data-logo-round={round ? '' : undefined}
	style="width: {size}px; height: {size}px; {framed || fill
		? `background: ${fill ?? 'var(--color-topology-node-bg)'};`
		: ''}"
>
	<img
		bind:this={imgElement}
		src={iconUrl}
		alt="{iconName} icon"
		width={logoSize}
		height={logoSize}
		class="block object-contain"
		style="width: {logoSize}px; height: {logoSize}px;"
		onerror={handleError}
	/>
</div>
