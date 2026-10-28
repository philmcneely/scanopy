-- Seed a large synthetic L2 Physical topology for render-performance profiling.
--
-- THROWAWAY. This exists to reproduce a customer's slow L2 Physical view locally
-- (perf/l2-topology-scale). It is not product tooling and is not wired into any
-- Makefile target — run it by hand, once, on a dev database.
--
-- Prerequisite: a site to write to. Demo data supplies the org, site, subnets and
-- topology row, and by default this script scales the L2 layer on top of the site
-- that already has physical links. Pass `-v site_id` to write to a different one —
-- see "Target site" below — which is how you keep a working estate and a
-- 1,500-container profiling estate in the same database. The script writes
-- `hosts`, `interfaces` and their neighbour rows; a site row and a `topologies`
-- row are the caller's to create.
--
-- Run:
--   psql postgres://postgres:password@localhost:5432/scanopy -v ON_ERROR_STOP=1 \
--     < backend/scripts/seed-l2-perf.sql
--
-- Idempotent *per site*: every id is derived from md5(<target site> || <stable
-- key>), so re-running against the same site updates the same rows rather than
-- duplicating them, and seeding a second site produces a disjoint set. The site
-- has to be part of the key — without it the ids collide across sites and
-- `ON CONFLICT ... DO UPDATE` quietly refreshes the *first* site's rows in place,
-- leaving the one you asked for empty and the `site_id` knob doing nothing.
--
-- To undo, see the DELETE at the bottom of this file (commented out).
--
-- Shape produced (override on the command line):
--   n_switches            core switches, each a Host container in the L2 view
--   hosts_per_switch      hosts hanging off each switch -> the tall columns
--   extra_ifaces_per_host additional unlinked ports per host, for realistic
--                         element counts inside each host container
--   device_level_pct      percentage of uplinks resolved only as far as the
--                         far-end device, drawing a dashed NeighborLink between
--                         host containers instead of a port-precise PhysicalLink
--
-- Defaults give 8 switches + 400 hosts = 408 host containers, ~1600 interface
-- elements and 400 PhysicalLink edges.
--
--   # ~600 devices — the interactive-performance profile
--   psql ... -v hosts_per_switch=75 < backend/scripts/seed-l2-perf.sql
--
--   # ~17k nodes — the shape of the customer's out-of-memory report. Their
--   # estate averaged ~24.6 nodes per host (one container + ~22 interfaces +
--   # grouping subcontainers), which is what a Windows/hypervisor SNMP agent
--   # reports: teaming members and vswitch uplinks all present as if_type 6.
--   psql ... -v hosts_per_switch=88 -v extra_ifaces_per_host=22 \
--     < backend/scripts/seed-l2-perf.sql
--
--   # the post-guard edge mix: switches that report one chassis MAC on every
--   # port name no port on each other, so almost every link degrades to a
--   # device-level NeighborLink.
--   psql ... -v device_level_pct=99 < backend/scripts/seed-l2-perf.sql
--
--   # a throwaway site of its own, so the estate you actually use is left alone.
--   # Create the site first, then point the seed at it; dropping that one row
--   # cascades the whole profile away afterwards.
--   psql ... -v site_id=<uuid> -v hosts_per_switch=185 \
--     < backend/scripts/seed-l2-perf.sql

\if :{?n_switches} \else \set n_switches 8 \endif
\if :{?hosts_per_switch} \else \set hosts_per_switch 50 \endif
\if :{?extra_ifaces_per_host} \else \set extra_ifaces_per_host 2 \endif
\if :{?device_level_pct} \else \set device_level_pct 0 \endif
\if :{?site_id} \else \set site_id '' \endif

BEGIN;

-- ---------------------------------------------------------------------------
-- Target site
-- ---------------------------------------------------------------------------
-- With `-v site_id=<uuid>`, that site and no other. A site_id that matches
-- nothing aborts rather than falling through to the election, because falling
-- through would seed exactly the site the caller was steering away from.
--
-- Without it, the election: prefer the site that already has port-precise
-- neighbours (its L2 Physical view is enabled), then the site with the most live
-- hosts, then any site at all.

-- The requested id, parked in a table because psql does not interpolate `:vars`
-- inside a dollar-quoted body and the guard below needs to name it.
CREATE TEMPORARY TABLE l2perf_request ON COMMIT DROP AS
SELECT :'site_id'::text AS requested;

