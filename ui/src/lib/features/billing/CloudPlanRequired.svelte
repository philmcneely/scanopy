<script lang="ts">
	import { Cloud } from 'lucide-svelte';
	import EmptyState from '$lib/shared/components/layout/EmptyState.svelte';
	import { useOrganizationQuery } from '$lib/features/organizations/queries';
	import { triggerUpgrade } from '$lib/features/billing/trigger-upgrade';
	import { formatLongDate } from '$lib/shared/utils/formatting';
	import type { IconComponent } from '$lib/shared/utils/types';
	import {
		settings_billing_license_pageNeedsCloudPlan,
		settings_billing_license_switchToCloud,
		settings_billing_license_switchToCloudAirGappedDesc,
		settings_billing_license_switchToCloudDesc
	} from '$lib/paraglide/messages';

	/**
	 * What a main-app page shows to an org on a licensed self-hosted plan: the
	 * page needs a cloud plan, and what switching does to the license key.
	 */
	let { page }: { page: string } = $props();

	const organizationQuery = useOrganizationQuery();
	let airGappedUntil = $derived(organizationQuery.data?.air_gapped_key_current_until ?? null);

	function switchToCloud() {
		triggerUpgrade({
			source: 'switch_to_cloud_empty_state',
			surface: 'sidebar',
			hosting: 'cloud'
		});
	}
</script>

<EmptyState
	IconComponent={Cloud as IconComponent}
	title={settings_billing_license_pageNeedsCloudPlan({ page })}
	subtitle={airGappedUntil
		? settings_billing_license_switchToCloudAirGappedDesc({ date: formatLongDate(airGappedUntil) })
		: settings_billing_license_switchToCloudDesc()}
	cta={settings_billing_license_switchToCloud()}
	onClick={switchToCloud}
/>
