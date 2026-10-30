<!--
	What to do about an entity Scanopy only inferred: an assumed subnet range, or a host another
	device reported.

	One layout for every unconfirmed entity, so confirming a subnet and confirming a host read the
	same: why it is unconfirmed, the evidence behind it, then the actions that settle it. Each
	entity type supplies its own actions and evidence; the badge that opens this is the shared
	Unconfirmed tag (`confirmation.ts`).
-->
<script lang="ts" module>
	import type { IconComponent } from '$lib/shared/utils/types';
	import type { Color } from '$lib/shared/utils/styling';

	export interface ConfirmAction {
		id: string;
		title: string;
		description: string;
		icon: IconComponent;
		color: Color;
		run: () => void;
	}
</script>

<script lang="ts">
	import type { Snippet } from 'svelte';
	import GenericModal from '$lib/shared/components/layout/GenericModal.svelte';
	import { createColorHelper } from '$lib/shared/utils/styling';

	interface Props {
		name: string;
		entityId?: string;
		title: string;
		intro: string;
		actions: ConfirmAction[];
		isOpen?: boolean;
		onClose: () => void;
		/** What the inference rests on, between the intro and the actions. */
		evidence?: Snippet;
	}

	let {
		name,
		entityId,
		title,
		intro,
		actions,
		isOpen = false,
		onClose,
		evidence
	}: Props = $props();
</script>

<GenericModal {name} {entityId} {title} {isOpen} {onClose} size="md">
	<!-- `modal-content` supplies no padding of its own, so a body has to bring the same
	     scrollable, padded container every other modal wraps its content in. -->
	<div class="flex min-h-0 flex-1 flex-col">
		<div class="flex-1 overflow-auto p-6">
			<div class="space-y-4">
				<p class="text-sm text-secondary">{intro}</p>

				{@render evidence?.()}

				<div class="flex flex-col gap-3">
					{#each actions as action (action.id)}
						{@const colors = createColorHelper(action.color)}
						<button onclick={() => action.run()} class="card w-full text-left">
							<div class="flex items-center gap-3">
								<div
									class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg {colors.bg}"
								>
									<action.icon class="h-5 w-5 {colors.icon}" />
								</div>
								<div class="min-w-0 flex-1">
									<p class="text-sm font-medium text-primary">{action.title}</p>
									<p class="text-xs text-secondary">{action.description}</p>
								</div>
							</div>
						</button>
					{/each}
				</div>
			</div>
		</div>
	</div>
</GenericModal>
