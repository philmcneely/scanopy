<script lang="ts">
	import { formatDate } from '$lib/shared/utils/formatting';
	import { useCurrentUserQuery, useLogoutMutation } from '$lib/features/auth/queries';
	import { useQueryClient } from '@tanstack/svelte-query';
	import { queryKeys } from '$lib/api/query-client';
	import { apiClient } from '$lib/api/client';
	import type { User } from '$lib/features/users/types';
	import { pushError, pushSuccess } from '$lib/shared/stores/feedback';
	import { Link, Key, LogOut, Mail, Shield } from 'lucide-svelte';
	import CookieSettingsPanel from '$lib/shared/components/feedback/CookieSettingsPanel.svelte';
	import { createForm } from '@tanstack/svelte-form';
	import { submitForm } from '$lib/shared/components/forms/form-context';
	import {
		required,
		email as emailValidator,
		password as passwordValidator,
		confirmPasswordMatch
	} from '$lib/shared/components/forms/validators';
	import InfoCard from '$lib/shared/components/data/InfoCard.svelte';
	import OidcProviderLogo from '$lib/features/auth/components/OidcProviderLogo.svelte';
	import TextInput from '$lib/shared/components/forms/input/TextInput.svelte';
	import Password from '$lib/shared/components/forms/input/Password.svelte';
	import { useConfigQuery } from '$lib/shared/stores/config-query';
	import type { ModalAction } from '$lib/shared/components/layout/modalActions';
	import { useModalActions } from '$lib/shared/components/layout/modalActionsContext.svelte';
	import {
		common_back,
		common_currentPassword,
		common_email,
		common_emailAndPassword,
		common_password,
		common_link,
		common_logout,
		common_redirecting,
		common_saveChanges,
		common_saving,
		common_unlink,
		settings_account_credentialsUpdated,
		settings_account_currentPasswordPlaceholder,
		settings_account_emailChangeIntro,
		settings_account_emailChangeIntroImmediate,
		settings_account_emailChanged,
		settings_account_enterEmail,
		settings_account_linkedOn,
		settings_account_loadingUser,
		settings_account_noChanges,
		settings_account_notLinked,
		settings_account_oidcLinkHelp,
		settings_account_oidcUnlinked,
		settings_account_setPassword,
		settings_account_signOut,
		settings_account_unlinkFirst,
		settings_account_updateCredentials,
		settings_account_updateEmail,
		settings_account_updatePassword,
		settings_account_identityProviders,
		settings_account_verificationSentTo,
		settings_account_cookiePreferencesDesc,
		cookies_preferences,
		common_manage
	} from '$lib/paraglide/messages';

	let {
		subView = $bindable<'main' | 'credentials' | 'email-change' | 'cookies'>('main'),
		onClose
	}: {
		subView?: 'main' | 'credentials' | 'email-change' | 'cookies';
		onClose: () => void;
	} = $props();

	// TanStack Query for current user
	const currentUserQuery = useCurrentUserQuery();
	const logoutMutation = useLogoutMutation();
	const queryClient = useQueryClient();

	let user = $derived(currentUserQuery.data);

	const configQuery = useConfigQuery();
	let configData = $derived(configQuery.data);

	let oidcProviders = $derived(configData?.oidc_providers ?? []);
	let hasOidcProviders = $derived(oidcProviders.length > 0);

	let linkingProviderSlug: string | null = $state(null);
	let savingCredentials = $state(false);
	let emailChangeLoading = $state(false);

	// Create form for password update
	const form = createForm(() => ({
		defaultValues: { currentPassword: '', password: '', confirmPassword: '' },
		onSubmit: async ({ value }) => {
			savingCredentials = true;
			try {
				if (!value.password) {
					pushError(settings_account_noChanges());
					return;
				}

				const body: { current_password?: string; new_password: string } = {
					new_password: value.password
				};

				if (value.currentPassword) {
					body.current_password = value.currentPassword;
				}

				const { data } = await apiClient.POST('/api/auth/update', {
					body
				});

				if (data?.success && data.data) {
					queryClient.setQueryData<User>(queryKeys.auth.currentUser(), data.data);
					pushSuccess(settings_account_credentialsUpdated());
					subView = 'main';
				}
			} finally {
				savingCredentials = false;
			}
		}
	}));

	// Create form for email change
	const emailChangeForm = createForm(() => ({
		defaultValues: { currentPassword: '', newEmail: '' },
		onSubmit: async ({ value }) => {
			const newEmail = value.newEmail.trim();
			if (!newEmail) return;
			emailChangeLoading = true;
			try {
				const { data } = await apiClient.POST('/api/auth/request-email-change', {
					body: { current_password: value.currentPassword, new_email: newEmail }
				});
				if (data?.success) {
					// A server with no email transport applies the change at once instead of
					// mailing a link to the new address.
					if (configData?.has_email_service) {
						pushSuccess(settings_account_verificationSentTo({ email: newEmail }));
					} else {
						pushSuccess(settings_account_emailChanged({ email: newEmail }));
						await queryClient.invalidateQueries({ queryKey: queryKeys.auth.currentUser() });
					}
					emailChangeForm.reset({ currentPassword: '', newEmail: '' });
					subView = 'main';
				}
			} finally {
				emailChangeLoading = false;
			}
		}
	}));

	// Reset form when switching to credentials view
	export function resetForm() {
		linkingProviderSlug = null;
		form.reset({ currentPassword: '', password: '', confirmPassword: '' });
		emailChangeForm.reset({ currentPassword: '', newEmail: '' });
	}

	// Find which provider (if any) is linked to this user
	let linkedProvider = $derived(
		user?.oidc_provider ? oidcProviders.find((p) => p.slug === user.oidc_provider) : null
	);

	function linkOidcAccount(providerSlug: string) {
		linkingProviderSlug = providerSlug;
		const returnUrl = encodeURIComponent(window.location.origin);
		window.location.href = `/api/auth/oidc/${providerSlug}/authorize?flow=link&return_url=${returnUrl}`;
	}

	async function unlinkOidcAccount(providerSlug: string) {
		const { data } = await apiClient.POST('/api/auth/oidc/{slug}/unlink', {
			params: { path: { slug: providerSlug } }
		});

		if (data?.success && data.data) {
			queryClient.setQueryData<User>(queryKeys.auth.currentUser(), data.data);
			pushSuccess(settings_account_oidcUnlinked());
		}
	}

	async function handleSubmit() {
		if (subView === 'email-change') {
			await submitForm(emailChangeForm);
		} else {
			await submitForm(form);
		}
	}

	function handleBack() {
		subView = 'main';
		form.reset({ currentPassword: '', password: '', confirmPassword: '' });
		emailChangeForm.reset({ currentPassword: '', newEmail: '' });
	}

	async function handleLogout() {
		try {
			await logoutMutation.mutateAsync();
			window.location.reload();
			onClose();
		} catch {
			// Error handled by mutation
		}
	}

	let hasLinkedOidc = $derived(!!user?.oidc_provider);
	let showSave = $derived(subView === 'credentials' || subView === 'email-change');
	let isSaving = $derived(subView === 'email-change' ? emailChangeLoading : savingCredentials);

	// Only the sub-views have a footer; the main view closes from the header.
	let actions = $derived<ModalAction[]>(
		subView !== 'main'
			? [
					{ role: 'secondary', label: common_back(), onClick: handleBack },
					...(showSave
						? [
								{
									role: 'primary' as const,
									label: isSaving ? common_saving() : common_saveChanges(),
									onClick: handleSubmit,
									disabled: isSaving
								}
							]
						: [])
				]
			: []
	);
	useModalActions(() => actions);
