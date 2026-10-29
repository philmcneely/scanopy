<!--
	The device facts a person can type for a device no scan identified: what it is, who made it,
	where it is. Each value a person types is stored as `Manual`, which no scan replaces; a value
	sent back unchanged keeps the source that produced it, so saving a description does not freeze
	a discovered model. Clearing a value hands it back to discovery.

	Labels come from the entity field registry, the names every other surface uses for these
	facts. Identifiers discovery matches on (chassis id, sysName, sysObjectID) and device-reported
	revisions stay read-only in the facts card beside this.
-->
<script lang="ts">
	import type { AnyFieldApi } from '@tanstack/svelte-form';
	import type { HostFormData } from '$lib/features/hosts/types/base';
	import TextInput from '$lib/shared/components/forms/input/TextInput.svelte';
	import SelectInput from '$lib/shared/components/forms/input/SelectInput.svelte';
	import AttributeSourceTag from '$lib/shared/components/data/AttributeSourceTag.svelte';
	import Tag from '$lib/shared/components/data/Tag.svelte';
	import { deviceTypeSourceTag } from '$lib/features/hosts/device-type-source';
	import { deviceTypes } from '$lib/shared/stores/metadata';
	import { entityFieldLabel, entityFields, fieldSource } from '$lib/shared/entity-fields';
	import { max } from '$lib/shared/components/forms/validators';
	import { EDITABLE_TEXT_FACTS } from './device-labels';
	import {
		hosts_deviceLabels,
		hosts_deviceLabelsHelp,
		hosts_deviceType
	} from '$lib/paraglide/messages';

	interface Props {
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		form: { Field: any };
		formData: HostFormData;
	}

	let { form, formData }: Props = $props();

	const HOST_FIELDS = new Map(entityFields('Host').map((field) => [field.id, field]));
	const textFacts = EDITABLE_TEXT_FACTS.flatMap((id) => {
		const field = HOST_FIELDS.get(id);
		return field ? [{ id, label: entityFieldLabel('Host', field), field }] : [];
	});

	let deviceTypeOptions = $derived(
		deviceTypes.getItems().map((item) => ({ value: item.id, label: deviceTypes.getName(item.id) }))
	);
</script>

<div class="space-y-4">
	<div>
		<h4 class="text-xs font-semibold tracking-wide text-secondary uppercase">
			{hosts_deviceLabels()}
		</h4>
		<p class="text-xs text-secondary">{hosts_deviceLabelsHelp()}</p>
	</div>

	<div class="flex items-end gap-2">
		<div class="min-w-0 flex-1">
			<form.Field name="device_type">
				{#snippet children(field: AnyFieldApi)}
					<SelectInput
						label={hosts_deviceType()}
						id="device_type"
						{field}
						options={deviceTypeOptions}
					/>
				{/snippet}
			</form.Field>
		</div>
		{#if formData.device_type_source}
			<span class="shrink-0 pb-2"
				><Tag {...deviceTypeSourceTag(formData.device_type_source)} /></span
			>
		{/if}
	</div>

	{#each textFacts as fact (fact.id)}
		<div class="flex items-end gap-2">
			<div class="min-w-0 flex-1">
				<form.Field
					name={fact.id}
					validators={{ onBlur: ({ value }: { value: string }) => max(200)(value) }}
				>
					{#snippet children(field: AnyFieldApi)}
						<TextInput label={fact.label} id={fact.id} {field} />
					{/snippet}
				</form.Field>
			</div>
			<span class="shrink-0 pb-2">
				<AttributeSourceTag source={fieldSource(fact.field, formData)} />
			</span>
		</div>
	{/each}
</div>
