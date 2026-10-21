-- Who named each VLAN.
--
-- Two discovery paths now name VLANs: SNMP from a switch's dot1qVlanStaticName, and the UniFi
-- integration from the controller's network list. Until now `upsert_from_discovery` overwrote the
-- name whenever a scan reported a different one, so a name a person typed into Scanopy lasted
-- until the next scan, and two sources reporting different names would take turns. `name_source`
-- is an `AttributeSource` on the shared ladder, so the same applier that guards a host's name and a
-- subnet's CIDR decides whether a reported name may replace the stored one: a person's entry is
-- never displaced, and a weaker source never replaces a stronger one.
--
-- Values are serde's externally tagged form: `"Manual"`, or `{"Authored":"Snmp"}` for a name a
-- person set on the switch and SNMP carried back.
--
-- Backfill, covering live, closed (`valid_to IS NOT NULL`) and snapshot rows alike, since the
-- predicate is on `source` alone:
--   - `source` Manual    -> "Manual". A person created the row, so the name is theirs.
--   - `source` Discovery -> {"Authored":"Snmp"}. SNMP was the only discovery path that wrote VLANs
--     before this release, and a VLAN name is something a person configured on the switch. A
--     Discovery row a person later renamed in Scanopy cannot be told apart from one they didn't,
--     and gets the SNMP rung; that rename was already lost on every rescan before this change.
--   - anything else stays "Unspecified", the floor, so the next reading replaces it.
--
-- Additive and prod-safe: ADD COLUMN with a non-volatile default is metadata-only on PG11+, no
-- rewrite. One UPDATE rather than a batched loop: `vlans` holds at most 4094 rows per site per
-- revision, the same reasoning `20260826120000_subnets_cidr_source.sql` gives for `subnets`.
--
-- No contract step: nothing is renamed or dropped, and older servers ignore the column.

SET lock_timeout = '5s';
SET statement_timeout = '30s';

ALTER TABLE vlans
    ADD COLUMN IF NOT EXISTS name_source JSONB NOT NULL DEFAULT '"Unspecified"'::jsonb;

UPDATE vlans
   SET name_source = CASE source->>'type'
                         WHEN 'Manual' THEN '"Manual"'::jsonb
                         WHEN 'Discovery' THEN '{"Authored":"Snmp"}'::jsonb
                     END
 WHERE source->>'type' IN ('Manual', 'Discovery')
   AND name_source = '"Unspecified"'::jsonb;
