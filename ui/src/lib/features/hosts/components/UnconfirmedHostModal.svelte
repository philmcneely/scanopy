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
		hosts_unconfirmedChassisId,
		hosts_unconfirmedMac,
		hosts_unconfirmedMacs,
		hosts_unconfirmedSysName
	} from '$lib/paraglide/messages';

	interface Props {
		/** A listed host, which carries its interfaces: a learned MAC is on one. */
		host:
			| (Host & {
					interfaces?: { mac_address?: string | null }[];
					reported_by_host_ids?: string[];
					found_by_discovery_id?: string | null;
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

	/** What the host is known by, each named for what it is: what it advertised, or its MAC. */
	let identity = $derived.by(() => {
		if (!host) return [];
		const sentences: string[] = [];
		if (host.chassis_id) sentences.push(hosts_unconfirmedChassisId({ id: host.chassis_id }));
		if (host.sys_name && host.sys_name !== host.chassis_id) {
			sentences.push(hosts_unconfirmedSysName({ name: host.sys_name }));
		}
		const macs = [
			...new Set((host.interfaces ?? []).flatMap((i) => (i.mac_address ? [i.mac_address] : [])))
		];
		if (macs.length === 1) sentences.push(hosts_unconfirmedMac({ mac: macs[0] }));
		if (macs.length > 1) sentences.push(hosts_unconfirmedMacs({ macs: macs.join(', ') }));
		return sentences;
	});

	let intro = $derived(via ? [unconfirmedReason(via), ...identity].join(' ') : '');

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
	reportedByHostIds={host?.reported_by_host_ids ?? []}
	sourceDiscoveryId={host?.found_by_discovery_id ?? null}
/>
