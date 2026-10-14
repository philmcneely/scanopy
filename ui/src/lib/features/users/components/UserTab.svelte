<script lang="ts">
	import CreateButton from '$lib/shared/components/data/controls/CreateButton.svelte';
	import { useSitesQuery } from '$lib/features/sites/queries';
	import { userColumns } from './UserTab.columns';
	import { Edit, UserX, Trash2 } from 'lucide-svelte';
	import type { CardAction } from '$lib/shared/components/data/types';
	import Loading from '$lib/shared/components/feedback/Loading.svelte';
	import EmptyState from '$lib/shared/components/layout/EmptyState.svelte';
	import DataControls from '$lib/shared/components/data/DataControls.svelte';
	import type { TableDefaults } from '$lib/shared/components/data/types';
	import { formatInviteUrl, useRevokeInviteMutation } from '$lib/features/organizations/queries';
	import { isUser, type User, type UserOrInvite } from '../types';
	import type { OrganizationInvite } from '$lib/features/organizations/types';
	import InviteModal from './InviteModal.svelte';
	import { metadata, permissions } from '$lib/shared/stores/metadata';
	import { useOrganizationQuery } from '$lib/features/organizations/queries';
	import UpgradeButton from '$lib/shared/components/UpgradeButton.svelte';
	import UserEditModal from './UserEditModal.svelte';
	import { useCurrentUserQuery } from '$lib/features/auth/queries';
	import {
		useMembersQuery,
		useMembersListQuery,
		useBulkDeleteUsersMutation,
		useDeleteUserMutation
	} from '../queries';
	import { listPageState } from '$lib/shared/components/data/listPage.svelte';
	import type { components } from '$lib/api/schema';

	type MemberOrderField = components['schemas']['MemberOrderField'];
	import type { TabProps } from '$lib/shared/types';
	import { downloadCsv } from '$lib/shared/utils/csvExport';
	import { modalState, resolveModalDeepLink } from '$lib/shared/stores/modal-registry';
	import {
		common_edit,
		common_confirmBulkDelete,
		common_delete,
		common_revoke,
		common_users,
		invites_confirmRevoke,
		users_confirmDeleteUser,
		users_inviteUser,
		users_noUsersFound,
		users_noUsersSubtitle,
		users_subtitle
	} from '$lib/paraglide/messages';

	let { isReadOnly = false }: TabProps = $props();

	// Query
	const currentUserQuery = useCurrentUserQuery();
	let currentUser = $derived(currentUserQuery.data);

	const organizationQuery = useOrganizationQuery();
	let org = $derived(organizationQuery.data);
	let seatLimit = $derived(org?.plan?.included_seats ?? null);
	let canBuyMoreSeats = $derived(
		org?.plan?.seat_cents !== undefined && org?.plan?.seat_cents !== null
	);

	// Every member the caller may see, for the seat count, filter options and deep links; the table
	// reads its own page.
	const membersQuery = useMembersQuery();
	const page = listPageState();
	const membersPageQuery = useMembersListQuery(() => page.params());
	const bulkDeleteUsersMutation = useBulkDeleteUsersMutation();

	// Derived data
	let membersData = $derived(membersQuery.data ?? []);
	let usersData = $derived(membersData.filter(isUser).map((member) => member.data));
	let pageMembers = $derived(membersPageQuery.data?.items ?? []);
	let membersPagination = $derived(membersPageQuery.data?.pagination ?? null);
	let isLoading = $derived(
		membersQuery.isPending || (membersPageQuery.isPending && !membersPageQuery.data)
	);

	// Force Svelte to track metadata reactivity
	$effect(() => {
		void $metadata;
	});

	let showInviteModal = $state(false);
	let showEditModal = $state(false);
	let editingUser = $state<User | null>(null);

	// Deep-link: open invite modal from URL
	$effect(() => {
		if ($modalState.name === 'invite-user' && !showInviteModal) {
			showInviteModal = true;
		}
	});

	// Deep-link: open user editor from URL (handles both fresh open and entity switch)
	$effect(() => {
		const result = resolveModalDeepLink(
			$modalState,
			'user-editor',
			usersData,
			showEditModal,
			editingUser?.id
		);
		if (result !== undefined) {
			editingUser = result;
			showEditModal = true;
		}
	});

	let userCount = $derived(usersData.length);
	let isAtSeatLimit = $derived(seatLimit !== null && userCount >= seatLimit && !canBuyMoreSeats);

	async function handleCreateInvite() {
		showInviteModal = true;
	}

	function handleCloseInviteModal() {
		showInviteModal = false;
	}

	// Check if user can invite
	let canInviteUsers = $derived(
		!isReadOnly && currentUser
			? (permissions.getMetadata(currentUser.permissions)?.grantable_user_permissions?.length ??
					0) > 0
			: false
	);

	async function handleBulkDelete(ids: string[]) {
		if (confirm(common_confirmBulkDelete({ count: ids.length, entity: common_users() }))) {
			await bulkDeleteUsersMutation.mutateAsync(ids);
		}
	}

	function handleEditUser(user: User) {
		editingUser = user;
		showEditModal = true;
	}

	function handleCloseEditModal() {
		showEditModal = false;
		editingUser = null;
	}

	// CSV export handler (exports users only, not invites)
	async function handleCsvExport() {
		await downloadCsv('User', {});
	}

	// Only define fields for users (invites won't be filtered/sorted)
	const sitesQuery = useSitesQuery();
	let sitesData = $derived(sitesQuery.data ?? []);

	const revokeInviteMutation = useRevokeInviteMutation();

	/** Whether the current user is allowed to grant — and so revoke — this invite. */
	function canRevoke(invite: OrganizationInvite): boolean {
		return currentUser
			? (permissions
					.getMetadata(currentUser.permissions)
					?.grantable_user_permissions?.includes(invite.permissions) ?? false)
			: false;
	}

	function handleRevokeInvite(invite: OrganizationInvite) {
		if (confirm(invites_confirmRevoke())) {
			revokeInviteMutation.mutate(invite.id);
		}
	}

	const deleteUserMutation = useDeleteUserMutation();

	function handleDeleteUser(user: User) {
		if (confirm(users_confirmDeleteUser())) {
			deleteUserMutation.mutate(user.id);
		}
	}

	/** Whether the current user may manage the given user's account. */
	function canManageUser(user: User): boolean {
		return currentUser
			? (permissions
					.getMetadata(currentUser.permissions)
					?.grantable_user_permissions?.includes(user.permissions) ?? false)
			: false;
	}

	/** Row actions. A user is edited or deleted; an invite is revoked. */
	function userActions(item: UserOrInvite): CardAction[] {
		if (isUser(item)) {
			const actions: CardAction[] = [
				{ label: common_edit(), icon: Edit, onClick: () => handleEditUser(item.data) }
			];
			// You cannot delete yourself, and you cannot delete someone whose role
			// you could not have granted in the first place.
			if (item.data.id !== currentUser?.id && canManageUser(item.data)) {
				actions.push({
					label: common_delete(),
					icon: Trash2,
					class: 'btn-icon-danger',
					onClick: () => handleDeleteUser(item.data)
				});
			}
			return actions;
		}
		if (!canRevoke(item.data)) return [];
		return [
			{
				label: common_revoke(),
				icon: UserX,
				class: 'btn-icon-danger',
				onClick: () => handleRevokeInvite(item.data)
			}
		];
	}

	const tableDefaults: TableDefaults<MemberOrderField> = {
		sort: { field: 'email', direction: 'asc' }
	};

	let userFields = $derived(
		userColumns({
			members: () => membersData,
			sites: () => sitesData,
			inviteUrl: formatInviteUrl
		})
	);
