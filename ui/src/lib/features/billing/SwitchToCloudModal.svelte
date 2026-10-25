<script lang="ts">
	import GenericModal from '$lib/shared/components/layout/GenericModal.svelte';
	import type { ModalAction } from '$lib/shared/components/layout/modalActions';
	import { triggerUpgrade } from '$lib/features/billing/trigger-upgrade';
	import {
		common_cancel,
		settings_billing_license_cloudAppNeedsCloudPlan,
		settings_billing_license_switchToCloud,
		settings_billing_license_switchToCloudDesc
	} from '$lib/paraglide/messages';

	/**
	 * Shown when an org on a self-hosted plan clicks into the cloud app from the
	 * sidebar: the app needs a cloud plan, and switching ends the license.
	 */
	let { isOpen, onClose }: { isOpen: boolean; onClose: () => void } = $props();

	let actions = $derived<ModalAction[]>([
		{ role: 'cancel', label: common_cancel(), onClick: () => onClose() },
		{
			role: 'primary',
			label: settings_billing_license_switchToCloud(),
			onClick: () =>
				triggerUpgrade({
					source: 'switch_to_cloud_interstitial',
					surface: 'sidebar',
					hosting: 'cloud',
					beforeModal: () => onClose()
				})
		}
	]);
</script>

<GenericModal
	{isOpen}
	title={settings_billing_license_cloudAppNeedsCloudPlan()}
	size="md"
	{onClose}
	{actions}
>
	<div class="p-6">
		<p class="text-sm text-secondary">{settings_billing_license_switchToCloudDesc()}</p>
	</div>
</GenericModal>
