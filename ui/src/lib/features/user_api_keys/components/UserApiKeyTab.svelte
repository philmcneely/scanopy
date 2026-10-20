<script lang="ts">
	import CreateButton from '$lib/shared/components/data/controls/CreateButton.svelte';
	import { Edit, Trash2 } from 'lucide-svelte';
	import type { CardAction } from '$lib/shared/components/data/types';
	import Loading from '$lib/shared/components/feedback/Loading.svelte';
	import EmptyState from '$lib/shared/components/layout/EmptyState.svelte';
	import DataControls from '$lib/shared/components/data/DataControls.svelte';
	import type { TableDefaults } from '$lib/shared/components/data/types';
	import { userApiKeyColumns } from './UserApiKeyTab.columns';
	import { useTagsQuery } from '$lib/features/tags/queries';
	import { useSitesQuery } from '$lib/features/sites/queries';
	import UserApiKeyModal from './UserApiKeyModal.svelte';
	import {
		useUserApiKeysListQuery,
		useUpdateUserApiKeyMutation,
		useDeleteUserApiKeyMutation,
		useBulkDeleteUserApiKeysMutation,
		type UserApiKey
	} from '../queries';
	import type { TabProps } from '$lib/shared/types';
	import { downloadCsv } from '$lib/shared/utils/csvExport';
	import { listPageState } from '$lib/shared/components/data/listPage.svelte';

	import {
		common_edit,
		common_delete,
		common_apiKeys,
		common_confirmBulkDelete,
		common_confirmDeleteName,
		common_create,
		userApiKeys_apiAccessUnavailableSubtitle,
		userApiKeys_apiAccessUnavailableTitle,
		userApiKeys_noApiKeysSubtitle,
		userApiKeys_noApiKeysYet,
		userApiKeys_subtitle
	} from '$lib/paraglide/messages';
	import { useOrganizationQuery } from '$lib/features/organizations/queries';
	import { billingPlans } from '$lib/shared/stores/metadata';
	import UpgradeButton from '$lib/shared/components/UpgradeButton.svelte';
	import { modalState, resolveModalDeepLink } from '$lib/shared/stores/modal-registry';
	import type { components } from '$lib/api/schema';

	type UserApiKeyOrderField = components['schemas']['UserApiKeyOrderField'];

	let { isReadOnly = false, isActive = false }: TabProps = $props();

	// Check if plan has api_access feature before querying
	const organizationQuery = useOrganizationQuery();
	let hasApiAccess = $derived.by(() => {
		const org = organizationQuery.data;
		if (!org?.plan) return false;
		return billingPlans.getMetadata(org.plan.type).features.api_access;
	});

	// Queries
	const tagsQuery = useTagsQuery();
	const page = listPageState(() => isActive);
	const userApiKeysQuery = useUserApiKeysListQuery(() => page.params(hasApiAccess));
	const sitesQuery = useSitesQuery();

	// Mutations
	const updateMutation = useUpdateUserApiKeyMutation();
	const deleteMutation = useDeleteUserApiKeyMutation();
	const bulkDeleteMutation = useBulkDeleteUserApiKeysMutation();

	// Derived data
	let tagsData = $derived(tagsQuery.data ?? []);
	let userApiKeysData = $derived(userApiKeysQuery.data?.items ?? []);
	let userApiKeysPagination = $derived(userApiKeysQuery.data?.pagination ?? null);
	let sitesData = $derived(sitesQuery.data ?? []);
	let isLoading = $derived(userApiKeysQuery.isPending && !userApiKeysQuery.data);

	let showModal = $state(false);
	let editingApiKey = $state<UserApiKey | null>(null);

	// Deep-link: open user API key editor from URL (handles both fresh open and entity switch)
	$effect(() => {
		const result = resolveModalDeepLink(
			$modalState,
			'user-api-key',
			userApiKeysData,
			showModal,
			editingApiKey?.id
		);
		if (result !== undefined) {
			editingApiKey = result;
			showModal = true;
		}
	});

	async function handleDelete(apiKey: UserApiKey) {
		if (confirm(common_confirmDeleteName({ name: apiKey.name }))) {
			deleteMutation.mutate(apiKey.id);
		}
	}

	async function handleUpdate(apiKey: UserApiKey) {
		await updateMutation.mutateAsync(apiKey);
		showModal = false;
		editingApiKey = null;
	}

	function handleCreate() {
		showModal = true;
		editingApiKey = null;
	}

	function handleClose() {
		showModal = false;
		editingApiKey = null;
	}

	function handleEdit(apiKey: UserApiKey) {
		showModal = true;
		editingApiKey = apiKey;
	}

	async function handleBulkDelete(ids: string[]) {
		if (confirm(common_confirmBulkDelete({ count: ids.length, entity: common_apiKeys() }))) {
			await bulkDeleteMutation.mutateAsync(ids);
		}
	}

	function getUserApiKeyTags(apiKey: UserApiKey): string[] {
		return apiKey.tags ?? [];
	}

	// CSV export handler
	async function handleCsvExport() {
		await downloadCsv('UserApiKey', page.listQuery);
	}

	/** Row actions. */
	function userApiKeyActions(apiKey: UserApiKey): CardAction[] {
		return [
			{ label: common_edit(), icon: Edit, isOpen: true, onClick: () => handleEdit(apiKey) },
			{
				label: common_delete(),
				icon: Trash2,
				tone: 'danger',
				isDelete: true,
				onBulk: isReadOnly ? undefined : handleBulkDelete,
				onClick: () => handleDelete(apiKey)
			}
		];
	}

	const tableDefaults: TableDefaults<UserApiKeyOrderField> = {
		sort: { field: 'name', direction: 'asc' }
	};

	let apiKeyFields = $derived(
		userApiKeyColumns({
			sites: () => sitesData,
			tags: () => tagsData
		})
	);
</script>

<!-- The page's own actions, last in the table toolbar beside the filter and columns. -->
{#snippet toolbarActions()}
	{#if !isReadOnly && hasApiAccess}
		<CreateButton onclick={handleCreate} />
	{/if}
{/snippet}

<div class="space-y-6">
	{#if !hasApiAccess}
		<EmptyState
			title={userApiKeys_apiAccessUnavailableTitle()}
			subtitle={userApiKeys_apiAccessUnavailableSubtitle()}
		>
			<UpgradeButton feature="api_access" surface="api_keys_tab" />
		</EmptyState>
	{:else if isLoading}
		<Loading />
	{:else if page.isEmpty(userApiKeysQuery)}
		<EmptyState
			title={userApiKeys_noApiKeysYet()}
			subtitle={userApiKeys_noApiKeysSubtitle()}
			onClick={handleCreate}
			cta={common_create()}
		/>
	{:else}
		<DataControls
			title={common_apiKeys()}
			subtitle={userApiKeys_subtitle()}
			{toolbarActions}
			items={userApiKeysData}
			fields={apiKeyFields}
			entityType="UserApiKey"
			canEditTags={!isReadOnly}
			getItemTags={getUserApiKeyTags}
			storageKey="scanopy-user-api-keys-table-state"
			defaults={tableDefaults}
			getItemId={(item) => item.id}
			serverPagination={userApiKeysPagination}
			onPageChange={page.onPageChange}
			onQueryChange={page.onQueryChange}
			serverQuery={{ stale: false }}
			getActions={userApiKeyActions}
			onCsvExport={handleCsvExport}
		></DataControls>
	{/if}
</div>

<UserApiKeyModal
	name="user-api-key"
	isOpen={showModal}
	onClose={handleClose}
	onUpdate={handleUpdate}
	apiKey={editingApiKey}
/>
