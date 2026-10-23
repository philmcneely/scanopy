-- no-transaction
--
-- SNMPv1 and SNMPv2c credentials become one community credential, `SnmpCommunity`, which the
-- daemon tries over v2c and then v1. Both old types carry only `community`, so the new one has the
-- same single field.
--
-- 1. Merge. An SNMPv1 credential whose community and daemon_os equal an SNMPv2c credential's in
--    the same organization is identical to it once both are `SnmpCommunity`. It merges into the
--    oldest such v2c credential (by created_at, then id), and every reference moves first, the
--    same way `20261011130001_merge_and_seed_default_credentials.sql` merged duplicate defaults:
--
--      site_credentials (site_id, credential_id)      re-pointed; a site already assigned the
--                                                     survivor keeps its one row
--      host_credentials (host_id, credential_id,      re-pointed; where the host already has the
--        ip_address_ids)                              survivor, ip_address_ids becomes the union
--                                                     (NULL, the whole host, wins)
--      entity_tags (entity_type '"Credential"')       live tags the survivor lacks move to it,
--                                                     closed and snapshot rows move as they are;
--                                                     a live tag it already has is a duplicate
--      discovery.integration_targets,                 credential ids rewritten; a target that
--        discovery.discovery_type,                    becomes identical to another in the same
--        discovery.run_type                           list is dropped as a duplicate
--
--    A v1 `public` credential merges into the organization's seeded default this way, which keeps
--    one default per organization.
--
--    **Dropped, approved by Maya on 2026-10-23:** the merged v1 credential's own name and
--    description. The survivor keeps its own. Every assignment, tag and reference carries over.
--    Two credentials of one type that were already identical are left as they are.
--
-- 2. Rewrite. Every remaining `SnmpV1` and `SnmpV2c` row gets `"type": "SnmpCommunity"`. The
--    `community` value is unchanged.
--
-- 3. The seeded default's description, where it still has the old seeded text, gets the new one
--    (`CredentialTypeDiscriminants::seeded_default`).
--
-- Credentials carry no SCD2 history or snapshot rows; `credential_type` is the only column that
-- stores the type tag. Discovery warnings name the wire tag `Snmp`, which is unchanged.
--
-- Released as `downtime` (org.scanopy.deploy_mode): a v0.19.0 container cannot read a
-- `SnmpCommunity` row, so old and new containers must not run together.
--
-- Batched at 1000 rows with a COMMIT per batch (hence `-- no-transaction`). It all runs in one DO
-- block, so the scratch table lives on one connection. Re-running from any partial state finishes
-- the job: the merge is recomputed from rows still typed `SnmpV1`, every insert skips what exists,
-- and the rewrite only touches rows not yet rewritten.

SET lock_timeout = '5s';
SET statement_timeout = '0';

DO $$
DECLARE
    last_id UUID;
    batch UUID[];
    org_row RECORD;
    pair RECORD;