CREATE TEMPORARY TABLE l2perf_target ON COMMIT DROP AS
WITH explicit AS (
    SELECT s.id AS site_id, 0 AS weight, 0 AS priority
    FROM sites s
    WHERE :'site_id' <> '' AND s.id::text = :'site_id'
),
linked AS (
    SELECT n.site_id, count(*) AS weight, 1 AS priority
    FROM interface_neighbor_interfaces n
    WHERE n.valid_to IS NULL
    GROUP BY n.site_id
),
populated AS (
    SELECT h.site_id, count(*) AS weight, 2 AS priority
    FROM hosts h
    WHERE h.valid_to IS NULL
    GROUP BY h.site_id
),
any_site AS (
    SELECT s.id AS site_id, 0 AS weight, 3 AS priority
    FROM sites s
)
SELECT site_id
FROM (
    SELECT * FROM explicit
    UNION ALL SELECT * FROM linked
    UNION ALL SELECT * FROM populated
    UNION ALL SELECT * FROM any_site
) c
-- An explicit id wins outright: it is priority 0, and the guard below refuses to
-- run at all if it matched nothing, so the later tiers can never stand in for it.
WHERE :'site_id' = '' OR priority = 0
ORDER BY priority, weight DESC
LIMIT 1;

DO $$
DECLARE
    requested text;
BEGIN
    IF EXISTS (SELECT 1 FROM l2perf_target) THEN
        RETURN;
    END IF;
    SELECT r.requested INTO requested FROM l2perf_request r;
    IF requested <> '' THEN
        RAISE EXCEPTION 'No site with id %. Nothing was written.', requested;
    END IF;
    RAISE EXCEPTION 'No site found. Create a site (or populate demo data) first.';
END $$;

-- ---------------------------------------------------------------------------
-- Switches
-- ---------------------------------------------------------------------------
INSERT INTO hosts (
    id, site_id, name, hostname, description, source,
    created_at, updated_at, sys_name, manufacturer, model
)
SELECT
    md5(t.site_id || ':l2perf:switch:' || s)::uuid,
    t.site_id,
    'l2perf-switch-' || lpad(s::text, 2, '0'),
    'l2perf-switch-' || lpad(s::text, 2, '0'),
    'Synthetic L2 perf switch',
    '{"type": "Discovery"}'::jsonb,
    now(), now(),
    'l2perf-switch-' || lpad(s::text, 2, '0'),
    'Scanopy', 'Synthetic-48P'
FROM l2perf_target t, generate_series(1, :n_switches) AS s
ON CONFLICT (id) DO UPDATE SET updated_at = now();

-- ---------------------------------------------------------------------------
-- Access hosts
-- ---------------------------------------------------------------------------
-- One host per switch port. `h` is the global host ordinal so names stay unique.
INSERT INTO hosts (
    id, site_id, name, hostname, description, source,
    created_at, updated_at, sys_name, manufacturer, model
)
SELECT
    md5(t.site_id || ':l2perf:host:' || s || ':' || p)::uuid,
    t.site_id,
    'l2perf-host-' || lpad(s::text, 2, '0') || '-' || lpad(p::text, 3, '0'),
    'l2perf-host-' || lpad(s::text, 2, '0') || '-' || lpad(p::text, 3, '0'),
    'Synthetic L2 perf host',
    '{"type": "Discovery"}'::jsonb,
    now(), now(),
    'l2perf-host-' || lpad(s::text, 2, '0') || '-' || lpad(p::text, 3, '0'),
    'Scanopy', 'Synthetic-Node'
FROM l2perf_target t,
     generate_series(1, :n_switches) AS s,
     generate_series(1, :hosts_per_switch) AS p
ON CONFLICT (id) DO UPDATE SET updated_at = now();

-- ---------------------------------------------------------------------------
-- Switch ports
-- ---------------------------------------------------------------------------
-- if_type 6 = ethernetCsmacd. Anything in l2_builder's EXCLUDED_IF_TYPES
-- (24, 53, 71, 131, 135, 136, 209) is filtered out of the view, so physical
-- ports must not use those.
INSERT INTO interfaces (
    id, host_id, site_id, if_index, if_descr, if_alias, if_type,
    speed_bps, admin_status, oper_status, created_at, updated_at
)
SELECT
    md5(t.site_id || ':l2perf:switchport:' || s || ':' || p)::uuid,
    md5(t.site_id || ':l2perf:switch:' || s)::uuid,
    t.site_id,
    p,
    'Port 1/0/' || p,
    'to l2perf-host-' || lpad(s::text, 2, '0') || '-' || lpad(p::text, 3, '0'),
    6,
    1000000000,
    1, 1,
    now(), now()
FROM l2perf_target t,
     generate_series(1, :n_switches) AS s,
     generate_series(1, :hosts_per_switch) AS p
ON CONFLICT (id) DO UPDATE SET updated_at = now();

-- ---------------------------------------------------------------------------
-- Host uplinks
-- ---------------------------------------------------------------------------
INSERT INTO interfaces (
    id, host_id, site_id, if_index, if_descr, if_alias, if_type,
    speed_bps, admin_status, oper_status, created_at, updated_at
)
SELECT
    md5(t.site_id || ':l2perf:uplink:' || s || ':' || p)::uuid,
    md5(t.site_id || ':l2perf:host:' || s || ':' || p)::uuid,
    t.site_id,
    1,
    'eth0',
    'uplink',
    6,
    1000000000,
    1, 1,
    now(), now()
