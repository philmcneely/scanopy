-- no-transaction
--
-- Contract for `20261012130000_discovery_warnings`: strip `results.warnings` from every
-- historical run's `discovery.run_type`, now that `discovery_warnings` holds them.
--
-- v0.18.1 moved a run's warnings into `discovery_warnings` and stopped reading or writing the JSON
-- copy. A v0.18.0 container still running during that rolling deploy could finish a run and write
-- its warnings to the JSON only. Those runs have no rows in the table, so their warnings don't
-- show. The first step below re-runs the `20261012130000` backfill, which picks them up: it skips
-- every run that already has rows.
--
-- Where each stored value goes:
--
--   results.warnings[i] (a run with no rows yet)   one `discovery_warnings` row, mapped exactly as
--                                                  `20261012130000` maps it
--   results.warnings[i] (a run with rows)          already in `discovery_warnings`; the JSON copy
--                                                  is the same warning
--   results.warnings                               removed from `run_type`
--
-- `known_codes` is today's `DiscoveryWarningCode`, not the `20261012130000` copy. `20261022120000`
-- rewrote `CredentialDaemonTooOld` in this JSON to `CredentialNeedsNewerDaemon`, which the old list
-- lacks and would have stored as `Unknown`.
--
-- The server writes `"warnings": []` on every run it saves (`SqlValue::RunType`), and reads a
-- missing key as empty (`#[serde(default)]`), so only non-empty arrays are stripped.
--
-- Batched at about 1000 rows with a COMMIT per batch (hence `-- no-transaction`). Both steps skip
-- what is already done, so re-running from any partial state finishes the job.

SET lock_timeout = '5s';
SET statement_timeout = '0';

DO $$
DECLARE
    known_codes TEXT[] := ARRAY[
        'InterfaceSetCutShort', 'InterfaceDetailsCutShort', 'SnmpWalkEntryCap',
        'SnmpWalkUnsupported', 'SnmpWalkDesynchronised', 'SnmpWalkPartialDiscarded',
        'SnmpWalkPartialRecorded', 'SnmpWalkBridgeMibAbsent', 'SnmpWalkNoAnswer', 'SnmpWalkFailed',
        'ClaimedCountReadCutShort', 'ClaimedCountUnderRead', 'ClaimedCapabilityReadCutShort',
        'ClaimedCapabilityEmpty', 'LldpLocalPortDropped', 'LldpLocalPortDroppedReadCutShort',
        'LldpLocalPortMisplaced', 'MalformedNeighboursWalkCutShort', 'MalformedNeighboursGhostRows',
        'MalformedNeighboursIncompleteRecords', 'MalformedNeighboursUnexpectedType',
        'MalformedNeighboursUnreadableIndex', 'SnmpCollectedNothing', 'VlanRecordingFailed',
        'EqualReachIntegrationsMerged', 'CredentialTargetNotScanned',
        'CredentialTargetNotResponding', 'CredentialDaemonOsMismatch', 'CredentialNeedsNewerDaemon',
        'CredentialWithoutTarget', 'CredentialGateClosed', 'CredentialRejected',
        'CredentialMalformed', 'CredentialTlsFailed', 'CredentialNotThisService',
        'CredentialCollectionFailed', 'CredentialCollectionTimedOut', 'CredentialUnreachable',
        'CredentialTimedOut', 'IntegrationWithoutCredential', 'ConnectionsWithoutProtocolResponse',
        'ScanTimeLimitWithEstimate', 'ScanTimeLimit', 'DcpSweepTimedOut', 'IcmpSweepTimedOut',
        'ReverseDnsTimedOut', 'LldpNeighbourNotFound', 'LldpNeighbourAmbiguous',
        'LldpPortNoStrategy', 'LldpPortNotFound', 'LldpPortAmbiguous', 'ProvisionalSubnetInferred',
        'NeighbourResolutionIncomplete', 'FdbResolutionIncomplete', 'OutdatedDaemonFormat',
        'WarningsTruncated', 'Unknown'
    ];
    uuid_pattern TEXT := '^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$';
    last_id UUID := '00000000-0000-0000-0000-000000000000';
    batch UUID[];
