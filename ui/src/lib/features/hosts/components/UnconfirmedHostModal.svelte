<!--
	What to do about a host Scanopy knows only from another device's report: one an LLDP/CDP
	neighbour advertised, or a MAC a switch learned on one of its ports.

	The same layout as resolving an assumed subnet range (`ConfirmEntityModal`), so an unconfirmed
	host and an unconfirmed subnet are settled the same way. The actions hand off to the flows that
	already own them: the host editor for labelling, the merge modal for a merge.
-->
<script lang="ts">
	import { Check, EyeOff, Replace } from '@lucide/svelte';

	import ConfirmEntityModal, {
		type ConfirmAction
	} from '$lib/shared/components/confirmation/ConfirmEntityModal.svelte';
	import { unconfirmedReason, unconfirmedVia } from '$lib/shared/utils/confirmation';
	import type { Host } from '../types/base';
	import {
		hosts_confirmAndLabel,
		hosts_confirmAndLabelDetail,
		common_dismiss,
		hosts_dismissUnconfirmedDetail,
		hosts_mergeUnconfirmed,
		hosts_mergeUnconfirmedDetail,
		hosts_resolveUnconfirmed,
		common_reportedBy,
		hosts_unconfirmedIdentity
	} from '$lib/paraglide/messages';
	import EntityTag from '$lib/shared/components/data/EntityTag.svelte';
	import { entityRef } from '$lib/shared/components/data/types';
	import { entities } from '$lib/shared/stores/metadata';
	import { hostDisplayName } from '$lib/features/hosts/host-display-name';
	import { useHostsByIds } from '../queries';

	interface Props {
		/** A listed host, which carries its interfaces: a learned MAC is on one. */
		host:
			| (Host & {
					interfaces?: { mac_address?: string | null }[];
					reported_by_host_ids?: string[];
			  })
			| null;
		isOpen?: boolean;
		/** Confirms the host, then opens the host editor on it. Owned by the caller. */
		onConfirmAndLabel: (host: Host) => Promise<void> | void;
		/** Opens the merge modal with this host as the one merged away. */
		onMerge: (host: Host) => void;
		onDismiss: (host: Host) => Promise<void> | void;
		onClose: () => void;
	}

	let { host, isOpen = false, onConfirmAndLabel, onMerge, onDismiss, onClose }: Props = $props();

	let via = $derived(host ? unconfirmedVia(host) : null);

	/** What the host is known by: the chassis id a neighbour advertised, or the learned MAC. */
	let identifiers = $derived.by(() => {
		if (!host) return [];
		const values = new Set<string>();
		if (host.chassis_id) values.add(host.chassis_id);
		if (host.sys_name) values.add(host.sys_name);
		for (const iface of host.interfaces ?? []) {
			if (iface.mac_address) values.add(iface.mac_address);
		}
		return [...values];
	});

	// The hosts whose report this one is known from: the switch that learned its MAC, or the
	// neighbour that advertised it, as entity links the way an assumed range names its reporters.
	const reportersQuery = useHostsByIds(() => host?.reported_by_host_ids ?? []);
	let reporters = $derived(reportersQuery.data ?? []);

	let intro = $derived(
		via
			? [
					unconfirmedReason(via),
					identifiers.length
						? hosts_unconfirmedIdentity({ identifiers: identifiers.join(', ') })
						: ''
				]
					.filter(Boolean)
					.join(' ')
			: ''
	);

	let actions = $derived.by<ConfirmAction[]>(() => {
		if (!host) return [];
		const current = host;
		return [
			{
				id: 'confirm',
				title: hosts_confirmAndLabel(),
				description: hosts_confirmAndLabelDetail(),
				icon: Check,
				color: 'Green',
				run: () => void onConfirmAndLabel(current)
			},
			{
				id: 'merge',
				title: hosts_mergeUnconfirmed(),
				description: hosts_mergeUnconfirmedDetail(),
				icon: Replace,
				color: 'Violet',
				run: () => onMerge(current)
			},
			{
				id: 'dismiss',
				title: common_dismiss(),
				description: hosts_dismissUnconfirmedDetail(),
				icon: EyeOff,
				color: 'Gray',
				run: () => void onDismiss(current)
			}
		];
	});
</script>

<ConfirmEntityModal
	name="unconfirmed-host"
	entityId={host?.id}
	title={hosts_resolveUnconfirmed()}
	{intro}
	{actions}
	{isOpen}
	{onClose}
>
	{#snippet evidence()}
		{#if reporters.length > 0}
			<div class="flex flex-wrap items-center gap-2 text-sm">
				<span class="text-secondary">{common_reportedBy()}</span>
				{#each reporters as reporter (reporter.id)}
					<EntityTag
						entityRef={entityRef('Host', reporter.id, reporter)}
						label={hostDisplayName(reporter)}
						color={entities.getColorHelper('Host').color}
					/>
				{/each}
			</div>
		{/if}
	{/snippet}
</ConfirmEntityModal>
