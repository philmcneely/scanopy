import type { components } from '$lib/api/schema';
import { permissions } from '$lib/shared/stores/metadata';

// Re-export generated types
export type User = components['schemas']['User'];
export type UserOrgPermissions = components['schemas']['UserOrgPermissions'];

/** A row of the members list: a user, or an invite still waiting to be accepted. */
export type UserOrInvite = components['schemas']['Member'];
type MemberUser = Extract<UserOrInvite, { type: 'user' }>;
type MemberInvite = Extract<UserOrInvite, { type: 'invite' }>;

export function isUser(item: UserOrInvite): item is MemberUser {
	return item.type === 'user';
}

export function isInvite(item: UserOrInvite): item is MemberInvite {
	return item.type === 'invite';
}

/**
 * Whether a user with this permission level may list the org's users. `GET /api/v1/users`
 * requires Admin, which is the set `manage_org_entities` marks.
 */
export function canListUsers(permission: UserOrgPermissions): boolean {
	return permissions.getMetadata(permission)?.manage_org_entities ?? false;
}