FROM l2perf_target t,
     generate_series(1, :n_switches) AS s,
     generate_series(1, :hosts_per_switch) AS p
ON CONFLICT (id) DO UPDATE SET updated_at = now();

-- ---------------------------------------------------------------------------
-- Uplink neighbours -> PhysicalLink and NeighborLink edges
-- ---------------------------------------------------------------------------
-- An `interface_neighbor_interfaces` row is what l2_builder turns into a
-- PhysicalLink. Only one direction is needed: the builder dedups the pair on
-- sorted endpoint ids.
--
-- The first :device_level_pct percent of each switch's ports instead get an
-- `interface_neighbor_hosts` row, the state a link degrades to when the far end
-- reports one MAC across every port: the device is known and the port is not, so
-- l2_builder draws a device-level NeighborLink between the two Host containers.
-- The two produce different node endpoints, strokes and layout behaviour, so a
-- view exercised only on port-precise links has not been exercised on what the
-- customer actually had.
INSERT INTO interface_neighbor_interfaces (id, site_id, interface_id, neighbor_interface_id)
SELECT
    md5(t.site_id || ':l2perf:neighbor-if:' || s || ':' || p)::uuid,
    t.site_id,
    md5(t.site_id || ':l2perf:uplink:' || s || ':' || p)::uuid,
    md5(t.site_id || ':l2perf:switchport:' || s || ':' || p)::uuid
FROM l2perf_target t,
     generate_series(1, :n_switches) AS s,
     generate_series(1, :hosts_per_switch) AS p
WHERE p > (:hosts_per_switch * :device_level_pct) / 100
ON CONFLICT (id) DO UPDATE SET updated_at = now();

INSERT INTO interface_neighbor_hosts (id, site_id, interface_id, neighbor_host_id)
SELECT
    md5(t.site_id || ':l2perf:neighbor-host:' || s || ':' || p)::uuid,
    t.site_id,
    md5(t.site_id || ':l2perf:uplink:' || s || ':' || p)::uuid,
    md5(t.site_id || ':l2perf:switch:' || s)::uuid
FROM l2perf_target t,
     generate_series(1, :n_switches) AS s,
     generate_series(1, :hosts_per_switch) AS p
WHERE p <= (:hosts_per_switch * :device_level_pct) / 100
ON CONFLICT (id) DO UPDATE SET updated_at = now();

-- ---------------------------------------------------------------------------
-- Extra unlinked host ports
-- ---------------------------------------------------------------------------
-- These carry no neighbour, so they add no edges — they exist to give each host
-- container a realistic number of child elements to lay out and render.
INSERT INTO interfaces (
    id, host_id, site_id, if_index, if_descr, if_alias, if_type,
    speed_bps, admin_status, oper_status, created_at, updated_at
)
SELECT
    md5(t.site_id || ':l2perf:extra:' || s || ':' || p || ':' || e)::uuid,
    md5(t.site_id || ':l2perf:host:' || s || ':' || p)::uuid,
    t.site_id,
    e + 1,
    'eth' || e,
    NULL,
    6,
    1000000000,
    1,
    CASE WHEN e % 2 = 0 THEN 2 ELSE 1 END,  -- alternate down/up for visual variety
    now(), now()
FROM l2perf_target t,
     generate_series(1, :n_switches) AS s,
     generate_series(1, :hosts_per_switch) AS p,
     generate_series(1, :extra_ifaces_per_host) AS e
ON CONFLICT (id) DO UPDATE SET updated_at = now();

-- ---------------------------------------------------------------------------
-- Report
-- ---------------------------------------------------------------------------
SELECT
    (SELECT name FROM sites WHERE id = (SELECT site_id FROM l2perf_target)) AS site,
    (SELECT count(*) FROM hosts h
       WHERE h.site_id = (SELECT site_id FROM l2perf_target)
         AND h.valid_to IS NULL) AS live_hosts,
    (SELECT count(*) FROM interfaces i
       WHERE i.site_id = (SELECT site_id FROM l2perf_target)
         AND i.valid_to IS NULL) AS live_interfaces,
    (SELECT count(*) FROM interface_neighbor_interfaces n
       WHERE n.site_id = (SELECT site_id FROM l2perf_target)
         AND n.valid_to IS NULL) AS physical_links,
    (SELECT count(*) FROM interface_neighbor_hosts n
       WHERE n.site_id = (SELECT site_id FROM l2perf_target)
         AND n.valid_to IS NULL) AS neighbor_links;

COMMIT;

-- ---------------------------------------------------------------------------
-- Undo
-- ---------------------------------------------------------------------------
-- Interfaces and their neighbour rows cascade from hosts, so removing the
-- synthetic hosts is enough. This matches by name across *every* site, which is
-- what you want when clearing up but not if you are keeping one seeded site and
-- dropping another — scope it by site_id in that case, or drop the throwaway site
-- row and let the cascade take everything with it.
--
--   DELETE FROM hosts WHERE name LIKE 'l2perf-%';