BEGIN
    PERFORM set_config('lock_timeout', '5s', false);
    PERFORM set_config('statement_timeout', '0', false);

    -- Each v1 credential and the oldest v2c credential identical to it.
    DROP TABLE IF EXISTS pg_temp.snmp_community_merge;
    CREATE TEMP TABLE snmp_community_merge AS
    SELECT v1.id AS duplicate_id, survivor.id AS survivor_id, v1.organization_id
      FROM credentials v1
      CROSS JOIN LATERAL (
          SELECT v2c.id FROM credentials v2c
           WHERE v2c.organization_id = v1.organization_id
             AND v2c.credential_type->>'type' = 'SnmpV2c'
             AND v2c.credential_type->'community' = v1.credential_type->'community'
             AND v2c.daemon_os IS NOT DISTINCT FROM v1.daemon_os
           ORDER BY v2c.created_at, v2c.id
           LIMIT 1) survivor
     WHERE v1.credential_type->>'type' = 'SnmpV1';
    CREATE INDEX ON snmp_community_merge (duplicate_id);

    -- 1a. Site assignments.
    INSERT INTO site_credentials (site_id, credential_id)
    SELECT sc.site_id, m.survivor_id
      FROM site_credentials sc
      JOIN snmp_community_merge m ON m.duplicate_id = sc.credential_id
    ON CONFLICT DO NOTHING;
    COMMIT;

    -- 1b. Host assignments. Several duplicates can sit on one host, so they are folded together
    --     first, then folded into any row the survivor already has.
    INSERT INTO host_credentials (host_id, credential_id, ip_address_ids)
    SELECT host_id, survivor_id,
           CASE WHEN bool_or(whole_host) THEN NULL
                ELSE coalesce(array_agg(DISTINCT ip) FILTER (WHERE ip IS NOT NULL), '{}')
           END
      FROM (SELECT hc.host_id, m.survivor_id, hc.ip_address_ids IS NULL AS whole_host, ip
              FROM host_credentials hc
              JOIN snmp_community_merge m ON m.duplicate_id = hc.credential_id
              LEFT JOIN LATERAL unnest(hc.ip_address_ids) AS ip ON true) moved
     GROUP BY host_id, survivor_id
    ON CONFLICT (host_id, credential_id) DO UPDATE
       SET ip_address_ids = CASE
               WHEN host_credentials.ip_address_ids IS NULL OR EXCLUDED.ip_address_ids IS NULL
                   THEN NULL
               ELSE ARRAY(SELECT DISTINCT unnest(host_credentials.ip_address_ids
                                                 || EXCLUDED.ip_address_ids))
           END;
    COMMIT;

    -- 1c. Tags. One live row per tag moves to the survivor unless it already has that tag; closed
    --     and snapshot rows move as they are. What is left on a duplicate is a second copy of a
    --     live tag the survivor holds, and goes with the duplicate.
    UPDATE entity_tags et
       SET entity_id = pick.survivor_id
      FROM (SELECT DISTINCT ON (m.survivor_id, t.tag_id) t.id, m.survivor_id
              FROM entity_tags t
              JOIN snmp_community_merge m ON m.duplicate_id = t.entity_id
             WHERE t.entity_type = '"Credential"' AND t.valid_to IS NULL
             ORDER BY m.survivor_id, t.tag_id, t.created_at, t.id) pick
     WHERE et.id = pick.id
       AND NOT EXISTS (SELECT 1 FROM entity_tags s
                        WHERE s.entity_id = pick.survivor_id
                          AND s.entity_type = '"Credential"'
                          AND s.tag_id = et.tag_id
                          AND s.valid_to IS NULL);
    UPDATE entity_tags et
       SET entity_id = m.survivor_id
      FROM snmp_community_merge m
     WHERE et.entity_id = m.duplicate_id
       AND et.entity_type = '"Credential"'
       AND et.valid_to IS NOT NULL;
    DELETE FROM entity_tags et
     USING snmp_community_merge m
     WHERE et.entity_id = m.duplicate_id AND et.entity_type = '"Credential"';
    COMMIT;

    -- 1d. Credential ids inside the discovery JSON, per organization, 1000 discoveries at a time.
    --     A UUID's text appears nowhere but as that id, so a text replacement reaches every place
    --     one is stored, however deeply nested.
    FOR org_row IN SELECT DISTINCT organization_id FROM snmp_community_merge LOOP
        last_id := '00000000-0000-0000-0000-000000000000';
        LOOP
            SELECT array_agg(id ORDER BY id)
              INTO batch
              FROM (SELECT d.id FROM discovery d
                      JOIN sites s ON s.id = d.site_id
                     WHERE s.organization_id = org_row.organization_id AND d.id > last_id
                     ORDER BY d.id LIMIT 1000) t;
            EXIT WHEN batch IS NULL;

            FOR pair IN SELECT duplicate_id, survivor_id FROM snmp_community_merge
                      WHERE organization_id = org_row.organization_id LOOP
                UPDATE discovery d
                   SET integration_targets = replace(d.integration_targets::text,
                                                     pair.duplicate_id::text,
                                                     pair.survivor_id::text)::jsonb,
                       discovery_type = replace(d.discovery_type::text,
                                                pair.duplicate_id::text,
                                                pair.survivor_id::text)::jsonb,
                       run_type = replace(d.run_type::text,
                                          pair.duplicate_id::text,
                                          pair.survivor_id::text)::jsonb
                 WHERE d.id = ANY(batch)
                   AND (d.integration_targets::text LIKE '%' || pair.duplicate_id::text || '%'
                        OR d.discovery_type::text LIKE '%' || pair.duplicate_id::text || '%'
                        OR d.run_type::text LIKE '%' || pair.duplicate_id::text || '%');
            END LOOP;

            -- Two targets that now name the same credential the same way are one target.
            UPDATE discovery d
               SET integration_targets = deduped.targets
              FROM (SELECT d2.id,
                           coalesce((SELECT jsonb_agg(e ORDER BY ord)
                                       FROM (SELECT DISTINCT ON (e) e, ord
                                               FROM jsonb_array_elements(d2.integration_targets)
                                                    WITH ORDINALITY AS a(e, ord)
                                              ORDER BY e, ord) firsts),
                                    '[]'::jsonb) AS targets
                      FROM discovery d2
                     WHERE d2.id = ANY(batch)) deduped
             WHERE d.id = deduped.id
               AND d.integration_targets IS DISTINCT FROM deduped.targets;

            last_id := batch[array_length(batch, 1)];
            COMMIT;
        END LOOP;
    END LOOP;

    -- 1e. The duplicates, now unreferenced. Their junction rows go with them (ON DELETE CASCADE),
    --     and each was copied to the survivor above.
    LOOP
        SELECT array_agg(duplicate_id) INTO batch
          FROM (SELECT m.duplicate_id FROM snmp_community_merge m
                  JOIN credentials c ON c.id = m.duplicate_id
                 LIMIT 1000) t;
        EXIT WHEN batch IS NULL;
        DELETE FROM credentials WHERE id = ANY(batch);
        COMMIT;
    END LOOP;

    -- 2. Every remaining v1 and v2c credential becomes `SnmpCommunity`, 1000 at a time.
    LOOP
        SELECT array_agg(id) INTO batch
          FROM (SELECT id FROM credentials
                 WHERE credential_type->>'type' IN ('SnmpV1', 'SnmpV2c')
                 LIMIT 1000) t;
        EXIT WHEN batch IS NULL;
        UPDATE credentials
           SET credential_type = credential_type || '{"type": "SnmpCommunity"}'::jsonb
         WHERE id = ANY(batch);
        COMMIT;
    END LOOP;

    -- 3. The seeded default's description, where it was never edited, 1000 at a time.
    LOOP
        SELECT array_agg(id) INTO batch
          FROM (SELECT id FROM credentials
                 WHERE credential_type->>'type' = 'SnmpCommunity'
                   AND description = 'Assigned to every new site. Tries the SNMPv2c community "public" on every host.'
                 LIMIT 1000) t;
        EXIT WHEN batch IS NULL;
        UPDATE credentials
           SET description = 'Assigned to every new site. Tries the SNMP community "public" on every host, over v2c and then v1.'
         WHERE id = ANY(batch);
        COMMIT;
    END LOOP;

    DROP TABLE IF EXISTS pg_temp.snmp_community_merge;
END $$;
