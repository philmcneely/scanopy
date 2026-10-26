<script lang="ts" module>
	import { credentialTypes } from '$lib/shared/stores/metadata';
	import type {
		TypedTypeMetadata,
		CredentialTypeMetadata,
		FeatureId
	} from '$lib/shared/stores/metadata';
	import {
		getStabilityTagProps,
		getTargetTagProps,
		getUpstreamSupportTagProps
	} from '$lib/features/credentials/types/base';
	import { upgradeTagProps } from '$lib/features/billing/upgrade-tag';
	import { common_availableOnPaidPlans } from '$lib/paraglide/messages';

	export type CredentialTypeOption = TypedTypeMetadata<CredentialTypeMetadata>;

	/** Optional context for the dropdown: a non-null `disabledReason` renders the
	 *  option as disabled (unselectable) with the reason as a hover tooltip. A non-null
	 *  `upgradeFeature` (see `credentialTypeUpgradeFeature`) disables it behind an upgrade badge,
	 *  the way the export options gate formats; the picker's `onDisabledClick` opens the upgrade. */
	export type CredentialTypeDisplayContext = {
		disabledReason?: string | null;
		upgradeFeature?: FeatureId | null;
	};

	export const CredentialTypeDisplay: EntityDisplayComponent<
		CredentialTypeOption,
		CredentialTypeDisplayContext
	> = {
		getId: (item) => item.id,
		getLabel: (item) => credentialTypes.getName(item.id),
		getDescription: (item) => credentialTypes.getDescription(item.id),
		getIcon: (item) => credentialTypes.getIconComponent(item.id),
		getIconColor: (item) => credentialTypes.getColorHelper(item.id).icon,
		getCategory: (item) => item.category ?? null,
		// The upgrade badge leads, then Beta, then the unofficial-API marker, then the target tags.
		// This is the one place credential-type tags are built, so it covers the wizard card grid,
		// the type dropdown, and dropdown search (RichSelect folds tag labels into its filter text)
		// in a single definition. Beta and unofficial are independent, so a type may carry both.
		getTags: (item, context) => {
			const upgrade = context?.upgradeFeature ? upgradeTagProps() : null;
			const stability = getStabilityTagProps(item.metadata?.stability);
			const upstream = getUpstreamSupportTagProps(item.metadata?.upstream_support);
			const targets = (item.metadata?.targets ?? []).map((t: string) => getTargetTagProps(t));
			return [upgrade, stability, upstream, ...targets].filter((tag) => tag !== null);
		},
		getDisabled: (_item, context) => !!context?.disabledReason || !!context?.upgradeFeature,
		getDisabledReason: (_item, context) =>
			context?.disabledReason ?? (context?.upgradeFeature ? common_availableOnPaidPlans() : null)
	};
</script>

<script lang="ts">
	import type { EntityDisplayComponent } from '../types';
	import ListSelectItem from '../ListSelectItem.svelte';

	interface Props {
		item: CredentialTypeOption;
		context?: object;
	}

	let { item, context = {} }: Props = $props();
</script>

<ListSelectItem {item} {context} displayComponent={CredentialTypeDisplay} />
