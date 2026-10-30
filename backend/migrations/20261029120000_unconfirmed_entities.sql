-- no-transaction
--
-- Unconfirmed hosts and subnets: one rule, "the entity's source is `Inferred`".
--
-- 1. `hosts.dismissed_at`: when a person dismissed an inferred host. A dismissed host stays as a
--    row so the next scan's minting pass can see the MAC or chassis id was dismissed and not mint
--    it again. Nullable, no default, no backfill: nothing has been dismissed yet.
--
-- 2. Subnets with an assumed range move to `source = Inferred { via: ReportedAddress }`. Until now
--    an assumed range was a subnet with `source = Discovery` and `cidr_source =
--    LldpNeighbourAddress`, and the badge read the range's source. The server now creates them as
--    `Inferred` and reads the subnet's source, as it does for hosts, so existing rows are carried
--    over. Every row with that range source is updated, live, closed (`valid_to IS NOT NULL`) and
--    snapshot (`snapshot_id IS NOT NULL`) alike, so a snapshot keeps showing the badge it showed
--    when taken. `cidr_source` is untouched. Nothing is dropped.
--
--    A row whose range a person confirmed carries `cidr_source = Manual` and is not matched. A
--    row already `Inferred` (the demo organization's) is not matched either.
--
-- Batched at about 1000 rows with a COMMIT per batch (hence `-- no-transaction`). The update
-- skips rows already moved, so re-running from any partial state finishes the job.

SET lock_timeout = '5s';
SET statement_timeout = '30s';

ALTER TABLE hosts ADD COLUMN IF NOT EXISTS dismissed_at TIMESTAMPTZ;

DO $$
DECLARE
    last_id UUID := '00000000-0000-0000-0000-000000000000';
    batch UUID[];
BEGIN
    PERFORM set_config('lock_timeout', '5s', false);
    PERFORM set_config('statement_timeout', '0', false);

    LOOP
        SELECT array_agg(id ORDER BY id)
        INTO batch
        FROM (
            SELECT s.id
            FROM subnets s
            WHERE s.id > last_id
              AND s.cidr_source = '"LldpNeighbourAddress"'::jsonb
              AND s.source->>'type' = 'Discovery'
            ORDER BY s.id
            LIMIT 1000
        ) next_rows;

        EXIT WHEN batch IS NULL;

        UPDATE subnets
        SET source = '{"type": "Inferred", "via": "ReportedAddress"}'::jsonb
        WHERE id = ANY(batch);

        last_id := batch[array_length(batch, 1)];
        COMMIT;
    END LOOP;
END $$;