BEGIN
    PERFORM set_config('lock_timeout', '5s', false);
    PERFORM set_config('statement_timeout', '0', false);

    -- 1. Backfill runs that have warnings in the JSON and no rows yet. Same as `20261012130000`.
    LOOP
        SELECT array_agg(id ORDER BY id)
        INTO batch
        FROM (
            SELECT id, SUM(n) OVER (ORDER BY id) - n AS before_this
            FROM (
                SELECT d.id, jsonb_array_length(d.run_type->'results'->'warnings') AS n
                FROM discovery d
                WHERE d.id > last_id
                  AND d.run_type->>'type' = 'Historical'
                  AND jsonb_typeof(d.run_type->'results'->'warnings') = 'array'
                  AND jsonb_array_length(d.run_type->'results'->'warnings') > 0
                  AND NOT EXISTS (
                      SELECT 1 FROM discovery_warnings x WHERE x.discovery_id = d.id
                  )
                ORDER BY d.id
                LIMIT 1000
            ) candidates
        ) running
        WHERE before_this < 1000;

        EXIT WHEN batch IS NULL;

        INSERT INTO discovery_warnings (
            id, discovery_id, site_id, code,
            credential_id, host_id, remote_host_id, subnet_id,
            details, created_at
        )
        SELECT
            gen_random_uuid(),
            d.id,
            d.site_id,
            CASE WHEN w.elem->>'code' = ANY(known_codes) THEN w.elem->>'code' ELSE 'Unknown' END,
            CASE WHEN w.elem->>'credential_id' ~ uuid_pattern THEN
                (SELECT c.id FROM credentials c WHERE c.id = (w.elem->>'credential_id')::uuid)
            END,
            CASE WHEN w.elem->>'host_id' ~ uuid_pattern THEN
                (SELECT h.id FROM hosts h WHERE h.id = (w.elem->>'host_id')::uuid)
            END,
            CASE WHEN w.elem->>'remote_host_id' ~ uuid_pattern THEN
                (SELECT h.id FROM hosts h WHERE h.id = (w.elem->>'remote_host_id')::uuid)
            END,
            CASE WHEN w.elem->>'subnet_id' ~ uuid_pattern THEN
                (SELECT s.id FROM subnets s WHERE s.id = (w.elem->>'subnet_id')::uuid)
            END,
            w.elem - 'credential_id' - 'host_id' - 'remote_host_id' - 'subnet_id',
            d.created_at
        FROM discovery d
        CROSS JOIN LATERAL (
            SELECT CASE
                WHEN jsonb_typeof(e) = 'object' THEN e
                ELSE jsonb_build_object('code', 'Unknown', 'detail', e #>> '{}')
            END AS elem
            FROM jsonb_array_elements(d.run_type->'results'->'warnings') e
        ) w
        WHERE d.id = ANY(batch);

        last_id := batch[array_length(batch, 1)];
        COMMIT;
    END LOOP;

    -- 2. Strip the JSON copy, 1000 runs at a time.
    last_id := '00000000-0000-0000-0000-000000000000';
    LOOP
        SELECT array_agg(id ORDER BY id)
        INTO batch
        FROM (
            SELECT d.id
            FROM discovery d
            WHERE d.id > last_id
              AND d.run_type->>'type' = 'Historical'
              AND d.run_type->'results' ? 'warnings'
              AND d.run_type->'results'->'warnings' <> '[]'::jsonb
            ORDER BY d.id
            LIMIT 1000
        ) next_runs;

        EXIT WHEN batch IS NULL;

        UPDATE discovery
        SET run_type = run_type #- '{results,warnings}'
        WHERE id = ANY(batch);

        last_id := batch[array_length(batch, 1)];
        COMMIT;
    END LOOP;
END $$;
