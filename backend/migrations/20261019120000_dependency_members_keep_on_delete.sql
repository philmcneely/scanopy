-- Dependency members outlive the service or binding they name.
--
-- A member used to be deleted with its service or binding by `ON DELETE CASCADE`, in the same
-- statement and with no event, so nothing could tell the user a dependency had lost a member.
-- The member now stays, naming an id that no longer resolves: the dependency service's
-- subscriber reports it (`DependencyMemberChange`), the dependency shows it as missing, and the
-- user decides what replaces it.
--
-- Drops the two foreign keys that cascaded. No row is changed or removed; `dependency_id` keeps
-- its cascade, so deleting a dependency still deletes its members.

SET lock_timeout = '5s';
SET statement_timeout = '5s';

ALTER TABLE dependency_members DROP CONSTRAINT IF EXISTS dependency_members_service_id_fkey;
ALTER TABLE dependency_members DROP CONSTRAINT IF EXISTS group_bindings_binding_id_fkey;
