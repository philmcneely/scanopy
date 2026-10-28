-- The device class a fingerprint names for a host, with the source that produced it.
--
-- `matched_device_type` holds a `DeviceType` id ("Nas", "Camera") as text, the way `subnet_type`
-- holds its type's id. It is written when a Recog fingerprint of something the host emitted (a
-- certificate subject, page title, favicon hash, banner) names the device's class; `NULL` when
-- none has. The host's displayed device type stays derived on read; this is one rung of it.
--
-- The `_source` sibling follows every other discovered attribute (20260828120000): the same
-- `Unspecified` default, displaced by the first real reading.
--
-- No backfill: nothing stored today holds a fingerprinted device class. Each host gets one from
-- its next scan. Nothing is dropped.
--
-- Additive and prod-safe: both are `ADD COLUMN` with a non-volatile default, which is
-- metadata-only on PG11+ (no rewrite). Older servers ignore both columns.

SET lock_timeout = '5s';
SET statement_timeout = '30s';

ALTER TABLE hosts ADD COLUMN IF NOT EXISTS matched_device_type TEXT;
ALTER TABLE hosts ADD COLUMN IF NOT EXISTS matched_device_type_source JSONB NOT NULL DEFAULT '"Unspecified"'::jsonb;
