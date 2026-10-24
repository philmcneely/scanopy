-- Existing topologies get the `ByDeviceType` element rule as their lowest-priority rule.
--
-- New topologies start with it directly below the empty tag rule (`default_element_rules`).
-- Stored topologies keep their own order: the rule is appended to the end of
-- `options.request.element_rules`, so every rule a user already has still claims its elements
-- first and device-type groups take only the hosts nothing else grouped. A topology that already
-- has the rule is left alone, so the migration is safe to re-run.
--
-- `topologies` holds one row per network (live views only; snapshots build from SCD2 copies and
-- shares store no element rules), so a single UPDATE stays well under the batching threshold.

SET lock_timeout = '5s';

UPDATE topologies
SET options = jsonb_set(
    options,
    '{request,element_rules}',
    (options->'request'->'element_rules')
        || jsonb_build_array(jsonb_build_object('id', gen_random_uuid(), 'rule', 'ByDeviceType'))
)
WHERE jsonb_typeof(options->'request'->'element_rules') = 'array'
  AND NOT (options->'request'->'element_rules') @> '[{"rule": "ByDeviceType"}]'::jsonb;
