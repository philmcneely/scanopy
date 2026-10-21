-- Stored `CredentialDaemonTooOld` warnings become `CredentialNeedsNewerDaemon` with a
-- `ParsesCredentialType` gap, and the old code is removed from the server.
--
-- Daemon version gates are now stated as the last release *without* a feature rather than the
-- first release with it, and a daemon too old for a credential is reported through one warning
-- whose `gap` names what it missed. The old warning carried the first release that could parse
-- the credential type; the new one carries the last release that could not.
--
-- Where each stored value goes:
--
--   details.code                       'CredentialDaemonTooOld' → 'CredentialNeedsNewerDaemon'
--   discovery_warnings.code            the same
--   details.integration                unchanged
--   details.credential_id              unchanged (NULL in the table; the id is in its column)
--   details.daemon_version             unchanged, or absent as before
--   details.minimum_daemon_version     details.last_unsupported_daemon_version: the git tag
--                                      immediately before it, from the table below. Every floor
--                                      the server ever sent is listed; a value that is not makes
--                                      the migration fail rather than guess.
--   (new) details.gap                  {"type": "ParsesCredentialType"}: the only case the old
--                                      warning recorded was a type the daemon could not parse.
--
-- `discovery.run_type -> results -> warnings` gets the same rewrite. Nothing reads that key, but
-- the pending contract migration that strips it first re-runs the `20261012130000` backfill, which
-- copies its warnings into `discovery_warnings`; rewriting them here means that copy carries the
-- new code (docs/db-expand-contract-ledger.md).
--
-- Released as `downtime` (org.scanopy.deploy_mode): the server that writes the new code and the
-- rewrite of the old one ship together, with no window where an old container writes
-- `CredentialDaemonTooOld` after this has run. Either direction would only degrade a warning to
-- `Unknown` (`warning_from_value` reads leniently), not fail a read.
--
-- Few rows: the old warning was raised only for a daemon too old for a credential type assigned
-- to its site. One transaction.

SET lock_timeout = '5s';
-- Few rows change, but finding the JSON ones scans every run's `run_type`.
SET statement_timeout = '10min';

CREATE TEMPORARY TABLE floor_to_last_unsupported (floor TEXT PRIMARY KEY, last_unsupported TEXT NOT NULL)
    ON COMMIT DROP;

INSERT INTO floor_to_last_unsupported (floor, last_unsupported) VALUES
    ('0.16.2', '0.16.1'),
    ('0.17.0', '0.16.2'),
    ('0.17.2', '0.17.1'),
    ('0.17.7', '0.17.6'),
    ('0.17.11', '0.17.10'),
    ('0.17.12', '0.17.11'),
    ('0.17.16', '0.17.15'),
    ('0.17.19', '0.17.18'),
    ('0.17.21', '0.17.20'),
    ('0.18.0', '0.17.22'),
    ('0.18.1', '0.18.0'),
    ('0.18.2', '0.18.1');

-- One warning in its new shape. Returns NULL for a floor the table does not cover.
CREATE FUNCTION pg_temp.needs_newer_daemon(warning JSONB) RETURNS JSONB AS $$
    SELECT (warning - 'minimum_daemon_version')
        || jsonb_build_object(
            'code', 'CredentialNeedsNewerDaemon',
            'gap', jsonb_build_object('type', 'ParsesCredentialType'),
            'last_unsupported_daemon_version', f.last_unsupported
        )
    FROM floor_to_last_unsupported f
    WHERE f.floor = warning->>'minimum_daemon_version'
$$ LANGUAGE SQL STABLE;

DO $$
DECLARE
    unmapped TEXT;
BEGIN
    SELECT string_agg(DISTINCT v, ', ') INTO unmapped
    FROM (
        SELECT details->>'minimum_daemon_version' AS v
        FROM discovery_warnings
        WHERE code = 'CredentialDaemonTooOld'
        UNION ALL
        SELECT w->>'minimum_daemon_version'
        FROM discovery d,
             jsonb_array_elements(
                 CASE WHEN jsonb_typeof(d.run_type->'results'->'warnings') = 'array'
                      THEN d.run_type->'results'->'warnings' ELSE '[]'::jsonb END
             ) w
        WHERE w->>'code' = 'CredentialDaemonTooOld'
    ) stored
    WHERE v IS NULL OR v NOT IN (SELECT floor FROM floor_to_last_unsupported);

    IF unmapped IS NOT NULL THEN
        RAISE EXCEPTION 'CredentialDaemonTooOld warnings carry floors with no known preceding release: %', unmapped;
    END IF;
END $$;

UPDATE discovery_warnings
SET code = 'CredentialNeedsNewerDaemon',
    details = pg_temp.needs_newer_daemon(details)
WHERE code = 'CredentialDaemonTooOld';

UPDATE discovery d
SET run_type = jsonb_set(
    d.run_type,
    '{results,warnings}',
    (
        SELECT jsonb_agg(
            CASE WHEN w->>'code' = 'CredentialDaemonTooOld'
                 THEN pg_temp.needs_newer_daemon(w)
                 ELSE w END
            ORDER BY i
        )
        FROM jsonb_array_elements(d.run_type->'results'->'warnings') WITH ORDINALITY AS e(w, i)
    )
)
WHERE jsonb_typeof(d.run_type->'results'->'warnings') = 'array'
  AND d.run_type->'results'->'warnings' @> '[{"code": "CredentialDaemonTooOld"}]';
