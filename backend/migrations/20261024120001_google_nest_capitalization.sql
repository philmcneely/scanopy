-- Capitalize the Google Nest service definitions: "Google Nest router" becomes "Google Nest Router"
-- and "Google Nest repeater" becomes "Google Nest Repeater".
--
-- A service definition's id is its name, stored as a JSON string in services.service_definition,
-- so the stored rows move with the rename. Live, closed and snapshot rows share the column and are
-- all renamed in place; no other column stores a definition id. Nothing is dropped.

SET lock_timeout = '5s';

UPDATE services
SET service_definition = '"Google Nest Router"'
WHERE service_definition = '"Google Nest router"';

UPDATE services
SET service_definition = '"Google Nest Repeater"'
WHERE service_definition = '"Google Nest repeater"';
