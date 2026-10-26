-- no-transaction
--
-- Every organization has a plan. `organizations.plan` no longer holds JSON `null`, and Free is
-- where a cloud organization starts and where an ended cloud plan lands:
--
--   plan = null, with a site      → Free. A cloud signup that never picked a plan.
--   plan = null, no site          → Self-Hosted Standard, plan_status NULL. A self-hosted license
--                                   buyer who signed up (no site step) and never picked a plan;
--                                   the app shows them the plan picker, whose trial starts it.
--                                   No trial is started for them here.
--   cloud plan, plan_status       → Free, plan_status NULL, last_downgrade_from_plan = the plan,
--     'cancelled' (lapsed)          last_downgrade_at = now(). Cloud plans now land on Free when
--                                   they end instead of staying read-only. License plans keep
--                                   their lapse and are not touched.
--   Free                          → Free with no host cap. The stored config carried
--                                   included_hosts = 25, and the stored config is what applies.
--
-- The Free and Self-Hosted Standard configs below are frozen copies of `get_free_plan()` and
-- `get_self_hosted_standard_plan()` (backend/src/server/billing/plans.rs) as of this migration.
--
-- Nothing is dropped: plan_status is cleared only where the subscription behind it no longer
-- exists, and the lapsed rows' plan moves to last_downgrade_from_plan.
--
-- Self-hosted servers create orgs on Community or Commercial, so they have no null rows; nothing
-- here changes their organizations.
--
-- Released as `downtime` (org.scanopy.deploy_mode): an older container creates cloud orgs with a
-- null plan, which the new one refuses to read.
--
-- Batched at 1000 rows with a COMMIT per batch (hence `-- no-transaction`). Each pass only matches
-- rows not yet rewritten, so re-running from any partial state finishes the job.

SET lock_timeout = '5s';
SET statement_timeout = '0';

DO $$
DECLARE
    free_plan CONSTANT JSONB := '{"type": "Free", "base_cents": 0, "rate": "Month", "trial_days": 0, "seat_cents": null, "site_cents": null, "host_cents": null, "included_seats": 1, "included_sites": 1, "included_hosts": null, "included_orgs": null}';
    standard_plan CONSTANT JSONB := '{"type": "SelfHostedStandard", "base_cents": 400000, "rate": "Year", "trial_days": 14, "seat_cents": null, "site_cents": null, "host_cents": null, "included_seats": 25, "included_sites": 50, "included_hosts": null, "included_orgs": 1}';
    updated INTEGER;
BEGIN
    PERFORM set_config('lock_timeout', '5s', false);
    PERFORM set_config('statement_timeout', '0', false);

    -- Orgs that never picked a plan.
    LOOP
        UPDATE organizations o
        SET plan = CASE
                WHEN EXISTS (SELECT 1 FROM sites s WHERE s.organization_id = o.id) THEN free_plan
                ELSE standard_plan
            END,
            plan_status = NULL,
            updated_at = now()
        WHERE o.id IN (
            SELECT id FROM organizations
            WHERE plan = 'null'::jsonb
            ORDER BY id
            LIMIT 1000
        );
        GET DIAGNOSTICS updated = ROW_COUNT;
        COMMIT;
        EXIT WHEN updated = 0;
    END LOOP;

    -- Lapsed cloud plans land on Free.
    LOOP
        UPDATE organizations
        SET last_downgrade_from_plan = plan,
            last_downgrade_at = now(),
            plan = free_plan,
            plan_status = NULL,
            updated_at = now()
        WHERE id IN (
            SELECT id FROM organizations
            WHERE plan->>'type' IN ('Starter', 'Pro', 'Team', 'Business', 'Enterprise')
              -- `canceled` is the legacy spelling (see `PlanStatus::Cancelled`).
              AND plan_status IN ('cancelled', 'canceled')
            ORDER BY id
            LIMIT 1000
        );
        GET DIAGNOSTICS updated = ROW_COUNT;
        COMMIT;
        EXIT WHEN updated = 0;
    END LOOP;

    -- Free loses its host cap.
    LOOP
        UPDATE organizations
        SET plan = free_plan,
            updated_at = now()
        WHERE id IN (
            SELECT id FROM organizations
            WHERE plan->>'type' = 'Free'
              AND plan <> free_plan
            ORDER BY id
            LIMIT 1000
        );
        GET DIAGNOSTICS updated = ROW_COUNT;
        COMMIT;
        EXIT WHEN updated = 0;
    END LOOP;
END $$;