</script>

<!-- The page's own actions, last in the table toolbar beside the filter and columns. -->
{#snippet toolbarActions()}
	<div class="flex items-center gap-3">
		{#if seatLimit !== null && !canBuyMoreSeats}
			<span class="text-sm {isAtSeatLimit ? 'text-amber-400' : 'text-tertiary'}">
				{userCount} / {seatLimit}
			</span>
		{/if}
		{#if canInviteUsers}
			{#if isAtSeatLimit}
				<UpgradeButton feature="seats" surface="users_tab" gate_type="limit_hit" />
			{:else}
				<CreateButton onclick={handleCreateInvite} label={users_inviteUser()} />
			{/if}
		{/if}
	</div>
{/snippet}

<div class="space-y-6">
	<!-- Loading state -->
	{#if isLoading}
		<Loading />
	{:else if pageMembers.length === 0 && !page.isNarrowed}
		<!-- Empty state -->
		<EmptyState title={users_noUsersFound()} subtitle={users_noUsersSubtitle()} />
	{:else}
		<DataControls
			title={common_users()}
			subtitle={users_subtitle()}
			{toolbarActions}
			items={pageMembers}
			fields={userFields}
			storageKey="scanopy-users-table-state"
			defaults={tableDefaults}
			onBulkDelete={handleBulkDelete}
			getItemId={(item) => item.id}
			serverPagination={membersPagination}
			onPageChange={page.onPageChange}
			onQueryChange={page.onQueryChange}
			serverQuery={{ stale: false, tags: false }}
			getActions={userActions}
			onCsvExport={handleCsvExport}
		></DataControls>
	{/if}
</div>

<InviteModal name="invite-user" isOpen={showInviteModal} onClose={handleCloseInviteModal} />
<UserEditModal
	name="user-editor"
	isOpen={showEditModal}
	user={editingUser}
	onClose={handleCloseEditModal}
/>
