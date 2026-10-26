import { describe, expect, it } from 'vitest';
import { canListUsers } from '$lib/features/users/types';

/**
 * `GET /api/v1/users` requires Admin. The gate reads the backend's permission metadata, so a
 * Viewer or Member never sends the request and never sees its 403 toast.
 */
describe('users list gate', () => {
	it('is closed below Admin', () => {
		expect(canListUsers('Viewer')).toBe(false);
		expect(canListUsers('Member')).toBe(false);
	});

	it('is open for Admin and Owner', () => {
		expect(canListUsers('Admin')).toBe(true);
		expect(canListUsers('Owner')).toBe(true);
	});
});
