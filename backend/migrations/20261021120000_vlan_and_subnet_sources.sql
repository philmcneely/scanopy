-- Who named each VLAN and subnet, and who typed each subnet.
--
-- Each `*_source` column is an `AttributeSource` on the shared ladder, so the applier that guards a
-- host's name and a subnet's CIDR now decides whether a reported VLAN name, subnet name or subnet
-- type may replace the stored one.
--
-- Every existing row starts at "Unspecified", the floor: nothing recorded who set these values. The
-- next scan that reads a value stamps its source; a person's edit stamps "Manual".
--
-- Additive and prod-safe: ADD COLUMN with a non-volatile default is metadata-only on PG11+, no
-- rewrite. No contract step: nothing is renamed or dropped, and older servers ignore the columns.

SET lock_timeout = '5s';
SET statement_timeout = '30s';

ALTER TABLE vlans
    ADD COLUMN IF NOT EXISTS name_source JSONB NOT NULL DEFAULT '"Unspecified"'::jsonb;

ALTER TABLE subnets
    ADD COLUMN IF NOT EXISTS name_source JSONB NOT NULL DEFAULT '"Unspecified"'::jsonb,
    ADD COLUMN IF NOT EXISTS subnet_type_source JSONB NOT NULL DEFAULT '"Unspecified"'::jsonb;
