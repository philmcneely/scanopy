<script lang="ts">
	import type { AnyFieldApi } from '@tanstack/svelte-form';
	import type { HostFormData } from '$lib/features/hosts/types/base';
	import { max } from '$lib/shared/components/forms/validators';
	import TextArea from '$lib/shared/components/forms/input/TextArea.svelte';
	import SelectSite from '$lib/features/sites/components/SelectSite.svelte';
	import TagPickerInline from '$lib/features/tags/components/TagPickerInline.svelte';
	import IdentitySection from './IdentitySection.svelte';
	import DeviceFactsSection from './DeviceFactsSection.svelte';
	import DeviceLabelFields from './DeviceLabelFields.svelte';
	import {
		common_tags,
		common_description,
		hosts_details_descriptionPlaceholder
	} from '$lib/paraglide/messages';

	interface Props {
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		form: { Field: any };
		formData: HostFormData;
		isEditing?: boolean;
	}

	let { form, formData = $bindable(), isEditing = false }: Props = $props();

	// site_id is read/written directly against formData — no local
	// snapshot. A prior `$state(formData.site_id)` mirror captured the
	// value once at mount, went stale when HostEditor reassigned formData
	// via resetForm(host), and then got clobbered by SelectSite's
	// auto-default (first site) on the falsy initial capture.
</script>

<!-- Side by side when the device facts card renders (editing a host that has facts), so it and the
     metadata below stay in view. With one card it keeps the full width. -->
<div class="grid grid-cols-1 items-start gap-6 p-6 lg:has-[>:nth-child(2)]:grid-cols-2">
	<div class="card-static card space-y-6">
		<IdentitySection {form} {formData} {isEditing} />

		<!-- Create only: an update keeps the host's existing site whatever the request says. -->
		{#if !isEditing}
			<SelectSite
				selectedSiteId={formData.site_id}
				onSiteChange={(id) => (formData.site_id = id)}
			/>
		{/if}

		<form.Field
			name="description"
			validators={{
				onBlur: ({ value }: { value: string }) => max(500)(value)
			}}
		>
			{#snippet children(field: AnyFieldApi)}
				<TextArea
					label={common_description()}
					id="description"
					placeholder={hosts_details_descriptionPlaceholder()}
					{field}
				/>
			{/snippet}
		</form.Field>

		<TagPickerInline
			entityType="Host"
			label={common_tags()}
			selectedTagIds={formData.tags}
			onChange={(tags) => (formData.tags = tags)}
		/>

		{#if isEditing}
			<DeviceLabelFields {form} {formData} />
		{/if}
	</div>

	{#if isEditing}
		<DeviceFactsSection host={formData} />
	{/if}
</div>