</script>

<form
	onsubmit={(e) => {
		e.preventDefault();
		e.stopPropagation();
		if (subView === 'email-change' || subView === 'credentials') handleSubmit();
	}}
	class="flex min-h-0 flex-1 flex-col"
>
	<div class="flex-1 overflow-auto p-6">
		{#if subView === 'main'}
			{#if user}
				<div class="space-y-6">
					<!-- Email & Password. A linked OIDC account owns the address, so it can't be changed here. -->
					<InfoCard title={common_emailAndPassword()}>
						<div class="flex flex-wrap items-center justify-between gap-4">
							<div class="flex min-w-0 items-center gap-4">
								<Mail class="h-5 w-5 shrink-0 text-secondary" />
								<div class="min-w-0">
									<p class="text-sm font-medium text-primary">{common_email()}</p>
									<p class="truncate text-xs text-secondary">{user.email}</p>
								</div>
							</div>
							{#if !hasLinkedOidc}
								<button
									type="button"
									onclick={() => {
										emailChangeForm.reset({ currentPassword: '', newEmail: '' });
										subView = 'email-change';
									}}
									class="btn-primary whitespace-nowrap"
								>
									{settings_account_updateEmail()}
								</button>
							{/if}
						</div>
						<div class="flex flex-wrap items-center justify-between gap-4">
							<div class="flex items-center gap-4">
								<Key class="h-5 w-5 shrink-0 text-secondary" />
								<p class="text-sm font-medium text-primary">{common_password()}</p>
							</div>
							<button
								type="button"
								onclick={() => {
									subView = 'credentials';
									form.reset({ currentPassword: '', password: '', confirmPassword: '' });
								}}
								class="btn-primary whitespace-nowrap"
							>
								{settings_account_updatePassword()}
							</button>
						</div>
					</InfoCard>

					<!-- Identity providers: link one for single sign-on. -->
					{#if hasOidcProviders}
						<InfoCard title={settings_account_identityProviders()}>
							<p class="text-xs text-secondary">
								{settings_account_oidcLinkHelp()}
							</p>
							<div class="space-y-3">
								{#each oidcProviders as provider (provider.slug)}
									{@const isLinked = hasLinkedOidc && user.oidc_provider === provider.slug}
									{@const isDisabled = hasLinkedOidc && !isLinked}
									<div class="flex items-center justify-between">
										<div class="mr-2 flex items-center gap-4">
											<OidcProviderLogo {provider}>
												{#snippet fallback()}
													<Link class="h-5 w-5 shrink-0 text-secondary" />
												{/snippet}
											</OidcProviderLogo>
											<div>
												<p class="text-sm font-medium text-primary">{provider.name}</p>
												{#if isLinked}
													<p class="text-xs text-secondary">
														{settings_account_linkedOn({
															date: formatDate(user.oidc_linked_at || '')
														})}
													</p>
												{:else if isDisabled}
													<p class="text-xs text-secondary">
														{settings_account_unlinkFirst({
															provider: linkedProvider?.name || ''
														})}
													</p>
												{:else}
													<p class="text-xs text-secondary">{settings_account_notLinked()}</p>
												{/if}
											</div>
										</div>
										{#if isLinked}
											<button
												type="button"
												onclick={() => unlinkOidcAccount(provider.slug)}
												class="btn-danger"
											>
												{common_unlink()}
											</button>
										{:else if !hasLinkedOidc}
											<button
												type="button"
												onclick={() => linkOidcAccount(provider.slug)}
												disabled={(linkingProviderSlug && linkingProviderSlug != provider.slug) ||
													isDisabled}
												class={isDisabled ? 'btn-disabled' : 'btn-primary'}
											>
												{linkingProviderSlug == provider.slug
													? common_redirecting()
													: common_link()}
											</button>
										{:else}
											<button type="button" disabled={isDisabled} class="btn-primary">
												{common_link()}
											</button>
										{/if}
									</div>
								{/each}
							</div>
						</InfoCard>
					{/if}

					<!-- Cookie Preferences -->
					{#if configData?.needs_cookie_consent}
						<InfoCard variant="compact">
							<div class="flex items-center justify-between">
								<div class="flex items-center gap-4">
									<Shield class="h-5 w-5 shrink-0 text-secondary" />
									<div>
										<p class="text-sm font-medium text-primary">
											{cookies_preferences()}
										</p>
										<p class="text-xs text-secondary">
											{settings_account_cookiePreferencesDesc()}
										</p>
									</div>
								</div>
								<button type="button" onclick={() => (subView = 'cookies')} class="btn-primary">
									{common_manage()}
								</button>
							</div>
						</InfoCard>
					{/if}

					<!-- Logout -->
					<InfoCard variant="compact">
						<div class="flex items-center justify-between">
							<div class="flex items-center gap-4">
								<LogOut class="h-5 w-5 text-secondary" />
								<span class="text-sm text-primary">{settings_account_signOut()}</span>
							</div>
							<button type="button" onclick={handleLogout} class="btn-secondary">
								{common_logout()}
							</button>
						</div>
					</InfoCard>
				</div>
			{:else}
				<div class="py-8 text-center text-secondary">{settings_account_loadingUser()}</div>
			{/if}
		{:else if subView === 'credentials'}
			<div class="space-y-2">
				<p class="mb-2 text-sm text-secondary">
					{user?.has_password
						? settings_account_updateCredentials()
						: settings_account_setPassword()}
				</p>
				<div class="space-y-6">
					{#if user?.has_password}
						<form.Field
							name="currentPassword"
							validators={{
								onBlur: ({ value }) => required(value)
							}}
						>
							{#snippet children(field)}
								<TextInput
									label={common_currentPassword()}
									id="currentPassword"
									type="password"
									{field}
									placeholder={settings_account_currentPasswordPlaceholder()}
								/>
							{/snippet}
						</form.Field>
					{/if}

					<form.Field
						name="password"
						validators={{
							onBlur: ({ value }) => passwordValidator(value)
						}}
					>
						{#snippet children(passwordField)}
							<form.Field
								name="confirmPassword"
								validators={{
									onBlur: ({ value, fieldApi }) =>
										confirmPasswordMatch(() => fieldApi.form.getFieldValue('password'))(value)
								}}
							>
								{#snippet children(confirmPasswordField)}
									<Password {passwordField} {confirmPasswordField} required={true} />
								{/snippet}
							</form.Field>
						{/snippet}
					</form.Field>
				</div>
			</div>
		{:else if subView === 'email-change'}
			<div class="space-y-2">
				<p class="mb-2 text-sm text-secondary">
					{configData?.has_email_service
						? settings_account_emailChangeIntro()
						: settings_account_emailChangeIntroImmediate()}
				</p>
				<div class="space-y-6">
					<emailChangeForm.Field
						name="currentPassword"
						validators={{
							onBlur: ({ value }) => required(value)
						}}
					>
						{#snippet children(field)}
							<TextInput
								label={common_currentPassword()}
								id="emailChangeCurrentPassword"
								type="password"
								{field}
								placeholder={settings_account_currentPasswordPlaceholder()}
							/>
						{/snippet}
					</emailChangeForm.Field>

					<emailChangeForm.Field
						name="newEmail"
						validators={{
							onBlur: ({ value }) => required(value) || emailValidator(value)
						}}
					>
						{#snippet children(field)}
							<TextInput
								label={common_email()}
								id="newEmail"
								{field}
								placeholder={settings_account_enterEmail()}
								required
							/>
						{/snippet}
					</emailChangeForm.Field>
				</div>
			</div>
		{:else if subView === 'cookies'}
			<CookieSettingsPanel onSave={() => (subView = 'main')} />
		{/if}
	</div>
</form>
