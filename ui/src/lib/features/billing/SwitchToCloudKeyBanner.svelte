<script lang="ts">
	import { KeyRound } from 'lucide-svelte';
	import AppBanner from '$lib/shared/components/feedback/AppBanner.svelte';
	import { useOrganizationQuery } from '$lib/features/organizations/queries';
	import { hasLicensedPlan } from '$lib/features/organizations/types';
	import { formatLongDate } from '$lib/shared/utils/formatting';
	import {
		settings_billing_license_switchToCloudAirGappedDesc,
		settings_billing_license_switchToCloudDesc
	} from '$lib/paraglide/messages';

	// In the plan picker, for an org on a license plan: what choosing a cloud plan does to the
	// license key its server runs on. This is the one place it is said before the switch.
	const organizationQuery = useOrganizationQuery();
	let org = $derived(organizationQuery.data);
	let airGappedUntil = $derived(org?.air_gapped_key_current_until ?? null);
</script>

{#if org != null && hasLicensedPlan(org)}
	<AppBanner
		variant="warning"
		icon={KeyRound}
		body={airGappedUntil
			? settings_billing_license_switchToCloudAirGappedDesc({
					date: formatLongDate(airGappedUntil)
				})
			: settings_billing_license_switchToCloudDesc()}
	/>
{/if}
