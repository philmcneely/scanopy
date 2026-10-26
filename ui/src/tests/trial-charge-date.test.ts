import { describe, expect, it } from 'vitest';
import type { Organization } from '$lib/features/organizations/types';
import { trialChargeDate } from '$lib/shared/utils/trial';

const now = Date.UTC(2026, 9, 25);
const day = 24 * 60 * 60 * 1000;

function org(fields: Partial<Organization>): Organization {
	return { plan_status: null, trial_end_date: null, ...fields } as Organization;
}

function plan(trial_days: number): Organization['plan'] {
	return { trial_days } as Organization['plan'];
}

describe('trialChargeDate', () => {
	it('keeps the current trial end for a trialing org, whatever plan is picked', () => {
		const trialing = org({ plan_status: 'trialing', trial_end_date: '2026-11-01T00:00:00Z' });
		expect(trialChargeDate(trialing, plan(14), now)).toEqual(new Date('2026-11-01T00:00:00Z'));
		expect(trialChargeDate(trialing, null, now)).toEqual(new Date('2026-11-01T00:00:00Z'));
	});

	it("counts the picked plan's trial from now for an org that never trialed", () => {
		expect(trialChargeDate(org({}), plan(14), now)).toEqual(new Date(now + 14 * day));
	});

	it('charges straight away once the trial is used or the plan has none', () => {
		const used = org({ plan_status: null, trial_end_date: '2026-09-01T00:00:00Z' });
		expect(trialChargeDate(used, plan(14), now)).toBeNull();
		expect(trialChargeDate(org({}), plan(0), now)).toBeNull();
		expect(trialChargeDate(org({ plan_status: 'active' }), null, now)).toBeNull();
	});
});
