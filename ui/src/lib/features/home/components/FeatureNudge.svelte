<script lang="ts" module>
	/** One of a nudge's buttons. */
	export interface NudgeAction {
		label: string;
		onAction: () => void;
	}
</script>

<script lang="ts">
	import { X, ArrowRight } from '@lucide/svelte';
	import { onMount } from 'svelte';
	import type { IconComponent } from '$lib/shared/utils/types';
	import { trackEvent } from '$lib/shared/utils/analytics';
	import { common_dismiss } from '$lib/paraglide/messages';

	let {
		id,
		title,
		description,
		actions,
		onDismiss,
		iconColor,
		Icon,
		dismissable = true,
		dismissOnAction = true
	}: {
		id: string;
		title: string;
		description: string;
		/** One button per action, in order. */
		actions: NudgeAction[];
		onDismiss?: () => void;
		dismissable?: boolean;
		/** Whether acting on the nudge also dismisses it. Off for a nudge that clears on its own
		 *  once the action is done, so backing out of the action leaves it in place. */
		dismissOnAction?: boolean;
		Icon: IconComponent;
		iconColor: string;
	} = $props();

	let dismissed = $state(false);
	let dismissKey = $derived(`nudge-dismissed:${id}`);

	onMount(() => {
		dismissed = localStorage.getItem(dismissKey) === 'true';
	});

	function dismiss() {
		trackEvent('nudge_dismissed', { nudge_id: id });
		localStorage.setItem(dismissKey, 'true');
		dismissed = true;
		onDismiss?.();
	}

	function handleAction(action: NudgeAction) {
		trackEvent('nudge_action_clicked', { nudge_id: id });
		if (dismissOnAction) dismiss();
		action.onAction();
	}
</script>

{#if !dismissed}
	<div class="card-static card">
		<div class="flex items-start justify-between gap-3">
			{#if Icon}
				<div class={`mt-0.5 shrink-0 ${iconColor}`}>
					<Icon size={20} />
				</div>
			{/if}
			<div class="flex-1">
				<h4 class="mb-1 text-sm font-semibold text-primary">{title}</h4>
				<!-- eslint-disable-next-line svelte/no-at-html-tags -- description is developer-provided, not user input -->
				<p class="text-sm text-tertiary">{@html description}</p>
				<div class="mt-2 flex flex-wrap gap-x-4 gap-y-1">
					{#each actions as action (action.label)}
						<button
							onclick={() => handleAction(action)}
							class="inline-flex items-center gap-1 text-sm font-medium text-blue-400 transition-colors hover:text-blue-300"
						>
							{action.label}
							<ArrowRight class="h-3.5 w-3.5" />
						</button>
					{/each}
				</div>
			</div>
			{#if dismissable}
				<button
					onclick={dismiss}
					class="shrink-0 rounded-sm p-0.5 text-tertiary transition-colors hover:bg-white/10"
					aria-label={common_dismiss()}
				>
					<X class="h-4 w-4" />
				</button>
			{/if}
		</div>
	</div>
{/if}
