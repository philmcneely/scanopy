<script lang="ts">
	import GenericModal from '$lib/shared/components/layout/GenericModal.svelte';
	import MiniTopologyPreview from './MiniTopologyPreview.svelte';
	import { trackEvent } from '$lib/shared/utils/analytics';
	import type { ModalAction } from '$lib/shared/components/layout/modalActions';
	import {
		daemons_promptTitle,
		daemons_promptBody,
		daemons_promptSkip,
		daemons_promptGetStarted
	} from '$lib/paraglide/messages';

	let {
		isOpen,
		onInstall,
		onSkip
	}: {
		isOpen: boolean;
		onInstall: () => void;
		onSkip: () => void;
	} = $props();

	let tracked = $state(false);

	$effect(() => {
		if (isOpen && !tracked) {
			trackEvent('daemon_prompt_viewed');
			tracked = true;
		}
	});

	let actions = $derived<ModalAction[]>([
		{ role: 'cancel', label: daemons_promptSkip(), onClick: () => onSkip() },
		{ role: 'primary', label: daemons_promptGetStarted(), onClick: () => onInstall() }
	]);
</script>

<GenericModal
	{isOpen}
	title={daemons_promptTitle()}
	size="md"
	onClose={onSkip}
	showCloseButton={false}
	preventCloseOnClickOutside={true}
	{actions}
>
	<div class="flex min-h-0 flex-1 flex-col">
		<div class="flex-1 overflow-auto p-6">
			<div class="space-y-6">
				<MiniTopologyPreview active={isOpen} />

				<p class="text-secondary text-sm">{daemons_promptBody()}</p>
			</div>
		</div>
	</div>
</GenericModal>
