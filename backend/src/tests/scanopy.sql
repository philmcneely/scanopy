--
-- PostgreSQL database dump
--

\restrict 8nJiNRZbRQoO2iZFcKZ0YLbyYEz6pSrJxxjWUWMvJQoeN5k0NaaYiH834yma8QG

-- Dumped from database version 17.11
-- Dumped by pg_dump version 17.11

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

ALTER TABLE IF EXISTS ONLY public.vlans DROP CONSTRAINT IF EXISTS vlans_snapshot_id_fkey;
ALTER TABLE IF EXISTS ONLY public.vlans DROP CONSTRAINT IF EXISTS vlans_site_id_fkey;
ALTER TABLE IF EXISTS ONLY public.vlans DROP CONSTRAINT IF EXISTS vlans_organization_id_fkey;
ALTER TABLE IF EXISTS ONLY public.vlans DROP CONSTRAINT IF EXISTS vlans_last_discovery_fk;
ALTER TABLE IF EXISTS ONLY public.vlans DROP CONSTRAINT IF EXISTS vlans_first_discovery_fk;
ALTER TABLE IF EXISTS ONLY public.users DROP CONSTRAINT IF EXISTS users_organization_id_fkey;
ALTER TABLE IF EXISTS ONLY public.user_site_access DROP CONSTRAINT IF EXISTS user_site_access_user_id_fkey;
ALTER TABLE IF EXISTS ONLY public.user_site_access DROP CONSTRAINT IF EXISTS user_site_access_site_id_fkey;
ALTER TABLE IF EXISTS ONLY public.user_api_keys DROP CONSTRAINT IF EXISTS user_api_keys_user_id_fkey;
ALTER TABLE IF EXISTS ONLY public.user_api_keys DROP CONSTRAINT IF EXISTS user_api_keys_organization_id_fkey;
ALTER TABLE IF EXISTS ONLY public.user_api_key_site_access DROP CONSTRAINT IF EXISTS user_api_key_site_access_site_id_fkey;
ALTER TABLE IF EXISTS ONLY public.user_api_key_site_access DROP CONSTRAINT IF EXISTS user_api_key_site_access_api_key_id_fkey;
ALTER TABLE IF EXISTS ONLY public.topologies DROP CONSTRAINT IF EXISTS topologies_site_id_fkey;
ALTER TABLE IF EXISTS ONLY public.tags DROP CONSTRAINT IF EXISTS tags_tag_group_id_fkey;
ALTER TABLE IF EXISTS ONLY public.tags DROP CONSTRAINT IF EXISTS tags_snapshot_id_fkey;
ALTER TABLE IF EXISTS ONLY public.tags DROP CONSTRAINT IF EXISTS tags_organization_id_fkey;
ALTER TABLE IF EXISTS ONLY public.tag_groups DROP CONSTRAINT IF EXISTS tag_groups_organization_id_fkey;
ALTER TABLE IF EXISTS ONLY public.subnets DROP CONSTRAINT IF EXISTS subnets_virtualization_service_id_fkey;
ALTER TABLE IF EXISTS ONLY public.subnets DROP CONSTRAINT IF EXISTS subnets_snapshot_id_fkey;
ALTER TABLE IF EXISTS ONLY public.subnets DROP CONSTRAINT IF EXISTS subnets_site_id_fkey;
ALTER TABLE IF EXISTS ONLY public.subnets DROP CONSTRAINT IF EXISTS subnets_last_discovery_fk;
ALTER TABLE IF EXISTS ONLY public.subnets DROP CONSTRAINT IF EXISTS subnets_first_discovery_fk;
ALTER TABLE IF EXISTS ONLY public.subnet_vlans DROP CONSTRAINT IF EXISTS subnet_vlans_vlan_id_fkey;
ALTER TABLE IF EXISTS ONLY public.subnet_vlans DROP CONSTRAINT IF EXISTS subnet_vlans_subnet_id_fkey;
ALTER TABLE IF EXISTS ONLY public.subnet_vlans DROP CONSTRAINT IF EXISTS subnet_vlans_snapshot_id_fkey;
ALTER TABLE IF EXISTS ONLY public.snapshots DROP CONSTRAINT IF EXISTS snapshots_site_id_fkey;
ALTER TABLE IF EXISTS ONLY public.snapshots DROP CONSTRAINT IF EXISTS snapshots_created_by_user_id_fkey;
ALTER TABLE IF EXISTS ONLY public.site_credentials DROP CONSTRAINT IF EXISTS site_credentials_site_id_fkey;
ALTER TABLE IF EXISTS ONLY public.site_credentials DROP CONSTRAINT IF EXISTS site_credentials_credential_id_fkey;
ALTER TABLE IF EXISTS ONLY public.shares DROP CONSTRAINT IF EXISTS shares_topology_id_fkey;
ALTER TABLE IF EXISTS ONLY public.shares DROP CONSTRAINT IF EXISTS shares_site_id_fkey;
ALTER TABLE IF EXISTS ONLY public.shares DROP CONSTRAINT IF EXISTS shares_created_by_fkey;
ALTER TABLE IF EXISTS ONLY public.services DROP CONSTRAINT IF EXISTS services_virtualization_service_id_fkey;
ALTER TABLE IF EXISTS ONLY public.services DROP CONSTRAINT IF EXISTS services_snapshot_id_fkey;
ALTER TABLE IF EXISTS ONLY public.services DROP CONSTRAINT IF EXISTS services_site_id_fkey;
ALTER TABLE IF EXISTS ONLY public.services DROP CONSTRAINT IF EXISTS services_last_discovery_fk;
ALTER TABLE IF EXISTS ONLY public.services DROP CONSTRAINT IF EXISTS services_host_id_fkey;
ALTER TABLE IF EXISTS ONLY public.services DROP CONSTRAINT IF EXISTS services_first_discovery_fk;
ALTER TABLE IF EXISTS ONLY public.ports DROP CONSTRAINT IF EXISTS ports_snapshot_id_fkey;
ALTER TABLE IF EXISTS ONLY public.ports DROP CONSTRAINT IF EXISTS ports_site_id_fkey;
ALTER TABLE IF EXISTS ONLY public.ports DROP CONSTRAINT IF EXISTS ports_last_discovery_fk;
ALTER TABLE IF EXISTS ONLY public.ports DROP CONSTRAINT IF EXISTS ports_host_id_fkey;
ALTER TABLE IF EXISTS ONLY public.ports DROP CONSTRAINT IF EXISTS ports_first_discovery_fk;
ALTER TABLE IF EXISTS ONLY public.sites DROP CONSTRAINT IF EXISTS organization_id_fkey;
ALTER TABLE IF EXISTS ONLY public.license_instances DROP CONSTRAINT IF EXISTS license_instances_organization_id_fkey;
ALTER TABLE IF EXISTS ONLY public.ip_addresses DROP CONSTRAINT IF EXISTS ip_addresses_subnet_id_fkey;
ALTER TABLE IF EXISTS ONLY public.ip_addresses DROP CONSTRAINT IF EXISTS ip_addresses_snapshot_id_fkey;
ALTER TABLE IF EXISTS ONLY public.ip_addresses DROP CONSTRAINT IF EXISTS ip_addresses_site_id_fkey;
ALTER TABLE IF EXISTS ONLY public.ip_addresses DROP CONSTRAINT IF EXISTS ip_addresses_last_discovery_fk;
ALTER TABLE IF EXISTS ONLY public.ip_addresses DROP CONSTRAINT IF EXISTS ip_addresses_host_id_fkey;
ALTER TABLE IF EXISTS ONLY public.ip_addresses DROP CONSTRAINT IF EXISTS ip_addresses_first_discovery_fk;
ALTER TABLE IF EXISTS ONLY public.invites DROP CONSTRAINT IF EXISTS invites_organization_id_fkey;
ALTER TABLE IF EXISTS ONLY public.invites DROP CONSTRAINT IF EXISTS invites_created_by_fkey;
ALTER TABLE IF EXISTS ONLY public.interfaces DROP CONSTRAINT IF EXISTS interfaces_snapshot_id_fkey;
ALTER TABLE IF EXISTS ONLY public.interfaces DROP CONSTRAINT IF EXISTS interfaces_site_id_fkey;
ALTER TABLE IF EXISTS ONLY public.interfaces DROP CONSTRAINT IF EXISTS interfaces_native_vlan_id_fkey;
ALTER TABLE IF EXISTS ONLY public.interfaces DROP CONSTRAINT IF EXISTS interfaces_last_discovery_fk;
ALTER TABLE IF EXISTS ONLY public.interfaces DROP CONSTRAINT IF EXISTS interfaces_ip_address_id_fkey;
ALTER TABLE IF EXISTS ONLY public.interfaces DROP CONSTRAINT IF EXISTS interfaces_host_id_fkey;
ALTER TABLE IF EXISTS ONLY public.interfaces DROP CONSTRAINT IF EXISTS interfaces_first_discovery_fk;
ALTER TABLE IF EXISTS ONLY public.interface_neighbor_interfaces DROP CONSTRAINT IF EXISTS interface_neighbor_interfaces_snapshot_id_fkey;
ALTER TABLE IF EXISTS ONLY public.interface_neighbor_interfaces DROP CONSTRAINT IF EXISTS interface_neighbor_interfaces_site_id_fkey;
ALTER TABLE IF EXISTS ONLY public.interface_neighbor_interfaces DROP CONSTRAINT IF EXISTS interface_neighbor_interfaces_neighbor_interface_id_fkey;
ALTER TABLE IF EXISTS ONLY public.interface_neighbor_interfaces DROP CONSTRAINT IF EXISTS interface_neighbor_interfaces_interface_id_fkey;
ALTER TABLE IF EXISTS ONLY public.interface_neighbor_hosts DROP CONSTRAINT IF EXISTS interface_neighbor_hosts_snapshot_id_fkey;
ALTER TABLE IF EXISTS ONLY public.interface_neighbor_hosts DROP CONSTRAINT IF EXISTS interface_neighbor_hosts_site_id_fkey;
ALTER TABLE IF EXISTS ONLY public.interface_neighbor_hosts DROP CONSTRAINT IF EXISTS interface_neighbor_hosts_neighbor_host_id_fkey;
ALTER TABLE IF EXISTS ONLY public.interface_neighbor_hosts DROP CONSTRAINT IF EXISTS interface_neighbor_hosts_interface_id_fkey;
ALTER TABLE IF EXISTS ONLY public.interface_neighbor_candidates DROP CONSTRAINT IF EXISTS interface_neighbor_candidates_site_id_fkey;
ALTER TABLE IF EXISTS ONLY public.interface_neighbor_candidates DROP CONSTRAINT IF EXISTS interface_neighbor_candidates_interface_id_fkey;
ALTER TABLE IF EXISTS ONLY public.hosts DROP CONSTRAINT IF EXISTS hosts_virtualization_service_id_fkey;
ALTER TABLE IF EXISTS ONLY public.hosts DROP CONSTRAINT IF EXISTS hosts_virtualization_interface_id_fkey;
ALTER TABLE IF EXISTS ONLY public.hosts DROP CONSTRAINT IF EXISTS hosts_snapshot_id_fkey;
ALTER TABLE IF EXISTS ONLY public.hosts DROP CONSTRAINT IF EXISTS hosts_site_id_fkey;
ALTER TABLE IF EXISTS ONLY public.hosts DROP CONSTRAINT IF EXISTS hosts_last_discovery_fk;
ALTER TABLE IF EXISTS ONLY public.hosts DROP CONSTRAINT IF EXISTS hosts_first_discovery_fk;
ALTER TABLE IF EXISTS ONLY public.host_credentials DROP CONSTRAINT IF EXISTS host_credentials_host_id_fkey;
ALTER TABLE IF EXISTS ONLY public.host_credentials DROP CONSTRAINT IF EXISTS host_credentials_credential_id_fkey;
ALTER TABLE IF EXISTS ONLY public.dependency_members DROP CONSTRAINT IF EXISTS group_bindings_group_id_fkey;
ALTER TABLE IF EXISTS ONLY public.entity_tags DROP CONSTRAINT IF EXISTS entity_tags_tag_id_fkey;
ALTER TABLE IF EXISTS ONLY public.entity_tags DROP CONSTRAINT IF EXISTS entity_tags_snapshot_id_fkey;
ALTER TABLE IF EXISTS ONLY public.discovery_warnings DROP CONSTRAINT IF EXISTS discovery_warnings_subnet_id_fkey;
ALTER TABLE IF EXISTS ONLY public.discovery_warnings DROP CONSTRAINT IF EXISTS discovery_warnings_site_id_fkey;
ALTER TABLE IF EXISTS ONLY public.discovery_warnings DROP CONSTRAINT IF EXISTS discovery_warnings_remote_host_id_fkey;
ALTER TABLE IF EXISTS ONLY public.discovery_warnings DROP CONSTRAINT IF EXISTS discovery_warnings_host_id_fkey;
ALTER TABLE IF EXISTS ONLY public.discovery_warnings DROP CONSTRAINT IF EXISTS discovery_warnings_discovery_id_fkey;
ALTER TABLE IF EXISTS ONLY public.discovery_warnings DROP CONSTRAINT IF EXISTS discovery_warnings_credential_id_fkey;
ALTER TABLE IF EXISTS ONLY public.discovery DROP CONSTRAINT IF EXISTS discovery_site_id_fkey;
ALTER TABLE IF EXISTS ONLY public.discovery DROP CONSTRAINT IF EXISTS discovery_daemon_id_fkey;
ALTER TABLE IF EXISTS ONLY public.dependency_members DROP CONSTRAINT IF EXISTS dependency_members_snapshot_id_fkey;
ALTER TABLE IF EXISTS ONLY public.dependencies DROP CONSTRAINT IF EXISTS dependencies_snapshot_id_fkey;
ALTER TABLE IF EXISTS ONLY public.dependencies DROP CONSTRAINT IF EXISTS dependencies_site_id_fkey;
ALTER TABLE IF EXISTS ONLY public.daemons DROP CONSTRAINT IF EXISTS daemons_user_id_fkey;
ALTER TABLE IF EXISTS ONLY public.daemons DROP CONSTRAINT IF EXISTS daemons_site_id_fkey;
ALTER TABLE IF EXISTS ONLY public.daemons DROP CONSTRAINT IF EXISTS daemons_api_key_id_fkey;
ALTER TABLE IF EXISTS ONLY public.daemon_interfaced_subnets DROP CONSTRAINT IF EXISTS daemon_interfaced_subnets_subnet_id_fkey;
ALTER TABLE IF EXISTS ONLY public.daemon_interfaced_subnets DROP CONSTRAINT IF EXISTS daemon_interfaced_subnets_daemon_id_fkey;
ALTER TABLE IF EXISTS ONLY public.credentials DROP CONSTRAINT IF EXISTS credentials_organization_id_fkey;
ALTER TABLE IF EXISTS ONLY public.bindings DROP CONSTRAINT IF EXISTS bindings_snapshot_id_fkey;
ALTER TABLE IF EXISTS ONLY public.bindings DROP CONSTRAINT IF EXISTS bindings_site_id_fkey;
ALTER TABLE IF EXISTS ONLY public.bindings DROP CONSTRAINT IF EXISTS bindings_service_id_fkey;
ALTER TABLE IF EXISTS ONLY public.bindings DROP CONSTRAINT IF EXISTS bindings_port_id_fkey;
ALTER TABLE IF EXISTS ONLY public.bindings DROP CONSTRAINT IF EXISTS bindings_last_discovery_fk;
ALTER TABLE IF EXISTS ONLY public.bindings DROP CONSTRAINT IF EXISTS bindings_ip_address_id_fkey;
ALTER TABLE IF EXISTS ONLY public.bindings DROP CONSTRAINT IF EXISTS bindings_first_discovery_fk;
ALTER TABLE IF EXISTS ONLY public.api_keys DROP CONSTRAINT IF EXISTS api_keys_site_id_fkey;
ALTER TABLE IF EXISTS ONLY public.api_keys DROP CONSTRAINT IF EXISTS api_keys_daemon_id_fkey;
DROP TRIGGER IF EXISTS reassign_daemons_before_user_delete ON public.users;
DROP INDEX IF EXISTS public.idx_vlans_snapshot_id;
DROP INDEX IF EXISTS public.idx_vlans_site_number_live;
DROP INDEX IF EXISTS public.idx_vlans_site;
DROP INDEX IF EXISTS public.idx_vlans_organization;
DROP INDEX IF EXISTS public.idx_vlans_live;
DROP INDEX IF EXISTS public.idx_vlans_lineage;
DROP INDEX IF EXISTS public.idx_vlans_as_of;
DROP INDEX IF EXISTS public.idx_users_password_reset_token;
DROP INDEX IF EXISTS public.idx_users_organization;
DROP INDEX IF EXISTS public.idx_users_oidc_provider_subject;
DROP INDEX IF EXISTS public.idx_users_email_verification_token;
DROP INDEX IF EXISTS public.idx_users_email_unsubscribe_token;
DROP INDEX IF EXISTS public.idx_users_email_lower;
DROP INDEX IF EXISTS public.idx_user_site_access_user;
DROP INDEX IF EXISTS public.idx_user_site_access_site;
DROP INDEX IF EXISTS public.idx_user_api_keys_user;
DROP INDEX IF EXISTS public.idx_user_api_keys_org;
DROP INDEX IF EXISTS public.idx_user_api_keys_key;
DROP INDEX IF EXISTS public.idx_user_api_key_site_access_site;
DROP INDEX IF EXISTS public.idx_user_api_key_site_access_key;
DROP INDEX IF EXISTS public.idx_topologies_site;
DROP INDEX IF EXISTS public.idx_tags_tag_group_id;
DROP INDEX IF EXISTS public.idx_tags_snapshot_id;
DROP INDEX IF EXISTS public.idx_tags_organization;
DROP INDEX IF EXISTS public.idx_tags_org_name_live;
DROP INDEX IF EXISTS public.idx_tags_live;
DROP INDEX IF EXISTS public.idx_tags_lineage;
DROP INDEX IF EXISTS public.idx_tags_as_of;
DROP INDEX IF EXISTS public.idx_tag_groups_org_system;
DROP INDEX IF EXISTS public.idx_tag_groups_org_name;
DROP INDEX IF EXISTS public.idx_subnets_virtualization_service_id;
DROP INDEX IF EXISTS public.idx_subnets_snapshot_id;
DROP INDEX IF EXISTS public.idx_subnets_site;
DROP INDEX IF EXISTS public.idx_subnets_live;
DROP INDEX IF EXISTS public.idx_subnets_lineage;
DROP INDEX IF EXISTS public.idx_subnets_as_of;
DROP INDEX IF EXISTS public.idx_subnet_vlans_vlan;
DROP INDEX IF EXISTS public.idx_subnet_vlans_unique_live;
DROP INDEX IF EXISTS public.idx_subnet_vlans_subnet;
DROP INDEX IF EXISTS public.idx_subnet_vlans_snapshot_id;
DROP INDEX IF EXISTS public.idx_subnet_vlans_live;
DROP INDEX IF EXISTS public.idx_subnet_vlans_lineage;
DROP INDEX IF EXISTS public.idx_subnet_vlans_as_of;
DROP INDEX IF EXISTS public.idx_snapshots_site_taken_at;
DROP INDEX IF EXISTS public.idx_sites_owner_organization;
DROP INDEX IF EXISTS public.idx_shares_topology;
DROP INDEX IF EXISTS public.idx_shares_site;
DROP INDEX IF EXISTS public.idx_shares_enabled;
DROP INDEX IF EXISTS public.idx_services_virtualization_service_id;
DROP INDEX IF EXISTS public.idx_services_snapshot_id;
DROP INDEX IF EXISTS public.idx_services_site;
DROP INDEX IF EXISTS public.idx_services_live;
DROP INDEX IF EXISTS public.idx_services_lineage;
DROP INDEX IF EXISTS public.idx_services_host_position;
DROP INDEX IF EXISTS public.idx_services_host_id;
DROP INDEX IF EXISTS public.idx_services_as_of;
DROP INDEX IF EXISTS public.idx_release_announcements_version;
DROP INDEX IF EXISTS public.idx_ports_unique_live;
DROP INDEX IF EXISTS public.idx_ports_snapshot_id;
DROP INDEX IF EXISTS public.idx_ports_site;
DROP INDEX IF EXISTS public.idx_ports_number;
DROP INDEX IF EXISTS public.idx_ports_live;
DROP INDEX IF EXISTS public.idx_ports_lineage;
DROP INDEX IF EXISTS public.idx_ports_host;
DROP INDEX IF EXISTS public.idx_ports_as_of;
DROP INDEX IF EXISTS public.idx_organizations_stripe_customer;
DROP INDEX IF EXISTS public.idx_license_instances_organization;
DROP INDEX IF EXISTS public.idx_ip_addresses_unique_live;
DROP INDEX IF EXISTS public.idx_ip_addresses_subnet;
DROP INDEX IF EXISTS public.idx_ip_addresses_snapshot_id;
DROP INDEX IF EXISTS public.idx_ip_addresses_site_mac;
DROP INDEX IF EXISTS public.idx_ip_addresses_site;
DROP INDEX IF EXISTS public.idx_ip_addresses_live;
DROP INDEX IF EXISTS public.idx_ip_addresses_lineage;
DROP INDEX IF EXISTS public.idx_ip_addresses_host_mac;
DROP INDEX IF EXISTS public.idx_ip_addresses_host;
DROP INDEX IF EXISTS public.idx_ip_addresses_as_of;
DROP INDEX IF EXISTS public.idx_invites_organization;
DROP INDEX IF EXISTS public.idx_invites_expires_at;
DROP INDEX IF EXISTS public.idx_interfaces_snapshot_id;
DROP INDEX IF EXISTS public.idx_interfaces_site;
DROP INDEX IF EXISTS public.idx_interfaces_mac_address;
DROP INDEX IF EXISTS public.idx_interfaces_live;
DROP INDEX IF EXISTS public.idx_interfaces_lineage;
DROP INDEX IF EXISTS public.idx_interfaces_ip_address;
DROP INDEX IF EXISTS public.idx_interfaces_host_name_live;
DROP INDEX IF EXISTS public.idx_interfaces_host_if_index;
DROP INDEX IF EXISTS public.idx_interfaces_host;
DROP INDEX IF EXISTS public.idx_interfaces_as_of;
DROP INDEX IF EXISTS public.idx_interface_neighbor_interfaces_snapshot_id;
DROP INDEX IF EXISTS public.idx_interface_neighbor_interfaces_neighbor;
DROP INDEX IF EXISTS public.idx_interface_neighbor_interfaces_natural_key;
DROP INDEX IF EXISTS public.idx_interface_neighbor_interfaces_live;
DROP INDEX IF EXISTS public.idx_interface_neighbor_interfaces_lineage;
DROP INDEX IF EXISTS public.idx_interface_neighbor_interfaces_as_of;
DROP INDEX IF EXISTS public.idx_interface_neighbor_hosts_snapshot_id;
DROP INDEX IF EXISTS public.idx_interface_neighbor_hosts_neighbor;
DROP INDEX IF EXISTS public.idx_interface_neighbor_hosts_natural_key;
DROP INDEX IF EXISTS public.idx_interface_neighbor_hosts_live;
DROP INDEX IF EXISTS public.idx_interface_neighbor_hosts_lineage;
DROP INDEX IF EXISTS public.idx_interface_neighbor_hosts_as_of;
DROP INDEX IF EXISTS public.idx_interface_neighbor_candidates_site;
DROP INDEX IF EXISTS public.idx_interface_neighbor_candidates_interface;
DROP INDEX IF EXISTS public.idx_hosts_virtualization_service_id;
DROP INDEX IF EXISTS public.idx_hosts_virtualization_interface_id;
DROP INDEX IF EXISTS public.idx_hosts_snapshot_id;
DROP INDEX IF EXISTS public.idx_hosts_site;
DROP INDEX IF EXISTS public.idx_hosts_live;
DROP INDEX IF EXISTS public.idx_hosts_lineage;
DROP INDEX IF EXISTS public.idx_hosts_chassis_id;
DROP INDEX IF EXISTS public.idx_hosts_as_of;
DROP INDEX IF EXISTS public.idx_entity_tags_unique_live;
DROP INDEX IF EXISTS public.idx_entity_tags_tag_id;
DROP INDEX IF EXISTS public.idx_entity_tags_snapshot_id;
DROP INDEX IF EXISTS public.idx_entity_tags_live;
DROP INDEX IF EXISTS public.idx_entity_tags_lineage;
DROP INDEX IF EXISTS public.idx_entity_tags_entity;
DROP INDEX IF EXISTS public.idx_entity_tags_as_of;
DROP INDEX IF EXISTS public.idx_discovery_warnings_subnet;
DROP INDEX IF EXISTS public.idx_discovery_warnings_site;
DROP INDEX IF EXISTS public.idx_discovery_warnings_remote_host;
DROP INDEX IF EXISTS public.idx_discovery_warnings_host;
DROP INDEX IF EXISTS public.idx_discovery_warnings_discovery;
DROP INDEX IF EXISTS public.idx_discovery_warnings_credential;
DROP INDEX IF EXISTS public.idx_discovery_site;
DROP INDEX IF EXISTS public.idx_discovery_daemon;
DROP INDEX IF EXISTS public.idx_dependency_members_unique_live;
DROP INDEX IF EXISTS public.idx_dependency_members_snapshot_id;
DROP INDEX IF EXISTS public.idx_dependency_members_service;
DROP INDEX IF EXISTS public.idx_dependency_members_live;
DROP INDEX IF EXISTS public.idx_dependency_members_lineage;
DROP INDEX IF EXISTS public.idx_dependency_members_dependency;
DROP INDEX IF EXISTS public.idx_dependency_members_binding;
DROP INDEX IF EXISTS public.idx_dependency_members_as_of;
DROP INDEX IF EXISTS public.idx_dependencies_snapshot_id;
DROP INDEX IF EXISTS public.idx_dependencies_site;
DROP INDEX IF EXISTS public.idx_dependencies_live;
DROP INDEX IF EXISTS public.idx_dependencies_lineage;
DROP INDEX IF EXISTS public.idx_dependencies_as_of;
DROP INDEX IF EXISTS public.idx_daemons_site;
DROP INDEX IF EXISTS public.idx_daemons_api_key;
DROP INDEX IF EXISTS public.idx_daemon_interfaced_subnets_subnet_id;
DROP INDEX IF EXISTS public.idx_daemon_host_id;
DROP INDEX IF EXISTS public.idx_credentials_type;
DROP INDEX IF EXISTS public.idx_credentials_org;
DROP INDEX IF EXISTS public.idx_bindings_snapshot_id;
DROP INDEX IF EXISTS public.idx_bindings_site;
DROP INDEX IF EXISTS public.idx_bindings_service;
DROP INDEX IF EXISTS public.idx_bindings_port;
DROP INDEX IF EXISTS public.idx_bindings_live;
DROP INDEX IF EXISTS public.idx_bindings_lineage;
DROP INDEX IF EXISTS public.idx_bindings_ip_address;
DROP INDEX IF EXISTS public.idx_bindings_as_of;
DROP INDEX IF EXISTS public.idx_api_keys_site;
DROP INDEX IF EXISTS public.idx_api_keys_key;
DROP INDEX IF EXISTS public.idx_api_keys_daemon_id;
ALTER TABLE IF EXISTS ONLY tower_sessions.session DROP CONSTRAINT IF EXISTS session_pkey;
ALTER TABLE IF EXISTS ONLY public.vlans DROP CONSTRAINT IF EXISTS vlans_pkey;
ALTER TABLE IF EXISTS ONLY public.users DROP CONSTRAINT IF EXISTS users_pkey;
ALTER TABLE IF EXISTS ONLY public.user_site_access DROP CONSTRAINT IF EXISTS user_site_access_user_id_site_id_key;
ALTER TABLE IF EXISTS ONLY public.user_site_access DROP CONSTRAINT IF EXISTS user_site_access_pkey;
ALTER TABLE IF EXISTS ONLY public.user_api_keys DROP CONSTRAINT IF EXISTS user_api_keys_pkey;
ALTER TABLE IF EXISTS ONLY public.user_api_keys DROP CONSTRAINT IF EXISTS user_api_keys_key_key;
ALTER TABLE IF EXISTS ONLY public.user_api_key_site_access DROP CONSTRAINT IF EXISTS user_api_key_site_access_pkey;
ALTER TABLE IF EXISTS ONLY public.user_api_key_site_access DROP CONSTRAINT IF EXISTS user_api_key_site_access_api_key_id_site_id_key;
ALTER TABLE IF EXISTS ONLY public.topologies DROP CONSTRAINT IF EXISTS topologies_pkey;
ALTER TABLE IF EXISTS ONLY public.tags DROP CONSTRAINT IF EXISTS tags_pkey;
ALTER TABLE IF EXISTS ONLY public.tag_groups DROP CONSTRAINT IF EXISTS tag_groups_pkey;
ALTER TABLE IF EXISTS ONLY public.subnets DROP CONSTRAINT IF EXISTS subnets_pkey;
ALTER TABLE IF EXISTS ONLY public.subnet_vlans DROP CONSTRAINT IF EXISTS subnet_vlans_pkey;
ALTER TABLE IF EXISTS ONLY public.snapshots DROP CONSTRAINT IF EXISTS snapshots_pkey;
ALTER TABLE IF EXISTS ONLY public.sites DROP CONSTRAINT IF EXISTS sites_pkey;
ALTER TABLE IF EXISTS ONLY public.site_credentials DROP CONSTRAINT IF EXISTS site_credentials_pkey;
ALTER TABLE IF EXISTS ONLY public.shares DROP CONSTRAINT IF EXISTS shares_pkey;
ALTER TABLE IF EXISTS ONLY public.services DROP CONSTRAINT IF EXISTS services_pkey;
ALTER TABLE IF EXISTS ONLY public.server_instance DROP CONSTRAINT IF EXISTS server_instance_pkey;
ALTER TABLE IF EXISTS ONLY public.release_announcements DROP CONSTRAINT IF EXISTS release_announcements_pkey;
ALTER TABLE IF EXISTS ONLY public.ports DROP CONSTRAINT IF EXISTS ports_pkey;
ALTER TABLE IF EXISTS ONLY public.organizations DROP CONSTRAINT IF EXISTS organizations_pkey;
ALTER TABLE IF EXISTS ONLY public.license_instances DROP CONSTRAINT IF EXISTS license_instances_pkey;
ALTER TABLE IF EXISTS ONLY public.ip_addresses DROP CONSTRAINT IF EXISTS ip_addresses_pkey;
ALTER TABLE IF EXISTS ONLY public.invites DROP CONSTRAINT IF EXISTS invites_pkey;
ALTER TABLE IF EXISTS ONLY public.interfaces DROP CONSTRAINT IF EXISTS interfaces_pkey;
ALTER TABLE IF EXISTS ONLY public.interface_neighbor_interfaces DROP CONSTRAINT IF EXISTS interface_neighbor_interfaces_pkey;
ALTER TABLE IF EXISTS ONLY public.interface_neighbor_hosts DROP CONSTRAINT IF EXISTS interface_neighbor_hosts_pkey;
ALTER TABLE IF EXISTS ONLY public.interface_neighbor_candidates DROP CONSTRAINT IF EXISTS interface_neighbor_candidates_pkey;
ALTER TABLE IF EXISTS ONLY public.hosts DROP CONSTRAINT IF EXISTS hosts_pkey;
ALTER TABLE IF EXISTS ONLY public.host_credentials DROP CONSTRAINT IF EXISTS host_credentials_pkey;
ALTER TABLE IF EXISTS ONLY public.dependencies DROP CONSTRAINT IF EXISTS groups_pkey;
ALTER TABLE IF EXISTS ONLY public.dependency_members DROP CONSTRAINT IF EXISTS group_bindings_pkey;
ALTER TABLE IF EXISTS ONLY public.entity_tags DROP CONSTRAINT IF EXISTS entity_tags_pkey;
ALTER TABLE IF EXISTS ONLY public.discovery_warnings DROP CONSTRAINT IF EXISTS discovery_warnings_pkey;
ALTER TABLE IF EXISTS ONLY public.discovery DROP CONSTRAINT IF EXISTS discovery_pkey;
ALTER TABLE IF EXISTS ONLY public.daemons DROP CONSTRAINT IF EXISTS daemons_pkey;
ALTER TABLE IF EXISTS ONLY public.daemon_interfaced_subnets DROP CONSTRAINT IF EXISTS daemon_interfaced_subnets_pkey;
ALTER TABLE IF EXISTS ONLY public.credentials DROP CONSTRAINT IF EXISTS credentials_pkey;
ALTER TABLE IF EXISTS ONLY public.bindings DROP CONSTRAINT IF EXISTS bindings_pkey;
ALTER TABLE IF EXISTS ONLY public.api_keys DROP CONSTRAINT IF EXISTS api_keys_pkey;
ALTER TABLE IF EXISTS ONLY public.api_keys DROP CONSTRAINT IF EXISTS api_keys_key_key;
ALTER TABLE IF EXISTS ONLY public._sqlx_migrations DROP CONSTRAINT IF EXISTS _sqlx_migrations_pkey;
DROP TABLE IF EXISTS tower_sessions.session;
DROP TABLE IF EXISTS public.vlans;
DROP TABLE IF EXISTS public.users;
DROP TABLE IF EXISTS public.user_site_access;
DROP TABLE IF EXISTS public.user_api_keys;
DROP TABLE IF EXISTS public.user_api_key_site_access;
DROP TABLE IF EXISTS public.topologies;
DROP TABLE IF EXISTS public.tags;
DROP TABLE IF EXISTS public.tag_groups;
DROP TABLE IF EXISTS public.subnets;
DROP TABLE IF EXISTS public.subnet_vlans;
DROP TABLE IF EXISTS public.snapshots;
DROP TABLE IF EXISTS public.sites;
DROP TABLE IF EXISTS public.site_credentials;
DROP TABLE IF EXISTS public.shares;
DROP TABLE IF EXISTS public.services;
DROP TABLE IF EXISTS public.server_instance;
DROP TABLE IF EXISTS public.release_announcements;
DROP TABLE IF EXISTS public.ports;
DROP TABLE IF EXISTS public.organizations;
DROP TABLE IF EXISTS public.license_instances;
DROP TABLE IF EXISTS public.ip_addresses;
DROP TABLE IF EXISTS public.invites;
DROP TABLE IF EXISTS public.interfaces;
DROP TABLE IF EXISTS public.interface_neighbor_interfaces;
DROP TABLE IF EXISTS public.interface_neighbor_hosts;
DROP TABLE IF EXISTS public.interface_neighbor_candidates;
DROP TABLE IF EXISTS public.hosts;
DROP TABLE IF EXISTS public.host_credentials;
DROP TABLE IF EXISTS public.entity_tags;
DROP TABLE IF EXISTS public.discovery_warnings;
DROP TABLE IF EXISTS public.discovery;
DROP TABLE IF EXISTS public.dependency_members;
DROP TABLE IF EXISTS public.dependencies;
DROP TABLE IF EXISTS public.daemons;
DROP TABLE IF EXISTS public.daemon_interfaced_subnets;
DROP TABLE IF EXISTS public.credentials;
DROP TABLE IF EXISTS public.bindings;
DROP TABLE IF EXISTS public.api_keys;
DROP TABLE IF EXISTS public._sqlx_migrations;
DROP FUNCTION IF EXISTS public.reassign_daemons_on_user_delete();
DROP EXTENSION IF EXISTS pgcrypto;
DROP SCHEMA IF EXISTS tower_sessions;
--
-- Name: tower_sessions; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA tower_sessions;


ALTER SCHEMA tower_sessions OWNER TO postgres;

--
-- Name: pgcrypto; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS pgcrypto WITH SCHEMA public;


--
-- Name: EXTENSION pgcrypto; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION pgcrypto IS 'cryptographic functions';


--
-- Name: reassign_daemons_on_user_delete(); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.reassign_daemons_on_user_delete() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
DECLARE
    new_owner_id UUID;
BEGIN
    SELECT id INTO new_owner_id
    FROM users
    WHERE organization_id = OLD.organization_id
      AND permissions = 'Owner'
      AND id != OLD.id
    ORDER BY created_at ASC
    LIMIT 1;

    IF new_owner_id IS NOT NULL THEN
        UPDATE daemons
        SET user_id = new_owner_id
        WHERE user_id = OLD.id;
    END IF;

    RETURN OLD;
END;
$$;


ALTER FUNCTION public.reassign_daemons_on_user_delete() OWNER TO postgres;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: _sqlx_migrations; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public._sqlx_migrations (
    version bigint NOT NULL,
    description text NOT NULL,
    installed_on timestamp with time zone DEFAULT now() NOT NULL,
    success boolean NOT NULL,
    checksum bytea NOT NULL,
    execution_time bigint NOT NULL
);


ALTER TABLE public._sqlx_migrations OWNER TO postgres;

--
-- Name: api_keys; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.api_keys (
    id uuid NOT NULL,
    key text NOT NULL,
    site_id uuid NOT NULL,
    name text NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    last_used timestamp with time zone,
    expires_at timestamp with time zone,
    is_enabled boolean DEFAULT true NOT NULL,
    plaintext text,
    daemon_id uuid
);


ALTER TABLE public.api_keys OWNER TO postgres;

--
-- Name: bindings; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.bindings (
    id uuid NOT NULL,
    site_id uuid NOT NULL,
    service_id uuid NOT NULL,
    binding_type text NOT NULL,
    ip_address_id uuid,
    port_id uuid,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    valid_from timestamp with time zone DEFAULT now() NOT NULL,
    valid_to timestamp with time zone,
    lineage_id uuid,
    last_seen_at timestamp with time zone DEFAULT now() NOT NULL,
    last_discovery_id uuid,
    first_discovery_id uuid,
    snapshot_id uuid,
    CONSTRAINT bindings_binding_type_check CHECK ((binding_type = ANY (ARRAY['IPAddress'::text, 'Port'::text]))),
    CONSTRAINT valid_binding CHECK ((((binding_type = 'IPAddress'::text) AND (ip_address_id IS NOT NULL) AND (port_id IS NULL)) OR ((binding_type = 'Port'::text) AND (port_id IS NOT NULL))))
);


ALTER TABLE public.bindings OWNER TO postgres;

--
-- Name: credentials; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.credentials (
    id uuid NOT NULL,
    organization_id uuid NOT NULL,
    name text NOT NULL,
    credential_type jsonb NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    daemon_os text,
    description text
);


ALTER TABLE public.credentials OWNER TO postgres;

--
-- Name: daemon_interfaced_subnets; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.daemon_interfaced_subnets (
    daemon_id uuid NOT NULL,
    subnet_id uuid NOT NULL
);


ALTER TABLE public.daemon_interfaced_subnets OWNER TO postgres;

--
-- Name: daemons; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.daemons (
    id uuid NOT NULL,
    site_id uuid NOT NULL,
    host_id uuid NOT NULL,
    created_at timestamp with time zone NOT NULL,
    last_seen timestamp with time zone,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    mode text DEFAULT '"Push"'::text,
    url text NOT NULL,
    name text,
    version text,
    user_id uuid NOT NULL,
    api_key_id uuid,
    is_unreachable boolean DEFAULT false NOT NULL,
    standby boolean DEFAULT false NOT NULL,
    standby_cleared_at timestamp with time zone,
    os text
);


ALTER TABLE public.daemons OWNER TO postgres;

--
-- Name: dependencies; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dependencies (
    id uuid NOT NULL,
    site_id uuid NOT NULL,
    name text NOT NULL,
    description text,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    source jsonb NOT NULL,
    color text NOT NULL,
    edge_style text DEFAULT '"SmoothStep"'::text,
    dependency_type text NOT NULL,
    member_type text DEFAULT 'Bindings'::text NOT NULL,
    valid_from timestamp with time zone DEFAULT now() NOT NULL,
    valid_to timestamp with time zone,
    lineage_id uuid,
    snapshot_id uuid
);


ALTER TABLE public.dependencies OWNER TO postgres;

--
-- Name: dependency_members; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dependency_members (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    dependency_id uuid NOT NULL,
    binding_id uuid,
    "position" integer NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    service_id uuid NOT NULL,
    valid_from timestamp with time zone DEFAULT now() NOT NULL,
    valid_to timestamp with time zone,
    lineage_id uuid,
    snapshot_id uuid
);


ALTER TABLE public.dependency_members OWNER TO postgres;

--
-- Name: discovery; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.discovery (
    id uuid NOT NULL,
    site_id uuid NOT NULL,
    daemon_id uuid NOT NULL,
    run_type jsonb NOT NULL,
    discovery_type jsonb NOT NULL,
    name text NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    scan_count integer DEFAULT 0 NOT NULL,
    force_full_scan boolean DEFAULT false NOT NULL,
    integration_targets jsonb DEFAULT '[]'::jsonb NOT NULL
);


ALTER TABLE public.discovery OWNER TO postgres;

--
-- Name: discovery_warnings; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.discovery_warnings (
    id uuid NOT NULL,
    discovery_id uuid NOT NULL,
    site_id uuid NOT NULL,
    code text NOT NULL,
    credential_id uuid,
    host_id uuid,
    remote_host_id uuid,
    subnet_id uuid,
    details jsonb NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.discovery_warnings OWNER TO postgres;

--
-- Name: entity_tags; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.entity_tags (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    entity_id uuid NOT NULL,
    entity_type character varying(50) NOT NULL,
    tag_id uuid NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    valid_from timestamp with time zone DEFAULT now() NOT NULL,
    valid_to timestamp with time zone,
    lineage_id uuid,
    snapshot_id uuid
);


ALTER TABLE public.entity_tags OWNER TO postgres;

--
-- Name: host_credentials; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.host_credentials (
    host_id uuid NOT NULL,
    credential_id uuid NOT NULL,
    ip_address_ids uuid[]
);


ALTER TABLE public.host_credentials OWNER TO postgres;

--
-- Name: hosts; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.hosts (
    id uuid NOT NULL,
    site_id uuid NOT NULL,
    name text NOT NULL,
    hostname text,
    description text,
    source jsonb NOT NULL,
    virtualization_metadata jsonb,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    sys_descr text,
    sys_object_id text,
    sys_location text,
    sys_contact text,
    management_url text,
    chassis_id text,
    manufacturer text,
    model text,
    serial_number text,
    sys_name text,
    valid_from timestamp with time zone DEFAULT now() NOT NULL,
    valid_to timestamp with time zone,
    lineage_id uuid,
    last_seen_at timestamp with time zone DEFAULT now() NOT NULL,
    last_discovery_id uuid,
    first_discovery_id uuid,
    snapshot_id uuid,
    virtualization_service_id uuid,
    firmware_revision text,
    sys_descr_source jsonb DEFAULT '"Unspecified"'::jsonb NOT NULL,
    sys_object_id_source jsonb DEFAULT '"Unspecified"'::jsonb NOT NULL,
    sys_location_source jsonb DEFAULT '"Unspecified"'::jsonb NOT NULL,
    sys_contact_source jsonb DEFAULT '"Unspecified"'::jsonb NOT NULL,
    management_url_source jsonb DEFAULT '"Unspecified"'::jsonb NOT NULL,
    chassis_id_source jsonb DEFAULT '"Unspecified"'::jsonb NOT NULL,
    sys_name_source jsonb DEFAULT '"Unspecified"'::jsonb NOT NULL,
    manufacturer_source jsonb DEFAULT '"Unspecified"'::jsonb NOT NULL,
    model_source jsonb DEFAULT '"Unspecified"'::jsonb NOT NULL,
    serial_number_source jsonb DEFAULT '"Unspecified"'::jsonb NOT NULL,
    firmware_revision_source jsonb DEFAULT '"Unspecified"'::jsonb NOT NULL,
    name_source jsonb DEFAULT '"Manual"'::jsonb NOT NULL,
    software_revision text,
    software_revision_source jsonb DEFAULT '"Unspecified"'::jsonb NOT NULL,
    hostname_source jsonb DEFAULT '"Unspecified"'::jsonb NOT NULL,
    os jsonb,
    os_source jsonb DEFAULT '"Unspecified"'::jsonb NOT NULL,
    virtualization_interface_id uuid,
    asset_tag text,
    asset_tag_source jsonb DEFAULT '"Unspecified"'::jsonb NOT NULL
);


ALTER TABLE public.hosts OWNER TO postgres;

--
-- Name: COLUMN hosts.sys_descr; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.hosts.sys_descr IS 'SNMP sysDescr.0 - full system description';


--
-- Name: COLUMN hosts.sys_object_id; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.hosts.sys_object_id IS 'SNMP sysObjectID.0 - vendor OID for device identification';


--
-- Name: COLUMN hosts.sys_location; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.hosts.sys_location IS 'SNMP sysLocation.0 - physical location';


--
-- Name: COLUMN hosts.sys_contact; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.hosts.sys_contact IS 'SNMP sysContact.0 - admin contact info';


--
-- Name: COLUMN hosts.management_url; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.hosts.management_url IS 'URL for device management interface (manual or discovered)';


--
-- Name: COLUMN hosts.chassis_id; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.hosts.chassis_id IS 'LLDP lldpLocChassisId - globally unique device identifier for deduplication';


--
-- Name: interface_neighbor_candidates; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.interface_neighbor_candidates (
    id uuid NOT NULL,
    site_id uuid NOT NULL,
    interface_id uuid NOT NULL,
    lldp_chassis_id jsonb,
    lldp_port_id jsonb,
    lldp_sys_name text,
    lldp_port_desc text,
    lldp_mgmt_addr inet,
    lldp_sys_desc text,
    cdp_device_id text,
    cdp_port_id text,
    cdp_platform text,
    cdp_address inet,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    evidence_source jsonb DEFAULT '"Unspecified"'::jsonb NOT NULL
);


ALTER TABLE public.interface_neighbor_candidates OWNER TO postgres;

--
-- Name: interface_neighbor_hosts; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.interface_neighbor_hosts (
    id uuid NOT NULL,
    site_id uuid NOT NULL,
    interface_id uuid NOT NULL,
    neighbor_host_id uuid NOT NULL,
    neighbor_seen_at timestamp with time zone,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    valid_from timestamp with time zone DEFAULT now() NOT NULL,
    valid_to timestamp with time zone,
    lineage_id uuid,
    last_seen_at timestamp with time zone DEFAULT now() NOT NULL,
    last_discovery_id uuid,
    first_discovery_id uuid,
    snapshot_id uuid
);


ALTER TABLE public.interface_neighbor_hosts OWNER TO postgres;

--
-- Name: interface_neighbor_interfaces; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.interface_neighbor_interfaces (
    id uuid NOT NULL,
    site_id uuid NOT NULL,
    interface_id uuid NOT NULL,
    neighbor_interface_id uuid NOT NULL,
    neighbor_seen_at timestamp with time zone,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    valid_from timestamp with time zone DEFAULT now() NOT NULL,
    valid_to timestamp with time zone,
    lineage_id uuid,
    last_seen_at timestamp with time zone DEFAULT now() NOT NULL,
    last_discovery_id uuid,
    first_discovery_id uuid,
    snapshot_id uuid
);


ALTER TABLE public.interface_neighbor_interfaces OWNER TO postgres;

--
-- Name: interfaces; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.interfaces (
    id uuid NOT NULL,
    host_id uuid NOT NULL,
    site_id uuid NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    if_index integer,
    if_descr text,
    if_alias text,
    if_type integer,
    speed_bps bigint,
    admin_status integer,
    oper_status integer,
    mac_address macaddr,
    ip_address_id uuid,
    if_name text,
    fdb_macs jsonb,
    native_vlan_id uuid,
    vlan_ids jsonb,
    valid_from timestamp with time zone DEFAULT now() NOT NULL,
    valid_to timestamp with time zone,
    lineage_id uuid,
    last_seen_at timestamp with time zone DEFAULT now() NOT NULL,
    last_discovery_id uuid,
    first_discovery_id uuid,
    snapshot_id uuid,
    mac_address_source jsonb DEFAULT '"Unspecified"'::jsonb NOT NULL,
    ip_configured boolean DEFAULT false NOT NULL,
    interface_source jsonb DEFAULT '"Unspecified"'::jsonb NOT NULL,
    fdb_macs_source jsonb DEFAULT '"Unspecified"'::jsonb NOT NULL,
    vlan_membership_source jsonb DEFAULT '"Unspecified"'::jsonb NOT NULL
);


ALTER TABLE public.interfaces OWNER TO postgres;

--
-- Name: TABLE interfaces; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON TABLE public.interfaces IS 'SNMP ifTable entries - physical/logical interfaces on network devices';


--
-- Name: COLUMN interfaces.if_index; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.interfaces.if_index IS 'SNMP ifIndex - stable identifier within device';


--
-- Name: COLUMN interfaces.if_descr; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.interfaces.if_descr IS 'SNMP ifDescr - interface description (e.g., GigabitEthernet0/1)';


--
-- Name: COLUMN interfaces.if_alias; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.interfaces.if_alias IS 'SNMP ifAlias - user-configured description';


--
-- Name: COLUMN interfaces.if_type; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.interfaces.if_type IS 'SNMP ifType - IANAifType integer (6=ethernet, 24=loopback, etc.)';


--
-- Name: COLUMN interfaces.speed_bps; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.interfaces.speed_bps IS 'Interface speed from ifSpeed/ifHighSpeed in bits per second';


--
-- Name: COLUMN interfaces.admin_status; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.interfaces.admin_status IS 'SNMP ifAdminStatus: 1=up, 2=down, 3=testing';


--
-- Name: COLUMN interfaces.oper_status; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.interfaces.oper_status IS 'SNMP ifOperStatus: 1=up, 2=down, 3=testing, 4=unknown, 5=dormant, 6=notPresent, 7=lowerLayerDown';


--
-- Name: COLUMN interfaces.ip_address_id; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.interfaces.ip_address_id IS 'FK to Interface entity when this ifEntry has an IP address (must be on same host)';


--
-- Name: invites; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.invites (
    id uuid NOT NULL,
    organization_id uuid NOT NULL,
    permissions text NOT NULL,
    site_ids uuid[] NOT NULL,
    url text NOT NULL,
    created_by uuid NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    expires_at timestamp with time zone NOT NULL,
    send_to text
);


ALTER TABLE public.invites OWNER TO postgres;

--
-- Name: ip_addresses; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.ip_addresses (
    id uuid NOT NULL,
    site_id uuid NOT NULL,
    host_id uuid NOT NULL,
    subnet_id uuid NOT NULL,
    ip_address inet NOT NULL,
    mac_address macaddr,
    name text,
    "position" integer DEFAULT 0 NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    valid_from timestamp with time zone DEFAULT now() NOT NULL,
    valid_to timestamp with time zone,
    lineage_id uuid,
    last_seen_at timestamp with time zone DEFAULT now() NOT NULL,
    last_discovery_id uuid,
    first_discovery_id uuid,
    snapshot_id uuid,
    mac_address_source jsonb DEFAULT '"Unspecified"'::jsonb NOT NULL,
    ip_address_source jsonb DEFAULT '"Unspecified"'::jsonb NOT NULL
);


ALTER TABLE public.ip_addresses OWNER TO postgres;

--
-- Name: TABLE ip_addresses; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON TABLE public.ip_addresses IS 'IP addresses assigned to hosts on subnets';


--
-- Name: license_instances; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.license_instances (
    id uuid NOT NULL,
    organization_id uuid NOT NULL,
    instance_id uuid,
    server_version text,
    last_seen_ip inet,
    non_production boolean,
    first_seen_at timestamp with time zone NOT NULL,
    last_seen_at timestamp with time zone NOT NULL,
    active_since timestamp with time zone NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.license_instances OWNER TO postgres;

--
-- Name: organizations; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.organizations (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    name text NOT NULL,
    stripe_customer_id text,
    plan jsonb NOT NULL,
    plan_status text,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    onboarding jsonb DEFAULT '[]'::jsonb,
    has_payment_method boolean DEFAULT false NOT NULL,
    trial_end_date timestamp with time zone,
    use_case text,
    last_paused_at timestamp with time zone,
    trial_extended_used boolean DEFAULT false NOT NULL,
    last_downgrade_at timestamp with time zone,
    last_downgrade_from_plan jsonb,
    last_discount_at timestamp with time zone,
    discount_save_offer_percent_off bigint,
    discount_save_offer_active_until timestamp with time zone,
    next_renewal_at timestamp with time zone,
    notifications jsonb DEFAULT '{}'::jsonb NOT NULL,
    license_paid_through timestamp with time zone,
    license_checkin_at timestamp with time zone,
    license_key_version bigint DEFAULT 0 NOT NULL,
    license_entitlement text,
    license_entitlement_at timestamp with time zone,
    license_key_issued_at timestamp with time zone,
    license_key_type text,
    bills_by_invoice boolean DEFAULT false NOT NULL,
    license_server_version text
);


ALTER TABLE public.organizations OWNER TO postgres;

--
-- Name: TABLE organizations; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON TABLE public.organizations IS 'Organizations that own sites and have Stripe subscriptions';


--
-- Name: COLUMN organizations.plan; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.organizations.plan IS 'The current billing plan for the organization (e.g., Community, Pro)';


--
-- Name: ports; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.ports (
    id uuid NOT NULL,
    site_id uuid NOT NULL,
    host_id uuid NOT NULL,
    port_number integer NOT NULL,
    protocol text NOT NULL,
    port_type text NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    valid_from timestamp with time zone DEFAULT now() NOT NULL,
    valid_to timestamp with time zone,
    lineage_id uuid,
    last_seen_at timestamp with time zone DEFAULT now() NOT NULL,
    last_discovery_id uuid,
    first_discovery_id uuid,
    snapshot_id uuid,
    port_source jsonb DEFAULT '"Unspecified"'::jsonb NOT NULL,
    CONSTRAINT ports_port_number_check CHECK (((port_number >= 0) AND (port_number <= 65535))),
    CONSTRAINT ports_protocol_check CHECK ((protocol = ANY (ARRAY['Tcp'::text, 'Udp'::text])))
);


ALTER TABLE public.ports OWNER TO postgres;

--
-- Name: release_announcements; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.release_announcements (
    id uuid NOT NULL,
    version text NOT NULL,
    email_sent_at timestamp with time zone,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.release_announcements OWNER TO postgres;

--
-- Name: server_instance; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.server_instance (
    id uuid NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.server_instance OWNER TO postgres;

--
-- Name: services; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.services (
    id uuid NOT NULL,
    site_id uuid NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    name text NOT NULL,
    host_id uuid NOT NULL,
    service_definition text NOT NULL,
    virtualization_metadata jsonb,
    source jsonb NOT NULL,
    "position" integer DEFAULT 0 NOT NULL,
    valid_from timestamp with time zone DEFAULT now() NOT NULL,
    valid_to timestamp with time zone,
    lineage_id uuid,
    last_seen_at timestamp with time zone DEFAULT now() NOT NULL,
    last_discovery_id uuid,
    first_discovery_id uuid,
    snapshot_id uuid,
    virtualization_service_id uuid
);


ALTER TABLE public.services OWNER TO postgres;

--
-- Name: shares; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.shares (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    topology_id uuid NOT NULL,
    site_id uuid NOT NULL,
    created_by uuid NOT NULL,
    name text NOT NULL,
    is_enabled boolean DEFAULT true NOT NULL,
    expires_at timestamp with time zone,
    password_hash text,
    allowed_domains text[],
    options jsonb NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    enabled_views jsonb
);


ALTER TABLE public.shares OWNER TO postgres;

--
-- Name: site_credentials; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.site_credentials (
    site_id uuid NOT NULL,
    credential_id uuid NOT NULL
);


ALTER TABLE public.site_credentials OWNER TO postgres;

--
-- Name: sites; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.sites (
    id uuid NOT NULL,
    name text NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    organization_id uuid NOT NULL,
    stale_after_hours bigint
);


ALTER TABLE public.sites OWNER TO postgres;

--
-- Name: COLUMN sites.organization_id; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.sites.organization_id IS 'The organization that owns and pays for this site';


--
-- Name: snapshots; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.snapshots (
    id uuid NOT NULL,
    site_id uuid NOT NULL,
    taken_at timestamp with time zone NOT NULL,
    created_by_user_id uuid,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.snapshots OWNER TO postgres;

--
-- Name: subnet_vlans; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.subnet_vlans (
    id uuid NOT NULL,
    subnet_id uuid NOT NULL,
    vlan_id uuid NOT NULL,
    created_at timestamp with time zone NOT NULL,
    valid_from timestamp with time zone DEFAULT now() NOT NULL,
    valid_to timestamp with time zone,
    lineage_id uuid,
    snapshot_id uuid
);


ALTER TABLE public.subnet_vlans OWNER TO postgres;

--
-- Name: subnets; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.subnets (
    id uuid NOT NULL,
    site_id uuid NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    cidr text NOT NULL,
    name text NOT NULL,
    description text,
    subnet_type text NOT NULL,
    source jsonb NOT NULL,
    valid_from timestamp with time zone DEFAULT now() NOT NULL,
    valid_to timestamp with time zone,
    lineage_id uuid,
    last_seen_at timestamp with time zone DEFAULT now() NOT NULL,
    last_discovery_id uuid,
    first_discovery_id uuid,
    snapshot_id uuid,
    virtualization_service_id uuid,
    cidr_source jsonb DEFAULT '"Unspecified"'::jsonb NOT NULL,
    name_source jsonb DEFAULT '"Unspecified"'::jsonb NOT NULL,
    subnet_type_source jsonb DEFAULT '"Unspecified"'::jsonb NOT NULL
);


ALTER TABLE public.subnets OWNER TO postgres;

--
-- Name: tag_groups; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.tag_groups (
    id uuid NOT NULL,
    organization_id uuid NOT NULL,
    name text NOT NULL,
    description text,
    group_type text NOT NULL,
    entity_types text[],
    ordered boolean DEFAULT true NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.tag_groups OWNER TO postgres;

--
-- Name: tags; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.tags (
    id uuid NOT NULL,
    organization_id uuid NOT NULL,
    name text NOT NULL,
    description text,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    color text NOT NULL,
    valid_from timestamp with time zone DEFAULT now() NOT NULL,
    valid_to timestamp with time zone,
    lineage_id uuid,
    snapshot_id uuid,
    icon text,
    tag_group_id uuid,
    "position" integer
);


ALTER TABLE public.tags OWNER TO postgres;

--
-- Name: topologies; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.topologies (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    site_id uuid NOT NULL,
    options jsonb NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.topologies OWNER TO postgres;

--
-- Name: user_api_key_site_access; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.user_api_key_site_access (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    api_key_id uuid NOT NULL,
    site_id uuid NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.user_api_key_site_access OWNER TO postgres;

--
-- Name: user_api_keys; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.user_api_keys (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    key text NOT NULL,
    user_id uuid NOT NULL,
    organization_id uuid NOT NULL,
    permissions text DEFAULT 'Viewer'::text NOT NULL,
    name text NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    last_used timestamp with time zone,
    expires_at timestamp with time zone,
    is_enabled boolean DEFAULT true NOT NULL
);


ALTER TABLE public.user_api_keys OWNER TO postgres;

--
-- Name: user_site_access; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.user_site_access (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    user_id uuid NOT NULL,
    site_id uuid NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.user_site_access OWNER TO postgres;

--
-- Name: users; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.users (
    id uuid NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    password_hash text,
    oidc_provider text,
    oidc_subject text,
    oidc_linked_at timestamp with time zone,
    email text NOT NULL,
    organization_id uuid NOT NULL,
    permissions text DEFAULT 'Member'::text NOT NULL,
    tags uuid[] DEFAULT '{}'::uuid[] NOT NULL,
    terms_accepted_at timestamp with time zone,
    email_verified boolean DEFAULT false NOT NULL,
    email_verification_token text,
    email_verification_expires timestamp with time zone,
    password_reset_token text,
    password_reset_expires timestamp with time zone,
    pending_email text,
    email_settings jsonb DEFAULT '{"discovery_digest": true}'::jsonb NOT NULL,
    session_epoch bigint DEFAULT 0 NOT NULL,
    display_settings jsonb DEFAULT '{}'::jsonb NOT NULL,
    email_unsubscribe_token uuid,
    release_notes_seen_version text,
    terms_notice_version date
);


ALTER TABLE public.users OWNER TO postgres;

--
-- Name: COLUMN users.organization_id; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.users.organization_id IS 'The single organization this user belongs to';


--
-- Name: COLUMN users.permissions; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.users.permissions IS 'User role within their organization: Owner, Member, Viewer';


--
-- Name: vlans; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.vlans (
    id uuid NOT NULL,
    vlan_number smallint NOT NULL,
    name text NOT NULL,
    description text,
    site_id uuid NOT NULL,
    organization_id uuid NOT NULL,
    source jsonb DEFAULT '"Manual"'::jsonb NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    valid_from timestamp with time zone DEFAULT now() NOT NULL,
    valid_to timestamp with time zone,
    lineage_id uuid,
    last_seen_at timestamp with time zone DEFAULT now() NOT NULL,
    last_discovery_id uuid,
    first_discovery_id uuid,
    snapshot_id uuid,
    name_source jsonb DEFAULT '"Unspecified"'::jsonb NOT NULL
);


ALTER TABLE public.vlans OWNER TO postgres;

--
-- Name: session; Type: TABLE; Schema: tower_sessions; Owner: postgres
--

CREATE TABLE tower_sessions.session (
    id text NOT NULL,
    data bytea NOT NULL,
    expiry_date timestamp with time zone NOT NULL
);


ALTER TABLE tower_sessions.session OWNER TO postgres;

--
-- Data for Name: _sqlx_migrations; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public._sqlx_migrations (version, description, installed_on, success, checksum, execution_time) FROM stdin;
20251006215000	users	2026-10-25 01:44:52.34626+00	t	\\x4f13ce14ff67ef0b7145987c7b22b588745bf9fbb7b673450c26a0f2f9a36ef8ca980e456c8d77cfb1b2d7a4577a64d7	-1
20251006215100	networks	2026-10-25 01:44:52.353939+00	t	\\xeaa5a07a262709f64f0c59f31e25519580c79e2d1a523ce72736848946a34b17dd9adc7498eaf90551af6b7ec6d4e0e3	-1
20251006215151	create_hosts	2026-10-25 01:44:52.359812+00	t	\\x6ec7487074c0724932d21df4cf1ed66645313cf62c159a7179e39cbc261bcb81a24f7933a0e3cf58504f2a90fc5c1962	-1
20251006215155	create_subnets	2026-10-25 01:44:52.365445+00	t	\\xefb5b25742bd5f4489b67351d9f2494a95f307428c911fd8c5f475bfb03926347bdc269bbd048d2ddb06336945b27926	-1
20251006215201	create_groups	2026-10-25 01:44:52.37125+00	t	\\x0a7032bf4d33a0baf020e905da865cde240e2a09dda2f62aa535b2c5d4b26b20be30a3286f1b5192bd94cd4a5dbb5bcd	-1
20251006215204	create_daemons	2026-10-25 01:44:52.377976+00	t	\\xcfea93403b1f9cf9aac374711d4ac72d8a223e3c38a1d2a06d9edb5f94e8a557debac3668271f8176368eadc5105349f	-1
20251006215212	create_services	2026-10-25 01:44:52.385282+00	t	\\xd5b07f82fc7c9da2782a364d46078d7d16b5c08df70cfbf02edcfe9b1b24ab6024ad159292aeea455f15cfd1f4740c1d	-1
20251029193448	user-auth	2026-10-25 01:44:52.398077+00	t	\\xfde8161a8db89d51eeade7517d90a41d560f19645620f2298f78f116219a09728b18e91251ae31e46a47f6942d5a9032	-1
20251030044828	daemon_api	2026-10-25 01:44:52.400709+00	t	\\x181eb3541f51ef5b038b2064660370775d1b364547a214a20dde9c9d4bb95a1c273cd4525ef29e61fa65a3eb4fee0400	-1
20251030170438	host-hide	2026-10-25 01:44:52.402766+00	t	\\x87c6fda7f8456bf610a78e8e98803158caa0e12857c5bab466a5bb0004d41b449004a68e728ca13f17e051f662a15454	-1
20251102224919	create_discovery	2026-10-25 01:44:52.417685+00	t	\\xb32a04abb891aba48f92a059fae7341442355ca8e4af5d109e28e2a4f79ee8e11b2a8f40453b7f6725c2dd6487f26573	-1
20251106235621	normalize-daemon-cols	2026-10-25 01:44:52.421036+00	t	\\x5b137118d506e2708097c432358bf909265b3cf3bacd662b02e2c81ba589a9e0100631c7801cffd9c57bb10a6674fb3b	-1
20251107034459	api_keys	2026-10-25 01:44:52.432849+00	t	\\x3133ec043c0c6e25b6e55f7da84cae52b2a72488116938a2c669c8512c2efe72a74029912bcba1f2a2a0a8b59ef01dde	-1
20251107222650	oidc-auth	2026-10-25 01:44:52.465706+00	t	\\xd349750e0298718cbcd98eaff6e152b3fb45c3d9d62d06eedeb26c75452e9ce1af65c3e52c9f2de4bd532939c2f31096	-1
20251110181948	orgs-billing	2026-10-25 01:44:52.480828+00	t	\\x5bbea7a2dfc9d00213bd66b473289ddd66694eff8a4f3eaab937c985b64c5f8c3ad2d64e960afbb03f335ac6766687aa	-1
20251113223656	group-enhancements	2026-10-25 01:44:52.483272+00	t	\\xbe0699486d85df2bd3edc1f0bf4f1f096d5b6c5070361702c4d203ec2bb640811be88bb1979cfe51b40805ad84d1de65	-1
20251117032720	daemon-mode	2026-10-25 01:44:52.485379+00	t	\\xdd0d899c24b73d70e9970e54b2c748d6b6b55c856ca0f8590fe990da49cc46c700b1ce13f57ff65abd6711f4bd8a6481	-1
20251118143058	set-default-plan	2026-10-25 01:44:52.487858+00	t	\\xd19142607aef84aac7cfb97d60d29bda764d26f513f2c72306734c03cec2651d23eee3ce6cacfd36ca52dbddc462f917	-1
20251118225043	save-topology	2026-10-25 01:44:52.500123+00	t	\\x011a594740c69d8d0f8b0149d49d1b53cfbf948b7866ebd84403394139cb66a44277803462846b06e762577adc3e61a3	-1
20251123232748	network-permissions	2026-10-25 01:44:52.504733+00	t	\\x161be7ae5721c06523d6488606f1a7b1f096193efa1183ecdd1c2c9a4a9f4cad4884e939018917314aaf261d9a3f97ae	-1
20251125001342	billing-updates	2026-10-25 01:44:52.506615+00	t	\\xa235d153d95aeb676e3310a52ccb69dfbd7ca36bba975d5bbca165ceeec7196da12119f23597ea5276c364f90f23db1e	-1
20251128035448	org-onboarding-status	2026-10-25 01:44:52.509368+00	t	\\x1d7a7e9bf23b5078250f31934d1bc47bbaf463ace887e7746af30946e843de41badfc2b213ed64912a18e07b297663d8	-1
20251129180942	nfs-consolidate	2026-10-25 01:44:52.51157+00	t	\\xb38f41d30699a475c2b967f8e43156f3b49bb10341bddbde01d9fb5ba805f6724685e27e53f7e49b6c8b59e29c74f98e	-1
20251206052641	discovery-progress	2026-10-25 01:44:52.514228+00	t	\\x9d433b7b8c58d0d5437a104497e5e214febb2d1441a3ad7c28512e7497ed14fb9458e0d4ff786962a59954cb30da1447	-1
20251206202200	plan-fix	2026-10-25 01:44:52.516056+00	t	\\x242f6699dbf485cf59a8d1b8cd9d7c43aeef635a9316be815a47e15238c5e4af88efaa0daf885be03572948dc0c9edac	-1
20251207061341	daemon-url	2026-10-25 01:44:52.519744+00	t	\\x01172455c4f2d0d57371d18ef66d2ab3b7a8525067ef8a86945c616982e6ce06f5ea1e1560a8f20dadcd5be2223e6df1	-1
20251210045929	tags	2026-10-25 01:44:52.531933+00	t	\\xe3dde83d39f8552b5afcdc1493cddfeffe077751bf55472032bc8b35fc8fc2a2caa3b55b4c2354ace7de03c3977982db	-1
20251210175035	terms	2026-10-25 01:44:52.533898+00	t	\\xe47f0cf7aba1bffa10798bede953da69fd4bfaebf9c75c76226507c558a3595c6bfc6ac8920d11398dbdf3b762769992	-1
20251213025048	hash-keys	2026-10-25 01:44:52.546929+00	t	\\xfc7cbb8ce61f0c225322297f7459dcbe362242b9001c06cb874b7f739cea7ae888d8f0cfaed6623bcbcb9ec54c8cd18b	-1
20251214050638	scanopy	2026-10-25 01:44:52.549465+00	t	\\x0108bb39832305f024126211710689adc48d973ff66e5e59ff49468389b75c1ff95d1fbbb7bdb50e33ec1333a1f29ea6	-1
20251215215724	topo-scanopy-fix	2026-10-25 01:44:52.551013+00	t	\\xed88a4b71b3c9b61d46322b5053362e5a25a9293cd3c420c9df9fcaeb3441254122b8a18f58c297f535c842b8a8b0a38	-1
20251217153736	category_rename	2026-10-25 01:44:52.553997+00	t	\\x03af7ec905e11a77e25038a3c272645da96014da7c50c585a25cea3f9a7579faba3ff45114a5e589d144c9550ba42421	-1
20251218053111	invite-persistence	2026-10-25 01:44:52.56144+00	t	\\x21d12f48b964acfd600f88e70ceb14abd9cf2a8a10db2eae2a6d8f44cf7d20749f93293631e6123e92b7c3c1793877c2	-1
20251219211216	create_shares	2026-10-25 01:44:52.57193+00	t	\\x036485debd3536f9e58ead728f461b925585911acf565970bf3b2ab295b12a2865606d6a56d334c5641dcd42adeb3d68	-1
20251220170928	permissions-cleanup	2026-10-25 01:44:52.574589+00	t	\\x632f7b6702b494301e0d36fd3b900686b1a7f9936aef8c084b5880f1152b8256a125566e2b5ac40216eaadd3c4c64a03	-1
20251220180000	commercial-to-community	2026-10-25 01:44:52.576273+00	t	\\x26fc298486c225f2f01271d611418377c403183ae51daf32fef104ec07c027f2017d138910c4fbfb5f49819a5f4194d6	-1
20251221010000	cleanup_subnet_type	2026-10-25 01:44:52.578033+00	t	\\xb521121f3fd3a10c0de816977ac2a2ffb6118f34f8474ffb9058722abc0dc4cf5cbec83bc6ee49e79a68e6b715087f40	-1
20251221020000	remove_host_target	2026-10-25 01:44:52.580183+00	t	\\x77b5f8872705676ca81a5704bd1eaee90b9a52b404bdaa27a23da2ffd4858d3e131680926a5a00ad2a0d7a24ba229046	-1
20251221030000	user_network_access	2026-10-25 01:44:52.590312+00	t	\\x5c23f5bb6b0b8ca699a17eee6730c4197a006ca21fecc79136a5e5697b9211a81b4cd08ceda70dace6a26408d021ff3a	-1
20251221040000	interfaces_table	2026-10-25 01:44:52.604012+00	t	\\xf7977b6f1e7e5108c614397d03a38c9bd9243fdc422575ec29610366a0c88f443de2132185878d8e291f06a50a8c3244	-1
20251221050000	ports_table	2026-10-25 01:44:52.61854+00	t	\\xdf72f9306b405be7be62c39003ef38408115e740b120f24e8c78b8e136574fff7965c52023b3bc476899613fa5f4fe35	-1
20251221060000	bindings_table	2026-10-25 01:44:52.637738+00	t	\\x933648a724bd179c7f47305e4080db85342d48712cde39374f0f88cde9d7eba8fe5fafba360937331e2a8178dec420c4	-1
20251221070000	group_bindings	2026-10-25 01:44:52.648879+00	t	\\x697475802f6c42e38deee6596f4ba786b09f7b7cd91742fbc5696dd0f9b3ddfce90dd905153f2b1a9e82f959f5a88302	-1
20251222020000	tag_cascade_delete	2026-10-25 01:44:52.651366+00	t	\\xabfb48c0da8522f5c8ea6d482eb5a5f4562ed41f6160a5915f0fd477c7dd0517aa84760ef99ab3a5db3e0f21b0c69b5f	-1
20251223232524	network_remove_default	2026-10-25 01:44:52.65341+00	t	\\x7099fe4e52405e46269d7ce364050da930b481e72484ad3c4772fd2911d2d505476d659fa9f400c63bc287512d033e18	-1
20251225100000	color_enum	2026-10-25 01:44:52.655625+00	t	\\x62cecd9d79a49835a3bea68a7959ab62aa0c1aaa7e2940dec6a7f8a714362df3649f0c1f9313672d9268295ed5a1cfa9	-1
20251227010000	topology_snapshot_migration	2026-10-25 01:44:52.662775+00	t	\\xc042591d254869c0e79c8b52a9ede680fd26f094e2c385f5f017e115f5e3f31ad155f4885d095344f2642ebb70755d54	-1
20251228010000	user_api_keys	2026-10-25 01:44:52.678981+00	t	\\xa41adb558a5b9d94a4e17af3f16839b83f7da072dbeac9251b12d8a84c7bec6df008009acf246468712a975bb36bb5f5	-1
20251230160000	daemon_version_and_maintainer	2026-10-25 01:44:52.68326+00	t	\\xafed3d9f00adb8c1b0896fb663af801926c218472a0a197f90ecdaa13305a78846a9e15af0043ec010328ba533fca68f	-1
20260103000000	service_position	2026-10-25 01:44:52.686341+00	t	\\x19d00e8c8b300d1c74d721931f4d771ec7bc4e06db0d6a78126e00785586fdc4bcff5b832eeae2fce0cb8d01e12a7fb5	-1
20260106000000	interface_mac_index	2026-10-25 01:44:52.689209+00	t	\\xa26248372a1e31af46a9c6fbdaef178982229e2ceeb90cc6a289d5764f87a38747294b3adf5f21276b5d171e42bdb6ac	-1
20260106204402	entity_tags_junction	2026-10-25 01:44:52.706273+00	t	\\xf73c604f9f0b8db065d990a861684b0dbd62c3ef9bead120c68431c933774de56491a53f021e79f09801680152f5a08a	-1
20260108033856	fix_entity_tags_json_format	2026-10-25 01:44:52.708904+00	t	\\x197eaa063d4f96dd0e897ad8fd96cc1ba9a54dda40a93a5c12eac14597e4dea4c806dd0a527736fb5807b7a8870d9916	-1
20260110000000	email_verification	2026-10-25 01:44:52.713991+00	t	\\xb8da8433f58ba4ce846b9fa0c2551795747a8473ad10266b19685504847458ea69d27a0ce430151cfb426f5f5fb6ac3a	-1
20260114145808	daemon_user_fk_set_null	2026-10-25 01:44:52.716058+00	t	\\x57b060be9fc314d7c5851c75661ca8269118feea6cf7ee9c61b147a0e117c4d39642cf0d1acdf7a723a9a76066c1b8ff	-1
20260116010000	snmp_credentials	2026-10-25 01:44:52.72578+00	t	\\x6f3971cf194d56883c61fa795406a8ab568307ed86544920d098b32a6a1ebb7effcb5ec38a70fdc9b617eff92d63d51e	-1
20260116020000	host_snmp_fields	2026-10-25 01:44:52.731992+00	t	\\xf2f088c13ab0dd34e1cb1e5327b0b4137440b0146e5ce1e78b8d2dfa05d9b5a12a328eeb807988453a8a43ad8a1c95ba	-1
20260116030000	if_entries	2026-10-25 01:44:52.74993+00	t	\\xa58391708f8b21901ab9250af528f638a6055462f70ffddfd7c451433aacdabd62825546fa8be108f23a3cae78b8ae28	-1
20260116100000	daemon_api_key_link	2026-10-25 01:44:52.754841+00	t	\\x41088aa314ab173344a6b416280721806b2f296a32a8d8cae58c7e5717f389fe599134ed03980ed97e4b7659e99c4f82	-1
20260131190000	add_hubspot_company_id	2026-10-25 01:44:52.756798+00	t	\\x4326f95f4954e176157c1c3e034074a3e5c44da4d60bbd7a9e4b6238c9ef52a30f8b38d3c887864b6e4c1163dc062beb	-1
20260201021238	fix_service_acronym_capitalization	2026-10-25 01:44:52.759124+00	t	\\x88b010ac8f0223d880ea6a730f11dc6d27fa5de9d8747de3431e46d59f1dbf2f72ae4a87c2e52c32152549f5c1f96bb2	-1
20260204004436	add_entity_tags_to_topology	2026-10-25 01:44:52.761255+00	t	\\x3eff1a1490e77065ec861ef1b9aad8c55de0170106a42720f7931b3929b179122b16e44390b2652771bf91bba32a7757	-1
20260205120000	billing_overhaul	2026-10-25 01:44:52.764649+00	t	\\xbf850cfa0c40a3c65f574efd15fd55a4b702296203d28077a09d1c22076fee8601f2b78345aef370ab9163657de767ab	-1
20260205183207	rename_hubspot_to_brevo	2026-10-25 01:44:52.766702+00	t	\\x4678a7d80215e5eafb5e80af0daa20e2868a3b4f2112e88cb1b2b9efc87d63de3fb96c133f359b224c658789ae4b0d13	-1
20260221120000	add_plan_limit_notifications	2026-10-25 01:44:52.768942+00	t	\\xef770dac07e1d80888832f33184dc46c1d3b8185b91c507cb404468d6ad8c29cacf455178801c67aa27b6a626d3ad82d	-1
20260222120000	add_pending_email	2026-10-25 01:44:52.770832+00	t	\\xddd220f7602c44548d56849c0a8d081ecd1da1383374a11e3e227c7d9becb73a49f5e5bb09ed65901c16df4c16e913e5	-1
20260301120000	add_if_name_to_if_entries	2026-10-25 01:44:52.772673+00	t	\\xc9fc0a2b77ecbf0e1d5ab292c4fe162a26113468c878dfd26a3c63d89c0ee1957ca328ecfe25c611867a0e73780f0cb6	-1
20260306002816	cleanup_standby	2026-10-25 01:44:52.774428+00	t	\\x01b0c236a8a4d0d97f0f633b18f8cbdb92b6d72063289989b90a1b7b6b303e65e0557eb09927b2580dcb7e8ee5966c75	-1
20260309120000	add_org_use_case	2026-10-25 01:44:52.776318+00	t	\\xdb8c8a2f0f9416ba3b687fc75453d7c12c50a6f386b4784d21bd6adfc4a4a7556c637c25cf116118402bbd12c0d5aafe	-1
20260313120000	snmp_extended_discovery	2026-10-25 01:44:52.779229+00	t	\\xc4e72539099de1b830d87a169bfbabba4b8fb378a3c4c4a1dfca698adf3e403d750040d784c26d9fa343be2908064c9d	-1
20260315120000	universal_credentials	2026-10-25 01:44:52.804127+00	t	\\x87dc6f39202e81d5555df78a9d056b143f11bd22e6d7f483065f605e242a360902c72c4d5a49717e7fcc24a366bb5ff5	-1
20260315120001	discovery_scan_settings	2026-10-25 01:44:52.806391+00	t	\\xe9da183fdd8e04e574f553f61f6f33efa046cdae38c846c8077b06c5260446fb4aa39da2449bda7f1d8cf3aa9f16e158	-1
20260315120002	backfill_org_created_milestone	2026-10-25 01:44:52.808115+00	t	\\x14f886a19773cd2263d86f88479be460d21f071d5212e3789c5c40b6415c293fc7d06c7b138351cc42108f89a14fe745	-1
20260316120000	fix_jsonb_null_if_entries	2026-10-25 01:44:52.810956+00	t	\\x65c358069710f7f86d6a3e257e658c2f241cc376433c3a0317b0ec9e1876a66f9738cb65c6ab1a5c197fe40d5aa2aa2b	-1
20260319120000	rename_snmp_to_snmpv2c	2026-10-25 01:44:52.812906+00	t	\\xdce5c9461f402e1672607078b2c571f0eb30b51d46f8e9414d8909efb40693f543e49e560cb7d703db274515043aa08e	-1
20260321120000	add_discovery_scan_count	2026-10-25 01:44:52.816339+00	t	\\x6c8201ab453a51632176d534c6604e0818e28a8a4a153e33e254f4dac0f9b67c9db394082cb663ff1b25941229cf96fc	-1
20260329120000	backfill_subnet_virtualization	2026-10-25 01:44:52.81967+00	t	\\xeac50ded27603dbb5e8773604a52143c9fa8654263e7dd12d3d128ce972c2feed84600e36b2e7a79525b58c44d2ad9d3	-1
20260402120000	rename_topology_node_types	2026-10-25 01:44:52.821301+00	t	\\xc4ba06868add823f83ff1948091bdfe17dbdde80bbec6fe2cf8da2b3689aeeebbe9e9de01b1292bff3c98a74d9e6279f	-1
20260403120000	topology_grouping_rules	2026-10-25 01:44:52.823875+00	t	\\x00799da1206d7c3b3c3db90b7d14437cc054ed2d7273020342e562c619a671e008ff4fdf0365170440b392956949e730	-1
20260405120000	rename_groups_to_dependencies	2026-10-25 01:44:52.836509+00	t	\\x9ce895b456366bf6e54316b22cabd2803aa542dd3733fffa680f0a3af5c4c55a612c5ee511371206921869b7f271c35b	-1
20260406120000	add_tag_is_application_group	2026-10-25 01:44:52.839038+00	t	\\xb7a71e5fdd96ca46c9c7577003309050a93bc53ad192ac5df78e7621f3ed64f07fb29b4658f17af55732cf6dfb7958c2	-1
20260406130000	add_vlans	2026-10-25 01:44:52.854128+00	t	\\x5b3e5d10578d90b5175e5718a28d7147a21b99af2fb3e0ed171d20ee8fd8838c290f648dafdd3b72ef60ff487f7f2494	-1
20260409000000	add_vlans_to_topologies	2026-10-25 01:44:52.856524+00	t	\\x5e0b9dc670580ceec3aa6eae005a39f98733fc27dc574b7f3922f4297813facd5d610af953dfec13e09d0b99eceb3865	-1
20260410000000	rename_interfaces_and_if_entries	2026-10-25 01:44:52.866246+00	t	\\x07f54a59869f458f41f45d75f250aee26b20a426f1ec29930606841770194d6aea0e9e6253a6375fbeebcf9b49121224	-1
20260414000000	add_share_enabled_views	2026-10-25 01:44:52.868102+00	t	\\xc56514355a5977c3242e728e7f5a2533e7b4a5cf8a7ce7757e412e51f1ad85e96d65c13ccd96d050be4a07799b9aef57	-1
20260415120000	rename_onboarding_first_group_created	2026-10-25 01:44:52.869603+00	t	\\x2c17035835d3ead105b76d98688c0b7bd328abdaf9f721d70d057c8afdf438819e93da56707deea5b469b81a7b84d5d7	-1
20260417000000	reindex_interfaces_identity	2026-10-25 01:44:52.875252+00	t	\\x10701e13bc3d838e2ec4a856555ebf338173792f220c405996d3c77e7987e9806798ca0328eb6259e4a62b7e05665b25	-1
20260418000000	add_standby_cleared_at_to_daemons	2026-10-25 01:44:52.876993+00	t	\\x547807de451d015a4ce1438796d5b95e2b98043c521015a21239f6778d10a8d3bf7d8b14e278e09aa0105f1935ad4181	-1
20260501000000	add_organization_billing_flags	2026-10-25 01:44:52.880569+00	t	\\x2de34c4af667d4cd8bc263c27f0526a4a2132022e2eb71ae94fd89edebbfb40cda840055a94e89a21925a317cdff285f	-1
20260502000000	scd2_add_columns	2026-10-25 01:44:52.901736+00	t	\\xe78a73574d86320c0de7fcd43682ee3cdd436dc64d1371f98cb67ef0cdf33097df6e8a92c9a26c5702795144103b085e	-1
20260502000001	scd2_add_discovery_fks_not_valid	2026-10-25 01:44:52.918309+00	t	\\xb49114eb6f8d77cb0c5062619da5c829e5bd65898710efb6207d2804be3677a95eebe88d41d21ae8cba21344fcd63f97	-1
20260502000002	scd2_validate_discovery_fks	2026-10-25 01:44:52.931549+00	t	\\x500201842b9f486d1397c0f0f3ee1a36bc101440d4d97fe0f0bfc971672f1f53f7945aed93c34e2e53d2d9cb913419f1	-1
20260502000003	scd2_backfill_with_metadata	2026-10-25 01:44:52.938032+00	t	\\xd299c1dbf7ec284995dc8a5fcf4b264e4f1ff970085a8631e6d00aa0bc6d6708a5e41d515bb9299c22ef128a958e68e9	-1
20260502000004	scd2_partial_unique_indexes	2026-10-25 01:44:52.969626+00	t	\\xbc30e554c37ec3ca72baa7fc634920257909944dc841231a966845fd4c5bc27ae20606c98867934c872e8883bd56efa1	-1
20260502000005	scd2_drop_old_unique_constraints	2026-10-25 01:44:52.974711+00	t	\\x660c0237299796c6e70a21780d529453a2ac8645f7b122a0b8aa4f362ff780de8849f53a66b71b3ea29ecc66877f47ad	-1
20260502000006	scd2_supporting_indexes	2026-10-25 01:44:53.071799+00	t	\\x4394aaf6951c4b7048f66334fb8c550c55d4cdc06beb29eb7f086a717ee9f26ddeb28e73a7b298633ef517f6c0f84ee0	-1
20260502000007	entitysource_metadata_strip	2026-10-25 01:44:53.076894+00	t	\\x1cfd1a00b100ca0b007be6e8b55f67dde412108f299f94b419a80af45b74b75fdead95637dccb13fdd57fed2f1132446	-1
20260502100000	add_user_email_settings	2026-10-25 01:44:53.079124+00	t	\\xe886b44ca9d3d6461d82b573f617391b8b5f22f18b8113a333aa8c275545ec6455060f68ecbfdd17af86fccc9a500d7a	-1
20260502120000	create_snapshots_table	2026-10-25 01:44:53.085395+00	t	\\x18f829b255b8e11e385903e51182f04fb835aa78ef0090865ce7743f747e4aaaa7b1a7b806a377e9b12af6205aed527a	-1
20260502120001	add_snapshot_id_fks	2026-10-25 01:44:53.100346+00	t	\\x0a080ceaa2922301464b30ea7f25e1f0de5493d15424d07a8dd1f2b4bf1880cf2d25f64d5bf1863f705a1557d867afa2	-1
20260502120002	add_snapshot_id_indexes	2026-10-25 01:44:53.132242+00	t	\\xb7514ff0cf637241d21e90b48214a3a3a5076d0142c99aca39479d2a82cf35e1abdbce8ad5663ca8272d5f74004ae5f9	-1
20260502120003	topology_snapshots_backfill	2026-10-25 01:44:53.140185+00	t	\\x5435da11acf8a9fc37e0520a5b2a959fce84b16080b2a5b67adcf249dea07ae533f858c56f87542152cfc271933c1428	-1
20260502120004	drop_legacy_topology_columns	2026-10-25 01:44:53.153956+00	t	\\xfe45e73be308e6267840664a662cd679fb43469a11f06a168b8b8cfc64b6e86eb10d948271b67a0541481dcc60447769	-1
20260627120000	add_discovery_integration_targets	2026-10-25 01:44:53.156265+00	t	\\x9adf9ea4c36868a3b86067214f8550b8e866aff5d3d6c6ab3d71d1a15ea5e4be0b4349db6d15308fefbceca8403f0cbb	-1
20260703120000	daemon_interfaced_subnets_junction	2026-10-25 01:44:53.162498+00	t	\\xf1399bf8104b1d9cff6a7f66c6040829f01dd64614294295f0ecc27412acc19fd6dd20eff66ce9eadf0cf0c0e8c4353e	-1
20260703120001	drop_discovery_pending_credential_ids	2026-10-25 01:44:53.165007+00	t	\\xc2cfe6f450d000ae36149b042ebcfed938c01487d78ca56a125d72bbeca0de0548824a2396761b3dc647dc80e2e43e80	-1
20260705120000	add_user_session_epoch	2026-10-25 01:44:53.167151+00	t	\\xb296babe8e50ecdd1f50770e12d4234b47ed41cdd826c545debf5f94226672cc2d6fd28eef0483a5fdc8837f5d280d88	-1
20260706120000	drop_credentials_target_ips_and_daemons_capabilities	2026-10-25 01:44:53.169734+00	t	\\xc544c88df68334ac0b3ec180da7ba2030837707f9e3cf2b75b4d56012646c24f8f60f094cc00f05c0f446a2a845e5e98	-1
20260716120000	api_keys_daemon_id	2026-10-25 01:44:53.173057+00	t	\\x484c3b2553bcf81c95aabce4be70b1e2048e097529e04a176a0b75c2edf28ecd0892c959bbd8e8166f621a9dc2dba0dc	-1
20260716120001	api_keys_daemon_id_validate	2026-10-25 01:44:53.175362+00	t	\\x723190451e98f9394b2134dccb89f8eeeb15f6a5644fb88cb24e9e0e7a4ebf5a0c455659930ce2d3096e20c004d7da89	-1
20260716120002	api_keys_daemon_id_unique	2026-10-25 01:44:53.179249+00	t	\\xf788be5a414917ec5c8b9040f85acd352ae3f580dab7e8833db5df19b7d2cf3b43b8a021c03a5082da27eca3bb7a07f0	-1
20260722120000	add_networks_stale_after_hours	2026-10-25 01:44:53.180961+00	t	\\x308d9e04ef943560d3e420c59a3c8017064d59bd28cb63f99c4b562124e2fa038630e71405171901d731420bd9de4e2f	-1
20260727120000	rename_plan_limit_notifications_to_notifications	2026-10-25 01:44:53.183522+00	t	\\x2447c735b8ac6885ae4341738294392f367abcc7c0f5e857bb08523dc5d5a8eccf77db5462b531370a0795135757098d	-1
20260728120000	rename_stale_if_entries_constraints	2026-10-25 01:44:53.188002+00	t	\\x577c54966f8c146a6e5101eb8a2f7f006ef7899ba95ec876be10970b3483b622b254d00dc9c9582efb1862404315764b	-1
20260803120000	drop_organizations_plan_limit_notifications	2026-10-25 01:44:53.190455+00	t	\\xc4c9b44a4cce6cf419cffcbbeade3d5eb119b51f321bdcd3c51a1046d8ec8df81d32f6e76c6472eefdcfae9899ef3107	-1
20260803120001	virtualization_service_id_column	2026-10-25 01:44:53.201621+00	t	\\xe07f6fee751430c94141be6d7ba17390f08f60bdcd01a65072708382b27e9543a9f27fb0cc76bb824940794d6ea67553	-1
20260819120000	hosts_name_source	2026-10-25 01:44:53.208388+00	t	\\x6526859d261bf724e6ac3bb2a51c8588764aab15ecdfad964e06e98bb59f05a753ea2950bdce40cbf7f945d157a230aa	-1
20260824120000	interfaces_neighbor_seen_at	2026-10-25 01:44:53.21101+00	t	\\x70a26934dcc110484dc95100cbc96879504a9bc0ae0fb12ca0effe032816e3282e255d90145c4d2c6bfe2eea351cb4c3	-1
20260826120000	subnets_cidr_source	2026-10-25 01:44:53.215107+00	t	\\x1d8c6a9258f1fd07a08a268cddf5f164bb3b8bb0af34d6d167e480d50b649cbdc5a7b89aa419a218266e9aa1b3d3b497	-1
20260827120000	hosts_firmware_revision	2026-10-25 01:44:53.219049+00	t	\\xa9b3a89986c699c70232d37f5909ab5fecd9c4d2c8708a11b2cad4775494a12080c6664964f942b0079a883f3ecd6ea2	-1
20260827130000	interfaces_unknown_port_facts	2026-10-25 01:44:53.221687+00	t	\\xfa35944166a6c60219a07a6792cd3abbaf909ba78e5494482d73d3d10ddacea618845822dc3dc875542772af742ac38d	-1
20260828120000	hosts_attribute_sources	2026-10-25 01:44:53.250295+00	t	\\x29220b8fb99e5ffb55ca71ea3c05f375214dd77d247d4a54dc7855c5654dbfbbbefceafeb5ddf10710c27085f0a866c4	-1
20260828120001	mac_address_sources	2026-10-25 01:44:53.253499+00	t	\\x39ea28fed133ae2c317bd5fad54e4d16f402ee895b75a6290322e75b434f5502aeddc9f6359d7e95625bb0c0d4ca1d4e	-1
20260828130000	hosts_software_revision	2026-10-25 01:44:53.256847+00	t	\\x83cdafd5691f1bc5fc2345a696ec9fd5c75700693f0011c65d352276d806a6f207f335189153ee8e0fa3fb7e60c18234	-1
20260828140000	ip_addresses_network_mac_index	2026-10-25 01:44:53.260974+00	t	\\x838b853bbb63bcfe0d0976f1c7289960e3133563151c47904ddda0ce5a0078bdc4af9ce07de63d3c024d89b831995535	-1
20260903120000	interfaces_if_descr_optional	2026-10-25 01:44:53.262991+00	t	\\x54495748b0271deebaf7cb06e9e4da9241563e83a162e4bfffaa9e571516223565ab239fcc8bb51fac9e37cbb7625757	-1
20260903120001	interface_neighbor_tables	2026-10-25 01:44:53.298035+00	t	\\xa8352a3ed9b845d96229adcb3e3eb8e796d22a8b6aa794bbc0d2ecafcbf76d68aa59a6e7713f134a57af0acceca3865a	-1
20260908120000	interfaces_ip_configured	2026-10-25 01:44:53.300666+00	t	\\x2467c0b912e78e9cac3818ffdec4157356f6b4a3c96e87a0171d88496439943eacbafefc7a4b51a2b68cc08bbbd29656	-1
20261005120001	rename_networks_to_sites_backfill	2026-10-25 01:44:53.373953+00	t	\\x085c1f2e6ec6b548e7f47fb092519f5f8889629e6198b30bd4a91e4e1d6b5e502513efb213c0601abf444d1f39183f81	-1
20261006120000	host_asset_tag	2026-10-25 01:44:53.376927+00	t	\\x1a9ef01df34f62321f6fdc26f4f75d952000cc366b54217a3107132643258e44957033c5f37b3618ff8d895d01eae918	-1
20261006120100	tag_group_and_icon	2026-10-25 01:44:53.378944+00	t	\\x3bad6e7e42b3a23b29a94462ddefba434ba2e7e92d1d9464db16644d40e45b97d01a83716203f5cda27d7bb5623dd436	-1
20261009120000	user_email_unsubscribe_token	2026-10-25 01:44:53.381231+00	t	\\x41aa0b879e77d264d1d6fc17c65bebe7e2d7796b9785ad79a85b928e23cc93c3cb8a0532ecbc8d72e34fc6e202d25ce1	-1
20261009120001	user_email_unsubscribe_token_index	2026-10-25 01:44:53.384879+00	t	\\x3cdfe6e7f6fe847ff4f18c7ee1d2430ee9833e48c6aa4eb3585c873529bb007b8786c26e6bad1c193ee8cf0dc0b23f0c	-1
20261009120100	tag_groups	2026-10-25 01:44:53.424979+00	t	\\xbdae7fe6cd73793312edbf2c987d3425f1e2c88c9fa98717a9e21550503734aaf67b3a1b804e145269962cd361aaa16a	-1
20261011120000	backfill_interface_credential_milestone	2026-10-25 01:44:53.427366+00	t	\\xfcad57a2a59537e0c9033a6ff906abf7115789554bc06c00c0caa388c220a6a5d3dc880c7aec0df9c6f561b3b94d7477	-1
20261011130000	release_notes	2026-10-25 01:44:53.432978+00	t	\\x46e5e77f7da9f2a01cb042045c01461eeb40b09477a5d2a40f302582b17e2b02e40f6e9dbe40ed1b24e667227ba3a2b3	-1
20261011130001	merge_and_seed_default_credentials	2026-10-25 01:44:53.457211+00	t	\\xc5fc006e4c216dd10154418fd2d52cf3e58b70039498b21a2fc74a1b3e8d9ce34af25da2b6bc197d0c7dff4f9ca910eb	-1
20261012120000	collection_sources	2026-10-25 01:44:53.462875+00	t	\\xfa0a6238fae38ae0c45b9657d540271cf8201a4981d6a16d84c856e8ba192e312b22472db6e2f734e1cd0a56eae723de	-1
20260914120000	hosts_hostname_source	2026-10-25 01:44:53.305336+00	t	\\xa57a5dc26c000ec5b0b4aaf66486018f637b1f1228ae994dafa06a653b5afae3d8dd2bb1264f5695c50053d656bccdc2	-1
20260915120000	organization_license_columns	2026-10-25 01:44:53.308107+00	t	\\xd46a3e083f8179128aae52f68541627b57131d66c93d84c2ad0002a7bf6c025f757345490b47d810511d49cd9528525d	-1
20260915120001	organizations_license_entitlement	2026-10-25 01:44:53.310079+00	t	\\x1da434d08d6afb9f0bbe4165f112cae53d3046c8578d4feb88d5f3ea0fa0649cd4bf2a69ac6c2c24978a561a6333fcb9	-1
20260916120000	organization_license_key_issued_at	2026-10-25 01:44:53.311871+00	t	\\x99d66ef126e64532e4e87c1a99b0ac630a2031b1b4c9ba41cbe257b8e3a83e47ec0fe1360a409b5cbb57401326d7a9d3	-1
20260917120000	organization_license_key_type	2026-10-25 01:44:53.313698+00	t	\\xac327476c465df672532ce71bfbe03a0d92ff5cc3053fae9a15fa24100308cda6b08e118267c3c1009118eeb90f46038	-1
20260917130000	organization_bills_by_invoice	2026-10-25 01:44:53.315985+00	t	\\x33bf32d3b3edce4fe6803c326785210f322b9cdfd065440098b53424df7042ba90da1ac6f99ade774f894da1b67b0979	-1
20260930120000	clear_zero_mac_addresses	2026-10-25 01:44:53.322315+00	t	\\xef8416f55b61953f00f1fd249a04c82d63d0d8d510e9db6cae96f79db7a70a61dfc5a22029a6878e74da05e60b63b7dd	-1
20261001120000	add_user_display_settings	2026-10-25 01:44:53.324609+00	t	\\xd7f9afb0762c40085ced713496cf39e9778c4257196e1ba02a0feec1af67eab309df78ce5b753ebde6d250f6fbf3ef32	-1
20261001120001	credential_daemon_os	2026-10-25 01:44:53.327531+00	t	\\xe679b2c2caea51e608200d064b81b02f1f4cee20cfa60a929db29b87666ba8a5d01e5d0739a2b9953f401f4171c11c9d	-1
20261002120000	daemon_os	2026-10-25 01:44:53.329364+00	t	\\x931ef66a80cd751db47512b5f9397a5d7df21aff77cbf28c5a190c69b27ce6040d2e2f2ed3c08e85faf13d28e5cba25f	-1
20261003120000	hosts_os	2026-10-25 01:44:53.331841+00	t	\\xb8a3b3a3e6ec9e21cc64468fa5b691dba7d39556aab7cd28fee01a21fcbde0782952b4b73cbdbd334a17c554769fbdce	-1
20261003130000	organization_license_server_version	2026-10-25 01:44:53.333763+00	t	\\xe2f1bf9692ed8bb1a08a1ae13b53f632fd5f2112d53225bbf13332af26639b9518edb28daa4b3ed98bf71ed4b1002d34	-1
20261004120000	host_virtualization_interface	2026-10-25 01:44:53.33739+00	t	\\x79224f29c669331336f577a5c9f7e5f674dec637f1a879e9c4016e8705af2e921e63cf4ab178cb21af1026317d6e57be	-1
20261004120001	host_virtualization_interface_validate	2026-10-25 01:44:53.340298+00	t	\\xf03d5b6826e7aca5a5efe1ab57878591bfd12ac5c097b1d71e5539348cf9a5e794c2ef19c5fa3ee8aac039920b9f1f4e	-1
20261004120002	host_virtualization_interface_index	2026-10-25 01:44:53.344045+00	t	\\x64e39691a98e9ed761812e1bfdeaa7b8d9aa13eb0401a39fb40dd36f21222ac47dad7f1b4ff9465a3cd0da1af42039ee	-1
20261005120000	rename_networks_to_sites	2026-10-25 01:44:53.363257+00	t	\\x5e08911ddc549e9d522eece781caa57fe42ee4cb6f7d16fb25b63bf6c7f95e897d2cbd12db345040f5a4035d420ace43	-1
20261012130000	discovery_warnings	2026-10-25 01:44:53.485994+00	t	\\xc760d410eaa7d4d14214bf602cd9ba7423c1a328b71768d79bcef0cee51b9b685c37438f74091fb9c3afd6483a98e46c	-1
20261013120000	license_instances	2026-10-25 01:44:53.494866+00	t	\\x86796dc3b14fc5805b84f6c6fd92fa8aaa560a004ea4ca8f4affa76f55b830f8d2403393cddfcf215070e84d499bb5cb	-1
20261013140000	terms_notice_version	2026-10-25 01:44:53.497033+00	t	\\x624bbb290511d7491b8816b152b06194417047e2b9b19eeb8b1d954ec17e6fee77c73d9fd19fb74d62f8c44d66bb1a93	-1
20261019120000	dependency_members_keep_on_delete	2026-10-25 01:44:53.501459+00	t	\\x2391912be8b3a07910ba2240ffe21065efe5836453ee2ba9ee7ca5fda5dea58d701795486c7e03a70909c88e5cb5d595	-1
20261021120000	vlan_and_subnet_sources	2026-10-25 01:44:53.504865+00	t	\\x2535fce0c0903904a1d9bfa86132278e9238231438d3b15be9863c47a362c284cc26d4b4ecb7830ee128a004d3d99878	-1
20261022120000	credential_needs_newer_daemon	2026-10-25 01:44:53.51545+00	t	\\xfcc14b3c03c4c930f7ffa6a0086ff9fe12d35c77ccc331b4469ebaf19ddcff05a1a21801b4f2c36fd7d14df5c403a1a1	-1
20261023120000	snmp_community_credential	2026-10-25 01:44:53.529405+00	t	\\x71e050db43cceee2a66e206cbd87f478cc8a6932440def65d5ea5c79f55ae7339bff4a9e7727b4bd7a9696dc535816cf	-1
20261024120000	topology_device_type_rule	2026-10-25 01:44:53.531821+00	t	\\xb5f60015b76437698dbd4381b285195bc4b866a3d20f1cfb588bd675b92aab51dedce82bcbc29749c5e04ed4e6050d87	-1
20261024120001	google_nest_capitalization	2026-10-25 01:44:53.534714+00	t	\\x204cc0286eea5cba45dded8756cd98207c511b505eef07ce0470fd35104d69092038adc091536ffb2dda65e288d73262	-1
\.


--
-- Data for Name: api_keys; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.api_keys (id, key, site_id, name, created_at, updated_at, last_used, expires_at, is_enabled, plaintext, daemon_id) FROM stdin;
f30b7413-52f9-4b1a-b605-30c018a2ac2e	0129999db5513d4207978057ec19a539b07036c925d4df24a742ebacca52ca1e	cd02a406-fbab-4ee0-a738-d530f5532b2f	Compat Test API Key	2026-10-25 01:52:24.62976+00	2026-10-25 01:52:24.62976+00	2026-10-25 01:52:55.41221+00	\N	t	\N	\N
6ea95919-6247-435a-a7bf-a683c78cd649	90efd35db33d26e228568ab5d09a161d44e0908cf2722010153719323c231669	cd02a406-fbab-4ee0-a738-d530f5532b2f	scanopy-daemon API Key	2026-10-25 01:44:56.59635+00	2026-10-25 01:44:56.59635+00	2026-10-25 01:53:12.015697+00	\N	t	\N	97bcf01f-d4f5-47de-b40b-4b652d4356c0
ef22c095-8a81-440b-b404-6518c77f7506	c9822a62315d7e5f29bac8c6d038fdf0ee7a3ef33a6a3ff67b82ecb45035d1e1	cd02a406-fbab-4ee0-a738-d530f5532b2f	scanopy-daemon-serverpoll API Key	2026-10-25 01:46:00.982192+00	2026-10-25 01:46:00.982192+00	2026-10-25 01:53:15.135825+00	\N	t	scp_d_Ll40i9zK1noiqu9AwV2tYVqOBQB61P1V	c9a92d0a-4254-44a9-990e-af221b639463
\.


--
-- Data for Name: bindings; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.bindings (id, site_id, service_id, binding_type, ip_address_id, port_id, created_at, updated_at, valid_from, valid_to, lineage_id, last_seen_at, last_discovery_id, first_discovery_id, snapshot_id) FROM stdin;
252b1078-0d64-4cbd-a889-00d02445629d	cd02a406-fbab-4ee0-a738-d530f5532b2f	0b62457e-a3d6-4712-b086-b55ab8eceb97	Port	618bc887-da03-48ca-8c41-542c0d06e3fa	fda4149b-1b8f-4143-a8fd-6326626f2c98	2026-03-06 07:04:08.184024+00	2026-03-06 07:04:08.184024+00	2026-10-25 01:53:00.34516+00	\N	\N	2026-10-25 01:53:00.34516+00	\N	\N	\N
\.


--
-- Data for Name: credentials; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.credentials (id, organization_id, name, credential_type, created_at, updated_at, daemon_os, description) FROM stdin;
018a3167-b654-4039-ad55-5767f7be50e9	35ff85ec-9fb5-46b6-a67d-944cbacc0277	SNMP (public)	{"type": "SnmpCommunity", "community": {"mode": "Inline", "value": "public"}}	2026-10-25 01:44:56.412238+00	2026-10-25 01:44:56.412238+00	\N	Assigned to every new site. Tries the SNMP community "public" on every host, over v2c and then v1.
603911ae-8719-415e-b19d-7128e1373f43	35ff85ec-9fb5-46b6-a67d-944cbacc0277	Docker socket	{"type": "DockerSocket"}	2026-10-25 01:44:56.416464+00	2026-10-25 01:44:56.416464+00	\N	Added to every new daemon. Reads the local Docker socket at its default path.
e385d42a-8600-4ea1-a75c-4c2b20e9a48d	35ff85ec-9fb5-46b6-a67d-944cbacc0277	Podman socket	{"type": "PodmanSocket"}	2026-10-25 01:44:56.420728+00	2026-10-25 01:44:56.420728+00	\N	Added to every new daemon. Reads the local Podman socket at its default path.
\.


--
-- Data for Name: daemon_interfaced_subnets; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.daemon_interfaced_subnets (daemon_id, subnet_id) FROM stdin;
c9a92d0a-4254-44a9-990e-af221b639463	9828043c-3f4e-4ae6-947a-8f340daa9642
c9a92d0a-4254-44a9-990e-af221b639463	a764172f-790f-4870-b9fe-aac03274c53c
97bcf01f-d4f5-47de-b40b-4b652d4356c0	a764172f-790f-4870-b9fe-aac03274c53c
97bcf01f-d4f5-47de-b40b-4b652d4356c0	9828043c-3f4e-4ae6-947a-8f340daa9642
\.


--
-- Data for Name: daemons; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.daemons (id, site_id, host_id, created_at, last_seen, updated_at, mode, url, name, version, user_id, api_key_id, is_unreachable, standby, standby_cleared_at, os) FROM stdin;
97bcf01f-d4f5-47de-b40b-4b652d4356c0	cd02a406-fbab-4ee0-a738-d530f5532b2f	2d0259ed-6b96-4930-847a-fa3ebdbf38a3	2026-10-25 01:44:56.619585+00	2026-10-25 01:53:12.027368+00	2026-10-25 01:44:56.619585+00	"daemon_poll"		scanopy-daemon	0.19.1	0f76ec49-3eef-434b-8b1e-9cf20c2c1a6b	6ea95919-6247-435a-a7bf-a683c78cd649	f	f	\N	"linux"
c9a92d0a-4254-44a9-990e-af221b639463	cd02a406-fbab-4ee0-a738-d530f5532b2f	e6565884-f116-4e52-a5c7-e3864d4c1e55	2026-10-25 01:46:00.992782+00	2026-10-25 01:53:15.142845+00	2026-10-25 01:46:00.992782+00	"server_poll"	http://daemon-serverpoll:60074	scanopy-daemon-serverpoll	0.19.1	0f76ec49-3eef-434b-8b1e-9cf20c2c1a6b	ef22c095-8a81-440b-b404-6518c77f7506	f	f	2026-10-25 01:53:15.142846+00	"linux"
\.


--
-- Data for Name: dependencies; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.dependencies (id, site_id, name, description, created_at, updated_at, source, color, edge_style, dependency_type, member_type, valid_from, valid_to, lineage_id, snapshot_id) FROM stdin;
b7410d30-4cd9-49cb-b764-385dd2bc3d27	cd02a406-fbab-4ee0-a738-d530f5532b2f		\N	2026-10-25 01:52:23.685276+00	2026-10-25 01:52:23.685276+00	{"type": "Manual"}	Yellow	"SmoothStep"	RequestPath	Services	2026-10-25 01:52:23.685276+00	\N	\N	\N
\.


--
-- Data for Name: dependency_members; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.dependency_members (id, dependency_id, binding_id, "position", created_at, service_id, valid_from, valid_to, lineage_id, snapshot_id) FROM stdin;
\.


--
-- Data for Name: discovery; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.discovery (id, site_id, daemon_id, run_type, discovery_type, name, created_at, updated_at, scan_count, force_full_scan, integration_targets) FROM stdin;
fb2efaaf-5c32-4d27-87c2-72506ba3fd74	cd02a406-fbab-4ee0-a738-d530f5532b2f	97bcf01f-d4f5-47de-b40b-4b652d4356c0	{"type": "Historical", "results": {"error": null, "phase": "Cancelled", "reason": "UserCancelled", "site_id": "cd02a406-fbab-4ee0-a738-d530f5532b2f", "progress": 0, "warnings": [], "daemon_id": "97bcf01f-d4f5-47de-b40b-4b652d4356c0", "session_id": "c10d2eae-1267-4023-997e-5a51fdf97281", "started_at": "2026-03-11T06:32:25.836913871Z", "finished_at": "2026-03-11T06:32:25.836988451Z", "discovery_id": null, "daemon_version": "0.14.18", "discovery_type": {"type": "Network", "subnet_ids": null, "snmp_credentials": {"daemon_os": null, "ip_overrides": [], "default_credential": null, "default_credential_id": null}, "host_naming_fallback": "BestService"}, "last_update_at": "2026-10-25T01:52:28.201936804Z", "hosts_discovered": null, "estimated_remaining_secs": null}}	{"type": "Network", "subnet_ids": null, "snmp_credentials": {"daemon_os": null, "ip_overrides": [], "default_credential": null, "default_credential_id": null}, "host_naming_fallback": "BestService"}	Network Discovery — My Site	2026-03-11 06:32:25.836913+00	2026-10-25 01:52:28.20541+00	0	f	[]
eb7da626-8e6d-41ce-a7d2-2503c470e727	cd02a406-fbab-4ee0-a738-d530f5532b2f	97bcf01f-d4f5-47de-b40b-4b652d4356c0	{"type": "Scheduled", "enabled": true, "last_run": "2026-10-25T01:45:04.753350861Z", "timezone": null, "cron_schedule": "0 0 0 * * 0"}	{"type": "Unified", "host_id": "2d0259ed-6b96-4930-847a-fa3ebdbf38a3", "subnet_ids": null, "scan_settings": {"arp_retries": null, "arp_rate_pps": null, "is_full_scan": false, "scan_rate_pps": null, "use_npcap_arp": false, "arp_scan_cutoff": null, "full_scan_interval": null, "port_scan_batch_size": null, "max_discovery_duration": null, "probe_raw_socket_ports": false, "trust_port_only_detections": false}, "host_naming_fallback": "BestService"}	Discovery	2026-10-25 01:44:56.625419+00	2026-10-25 01:46:00.535993+00	1	f	[]
c0ee8896-5849-43b2-9696-d6dfe106ad12	cd02a406-fbab-4ee0-a738-d530f5532b2f	97bcf01f-d4f5-47de-b40b-4b652d4356c0	{"type": "Historical", "results": {"error": null, "phase": "Complete", "reason": "Completed", "site_id": "cd02a406-fbab-4ee0-a738-d530f5532b2f", "progress": 100, "warnings": [], "daemon_id": "97bcf01f-d4f5-47de-b40b-4b652d4356c0", "session_id": "0073f594-456f-4861-8285-c8eb1345c48c", "started_at": "2026-10-25T01:45:04.782037127Z", "finished_at": "2026-10-25T01:46:00.478005198Z", "discovery_id": "eb7da626-8e6d-41ce-a7d2-2503c470e727", "daemon_version": "0.19.1", "discovery_type": {"type": "Unified", "host_id": "2d0259ed-6b96-4930-847a-fa3ebdbf38a3", "subnet_ids": null, "scan_settings": {"arp_retries": null, "arp_rate_pps": null, "is_full_scan": false, "scan_rate_pps": null, "use_npcap_arp": false, "arp_scan_cutoff": null, "full_scan_interval": null, "port_scan_batch_size": null, "max_discovery_duration": null, "probe_raw_socket_ports": false, "trust_port_only_detections": false}, "host_naming_fallback": "BestService"}, "last_update_at": "2026-10-25T01:46:00.525473592Z", "hosts_discovered": 6, "estimated_remaining_secs": 30}}	{"type": "Unified", "host_id": "2d0259ed-6b96-4930-847a-fa3ebdbf38a3", "subnet_ids": null, "scan_settings": {"arp_retries": null, "arp_rate_pps": null, "is_full_scan": false, "scan_rate_pps": null, "use_npcap_arp": false, "arp_scan_cutoff": null, "full_scan_interval": null, "port_scan_batch_size": null, "max_discovery_duration": null, "probe_raw_socket_ports": false, "trust_port_only_detections": false}, "host_naming_fallback": "BestService"}	Discovery	2026-10-25 01:45:04.782037+00	2026-10-25 01:46:00.535029+00	0	f	[]
d04eef0d-1e9a-46c1-927d-21525c111dba	cd02a406-fbab-4ee0-a738-d530f5532b2f	97bcf01f-d4f5-47de-b40b-4b652d4356c0	{"type": "Historical", "results": {"error": null, "phase": "Cancelled", "reason": "UserCancelled", "site_id": "cd02a406-fbab-4ee0-a738-d530f5532b2f", "progress": 0, "warnings": [], "daemon_id": "97bcf01f-d4f5-47de-b40b-4b652d4356c0", "session_id": "0ffe5496-d64e-41a0-ab86-ff28eed28819", "started_at": "2026-03-11T06:32:26.369555086Z", "finished_at": "2026-03-11T06:32:26.369642510Z", "discovery_id": null, "daemon_version": "0.14.18", "discovery_type": {"type": "Network", "subnet_ids": null, "snmp_credentials": {"daemon_os": null, "ip_overrides": [], "default_credential": null, "default_credential_id": null}, "host_naming_fallback": "BestService"}, "last_update_at": "2026-10-25T01:52:28.228096400Z", "hosts_discovered": null, "estimated_remaining_secs": null}}	{"type": "Network", "subnet_ids": null, "snmp_credentials": {"daemon_os": null, "ip_overrides": [], "default_credential": null, "default_credential_id": null}, "host_naming_fallback": "BestService"}	Network Discovery — My Site	2026-03-11 06:32:26.369555+00	2026-10-25 01:52:28.231449+00	0	f	[]
4ea1e918-a9b5-4a35-94aa-93a4c7a4c81c	cd02a406-fbab-4ee0-a738-d530f5532b2f	97bcf01f-d4f5-47de-b40b-4b652d4356c0	{"type": "Historical", "results": {"error": null, "phase": "Cancelled", "reason": "UserCancelled", "site_id": "cd02a406-fbab-4ee0-a738-d530f5532b2f", "progress": 0, "warnings": [], "daemon_id": "97bcf01f-d4f5-47de-b40b-4b652d4356c0", "session_id": "10fdd8f4-03b6-44ea-adb6-27e74136b365", "started_at": "2026-03-11T06:32:27.440078826Z", "finished_at": "2026-03-11T06:32:27.440151903Z", "discovery_id": null, "daemon_version": "0.14.18", "discovery_type": {"type": "Network", "subnet_ids": null, "snmp_credentials": {"daemon_os": null, "ip_overrides": [], "default_credential": null, "default_credential_id": null}, "host_naming_fallback": "BestService"}, "last_update_at": "2026-10-25T01:52:28.257707427Z", "hosts_discovered": null, "estimated_remaining_secs": null}}	{"type": "Network", "subnet_ids": null, "snmp_credentials": {"daemon_os": null, "ip_overrides": [], "default_credential": null, "default_credential_id": null}, "host_naming_fallback": "BestService"}	Network Discovery — My Site	2026-03-11 06:32:27.440078+00	2026-10-25 01:52:28.261134+00	0	f	[]
ec88b0a2-fc43-4c40-b3c4-a873ef5d30e0	cd02a406-fbab-4ee0-a738-d530f5532b2f	c9a92d0a-4254-44a9-990e-af221b639463	{"type": "Scheduled", "enabled": true, "last_run": "2026-10-25T01:46:23.597198602Z", "timezone": null, "cron_schedule": "0 0 0 * * 0"}	{"type": "Unified", "host_id": "e6565884-f116-4e52-a5c7-e3864d4c1e55", "subnet_ids": null, "scan_settings": {"arp_retries": null, "arp_rate_pps": null, "is_full_scan": false, "scan_rate_pps": null, "use_npcap_arp": false, "arp_scan_cutoff": null, "full_scan_interval": null, "port_scan_batch_size": null, "max_discovery_duration": null, "probe_raw_socket_ports": false, "trust_port_only_detections": false}, "host_naming_fallback": "BestService"}	Discovery	2026-10-25 01:46:00.996404+00	2026-10-25 01:49:23.624387+00	1	f	[]
1d9303d7-2559-4524-8219-7b901a3675ef	cd02a406-fbab-4ee0-a738-d530f5532b2f	c9a92d0a-4254-44a9-990e-af221b639463	{"type": "AdHoc", "last_run": "2026-10-25T01:46:25.573905542Z"}	{"type": "Unified", "host_id": "e6565884-f116-4e52-a5c7-e3864d4c1e55", "subnet_ids": null, "scan_settings": {"arp_retries": null, "arp_rate_pps": null, "is_full_scan": false, "scan_rate_pps": null, "use_npcap_arp": false, "arp_scan_cutoff": null, "full_scan_interval": null, "port_scan_batch_size": null, "max_discovery_duration": null, "probe_raw_socket_ports": false, "trust_port_only_detections": false}, "host_naming_fallback": "BestService"}	ServerPoll Integration Test Discovery	2026-10-25 01:46:25.562232+00	2026-10-25 01:52:23.623864+00	1	f	[]
19540abf-e76f-4ba3-b780-5d2b0f049059	cd02a406-fbab-4ee0-a738-d530f5532b2f	c9a92d0a-4254-44a9-990e-af221b639463	{"type": "Historical", "results": {"error": null, "phase": "Complete", "reason": "Completed", "site_id": "cd02a406-fbab-4ee0-a738-d530f5532b2f", "progress": 100, "warnings": [], "daemon_id": "c9a92d0a-4254-44a9-990e-af221b639463", "session_id": "e4869e95-eba9-4338-a8cf-87cfaf7052ca", "started_at": "2026-10-25T01:46:23.614260486Z", "finished_at": "2026-10-25T01:48:54.351446942Z", "discovery_id": "ec88b0a2-fc43-4c40-b3c4-a873ef5d30e0", "daemon_version": "0.19.1", "discovery_type": {"type": "Unified", "host_id": "e6565884-f116-4e52-a5c7-e3864d4c1e55", "subnet_ids": null, "scan_settings": {"arp_retries": null, "arp_rate_pps": null, "is_full_scan": false, "scan_rate_pps": null, "use_npcap_arp": false, "arp_scan_cutoff": null, "full_scan_interval": null, "port_scan_batch_size": null, "max_discovery_duration": null, "probe_raw_socket_ports": false, "trust_port_only_detections": false}, "host_naming_fallback": "BestService"}, "last_update_at": "2026-10-25T01:49:23.617670213Z", "hosts_discovered": null, "estimated_remaining_secs": null}}	{"type": "Unified", "host_id": "e6565884-f116-4e52-a5c7-e3864d4c1e55", "subnet_ids": null, "scan_settings": {"arp_retries": null, "arp_rate_pps": null, "is_full_scan": false, "scan_rate_pps": null, "use_npcap_arp": false, "arp_scan_cutoff": null, "full_scan_interval": null, "port_scan_batch_size": null, "max_discovery_duration": null, "probe_raw_socket_ports": false, "trust_port_only_detections": false}, "host_naming_fallback": "BestService"}	Discovery	2026-10-25 01:46:23.61426+00	2026-10-25 01:49:23.623664+00	0	f	[]
5e9776c0-4d70-4a8a-91c3-a273b868fe01	cd02a406-fbab-4ee0-a738-d530f5532b2f	97bcf01f-d4f5-47de-b40b-4b652d4356c0	{"type": "Historical", "results": {"error": null, "phase": "Cancelled", "reason": "UserCancelled", "site_id": "cd02a406-fbab-4ee0-a738-d530f5532b2f", "progress": 0, "warnings": [], "daemon_id": "97bcf01f-d4f5-47de-b40b-4b652d4356c0", "session_id": "4273fe18-d031-4977-950e-ad5a49d2ea8e", "started_at": "2026-03-11T06:32:27.708165946Z", "finished_at": "2026-03-11T06:32:27.708228433Z", "discovery_id": null, "daemon_version": "0.14.18", "discovery_type": {"type": "Network", "subnet_ids": null, "snmp_credentials": {"daemon_os": null, "ip_overrides": [], "default_credential": null, "default_credential_id": null}, "host_naming_fallback": "BestService"}, "last_update_at": "2026-10-25T01:52:28.269231695Z", "hosts_discovered": null, "estimated_remaining_secs": null}}	{"type": "Network", "subnet_ids": null, "snmp_credentials": {"daemon_os": null, "ip_overrides": [], "default_credential": null, "default_credential_id": null}, "host_naming_fallback": "BestService"}	Network Discovery — My Site	2026-03-11 06:32:27.708165+00	2026-10-25 01:52:28.272435+00	0	f	[]
607750b5-8751-459c-a98e-b65b73a61af9	cd02a406-fbab-4ee0-a738-d530f5532b2f	97bcf01f-d4f5-47de-b40b-4b652d4356c0	{"type": "Historical", "results": {"error": null, "phase": "Cancelled", "reason": "UserCancelled", "site_id": "cd02a406-fbab-4ee0-a738-d530f5532b2f", "progress": 0, "warnings": [], "daemon_id": "97bcf01f-d4f5-47de-b40b-4b652d4356c0", "session_id": "ec9dc330-67d0-4ce5-94d8-506859c74940", "started_at": "2026-03-11T06:32:29.590854940Z", "finished_at": "2026-03-11T06:32:29.590913560Z", "discovery_id": null, "daemon_version": "0.14.18", "discovery_type": {"type": "Network", "subnet_ids": null, "snmp_credentials": {"daemon_os": null, "ip_overrides": [], "default_credential": null, "default_credential_id": null}, "host_naming_fallback": "BestService"}, "last_update_at": "2026-10-25T01:52:28.349092045Z", "hosts_discovered": null, "estimated_remaining_secs": null}}	{"type": "Network", "subnet_ids": null, "snmp_credentials": {"daemon_os": null, "ip_overrides": [], "default_credential": null, "default_credential_id": null}, "host_naming_fallback": "BestService"}	Network Discovery — My Site	2026-03-11 06:32:29.590854+00	2026-10-25 01:52:28.352744+00	0	f	[]
10262d32-a510-40d5-a054-85c15bdbe0e6	cd02a406-fbab-4ee0-a738-d530f5532b2f	97bcf01f-d4f5-47de-b40b-4b652d4356c0	{"type": "Historical", "results": {"error": null, "phase": "Complete", "reason": "Completed", "site_id": "cd02a406-fbab-4ee0-a738-d530f5532b2f", "progress": 100, "warnings": [], "daemon_id": "97bcf01f-d4f5-47de-b40b-4b652d4356c0", "session_id": "6845dcc8-6ea0-47af-9ce3-054b8cf1f667", "started_at": "2026-01-23T03:40:38.920476295Z", "finished_at": "2026-01-23T03:40:38.980206587Z", "discovery_id": null, "daemon_version": "0.17.17", "discovery_type": {"type": "SelfReport", "host_id": "1438e666-92b6-4fad-bc37-aa2717d9ba42"}, "last_update_at": "2026-10-25T01:52:46.544984202Z", "hosts_discovered": null, "estimated_remaining_secs": null}}	{"type": "SelfReport", "host_id": "1438e666-92b6-4fad-bc37-aa2717d9ba42"}	Self Report — My Site	2026-01-23 03:40:38.920476+00	2026-10-25 01:52:46.54819+00	0	f	[]
9759b2b8-94ab-4f28-9fb6-e9c09e73327d	cd02a406-fbab-4ee0-a738-d530f5532b2f	97bcf01f-d4f5-47de-b40b-4b652d4356c0	{"type": "Historical", "results": {"error": null, "phase": "Complete", "reason": "Completed", "site_id": "cd02a406-fbab-4ee0-a738-d530f5532b2f", "progress": 100, "warnings": [], "daemon_id": "97bcf01f-d4f5-47de-b40b-4b652d4356c0", "session_id": "c8c6530c-aaff-4d7c-a872-ba4457906d77", "started_at": "2026-01-05T04:59:56.611843173Z", "finished_at": "2026-01-05T04:59:56.687605923Z", "discovery_id": null, "daemon_version": "0.17.2", "discovery_type": {"type": "SelfReport", "host_id": "cc741d90-bcc0-4653-b38b-52b23f9e6a61"}, "last_update_at": "2026-10-25T01:52:58.751816531Z", "hosts_discovered": null, "estimated_remaining_secs": null}}	{"type": "SelfReport", "host_id": "cc741d90-bcc0-4653-b38b-52b23f9e6a61"}	Self Report — My Site	2026-01-05 04:59:56.611843+00	2026-10-25 01:52:58.756089+00	0	f	[]
ffa44a05-f8b1-4da0-8db6-3b28927e5012	cd02a406-fbab-4ee0-a738-d530f5532b2f	c9a92d0a-4254-44a9-990e-af221b639463	{"type": "Historical", "results": {"error": null, "phase": "Complete", "reason": "Completed", "site_id": "cd02a406-fbab-4ee0-a738-d530f5532b2f", "progress": 100, "warnings": [], "daemon_id": "c9a92d0a-4254-44a9-990e-af221b639463", "session_id": "73dd31fe-9825-4a97-93fc-f5e12fd38aaf", "started_at": "2026-10-25T01:49:23.704382351Z", "finished_at": "2026-10-25T01:51:54.233315913Z", "discovery_id": "1d9303d7-2559-4524-8219-7b901a3675ef", "daemon_version": "0.19.1", "discovery_type": {"type": "Unified", "host_id": "e6565884-f116-4e52-a5c7-e3864d4c1e55", "subnet_ids": null, "scan_settings": {"arp_retries": null, "arp_rate_pps": null, "is_full_scan": false, "scan_rate_pps": null, "use_npcap_arp": false, "arp_scan_cutoff": null, "full_scan_interval": null, "port_scan_batch_size": null, "max_discovery_duration": null, "probe_raw_socket_ports": false, "trust_port_only_detections": false}, "host_naming_fallback": "BestService"}, "last_update_at": "2026-10-25T01:52:23.615975580Z", "hosts_discovered": null, "estimated_remaining_secs": null}}	{"type": "Unified", "host_id": "e6565884-f116-4e52-a5c7-e3864d4c1e55", "subnet_ids": null, "scan_settings": {"arp_retries": null, "arp_rate_pps": null, "is_full_scan": false, "scan_rate_pps": null, "use_npcap_arp": false, "arp_scan_cutoff": null, "full_scan_interval": null, "port_scan_batch_size": null, "max_discovery_duration": null, "probe_raw_socket_ports": false, "trust_port_only_detections": false}, "host_naming_fallback": "BestService"}	Discovery	2026-10-25 01:49:23.704382+00	2026-10-25 01:52:23.622783+00	0	f	[]
6cac6852-aa93-4839-82e6-4b3bb1e78691	cd02a406-fbab-4ee0-a738-d530f5532b2f	97bcf01f-d4f5-47de-b40b-4b652d4356c0	{"type": "Historical", "results": {"error": null, "phase": "Cancelled", "reason": "UserCancelled", "site_id": "cd02a406-fbab-4ee0-a738-d530f5532b2f", "progress": 0, "warnings": [], "daemon_id": "97bcf01f-d4f5-47de-b40b-4b652d4356c0", "session_id": "969d115c-0112-4ba3-8757-bb1ece340112", "started_at": "2026-03-11T06:32:27.172078644Z", "finished_at": "2026-03-11T06:32:27.172135060Z", "discovery_id": null, "daemon_version": "0.14.18", "discovery_type": {"type": "Network", "subnet_ids": null, "snmp_credentials": {"daemon_os": null, "ip_overrides": [], "default_credential": null, "default_credential_id": null}, "host_naming_fallback": "BestService"}, "last_update_at": "2026-10-25T01:52:28.246332430Z", "hosts_discovered": null, "estimated_remaining_secs": null}}	{"type": "Network", "subnet_ids": null, "snmp_credentials": {"daemon_os": null, "ip_overrides": [], "default_credential": null, "default_credential_id": null}, "host_naming_fallback": "BestService"}	Network Discovery — My Site	2026-03-11 06:32:27.172078+00	2026-10-25 01:52:28.249499+00	0	f	[]
1886a01c-4a51-4640-aac2-98f7d0f6f434	cd02a406-fbab-4ee0-a738-d530f5532b2f	97bcf01f-d4f5-47de-b40b-4b652d4356c0	{"type": "Historical", "results": {"error": null, "phase": "Cancelled", "reason": "UserCancelled", "site_id": "cd02a406-fbab-4ee0-a738-d530f5532b2f", "progress": 0, "warnings": [], "daemon_id": "97bcf01f-d4f5-47de-b40b-4b652d4356c0", "session_id": "22ea697f-065c-4d2e-a81f-0809764aad01", "started_at": "2026-03-11T06:32:28.783225595Z", "finished_at": "2026-03-11T06:32:28.783284155Z", "discovery_id": null, "daemon_version": "0.14.18", "discovery_type": {"type": "Network", "subnet_ids": null, "snmp_credentials": {"daemon_os": null, "ip_overrides": [], "default_credential": null, "default_credential_id": null}, "host_naming_fallback": "BestService"}, "last_update_at": "2026-10-25T01:52:28.314910167Z", "hosts_discovered": null, "estimated_remaining_secs": null}}	{"type": "Network", "subnet_ids": null, "snmp_credentials": {"daemon_os": null, "ip_overrides": [], "default_credential": null, "default_credential_id": null}, "host_naming_fallback": "BestService"}	Network Discovery — My Site	2026-03-11 06:32:28.783225+00	2026-10-25 01:52:28.318082+00	0	f	[]
f4bfa4e2-0a87-4e86-b5ef-0e201cf0722e	cd02a406-fbab-4ee0-a738-d530f5532b2f	97bcf01f-d4f5-47de-b40b-4b652d4356c0	{"type": "Historical", "results": {"error": null, "phase": "Cancelled", "reason": "UserCancelled", "site_id": "cd02a406-fbab-4ee0-a738-d530f5532b2f", "progress": 0, "warnings": [], "daemon_id": "97bcf01f-d4f5-47de-b40b-4b652d4356c0", "session_id": "bbe36108-56c4-4d82-9087-e577842ff202", "started_at": "2026-03-11T06:32:29.052254842Z", "finished_at": "2026-03-11T06:32:29.052318772Z", "discovery_id": null, "daemon_version": "0.14.18", "discovery_type": {"type": "Network", "subnet_ids": null, "snmp_credentials": {"daemon_os": null, "ip_overrides": [], "default_credential": null, "default_credential_id": null}, "host_naming_fallback": "BestService"}, "last_update_at": "2026-10-25T01:52:28.325896494Z", "hosts_discovered": null, "estimated_remaining_secs": null}}	{"type": "Network", "subnet_ids": null, "snmp_credentials": {"daemon_os": null, "ip_overrides": [], "default_credential": null, "default_credential_id": null}, "host_naming_fallback": "BestService"}	Network Discovery — My Site	2026-03-11 06:32:29.052254+00	2026-10-25 01:52:28.329288+00	0	f	[]
ed16604e-72c4-4921-99c0-cefafc206866	cd02a406-fbab-4ee0-a738-d530f5532b2f	97bcf01f-d4f5-47de-b40b-4b652d4356c0	{"type": "Historical", "results": {"error": null, "phase": "Cancelled", "reason": "UserCancelled", "site_id": "cd02a406-fbab-4ee0-a738-d530f5532b2f", "progress": 0, "warnings": [], "daemon_id": "97bcf01f-d4f5-47de-b40b-4b652d4356c0", "session_id": "3b0868af-d558-45e3-b688-2cf55472b6ee", "started_at": "2026-03-11T06:32:30.393154209Z", "finished_at": "2026-03-11T06:32:30.393220914Z", "discovery_id": null, "daemon_version": "0.14.18", "discovery_type": {"type": "Network", "subnet_ids": null, "snmp_credentials": {"daemon_os": null, "ip_overrides": [], "default_credential": null, "default_credential_id": null}, "host_naming_fallback": "BestService"}, "last_update_at": "2026-10-25T01:52:28.379600010Z", "hosts_discovered": null, "estimated_remaining_secs": null}}	{"type": "Network", "subnet_ids": null, "snmp_credentials": {"daemon_os": null, "ip_overrides": [], "default_credential": null, "default_credential_id": null}, "host_naming_fallback": "BestService"}	Network Discovery — My Site	2026-03-11 06:32:30.393154+00	2026-10-25 01:52:28.382942+00	0	f	[]
455b1e78-ed03-4697-a811-468309988cae	cd02a406-fbab-4ee0-a738-d530f5532b2f	97bcf01f-d4f5-47de-b40b-4b652d4356c0	{"type": "Historical", "results": {"error": null, "phase": "Complete", "reason": "Completed", "site_id": "cd02a406-fbab-4ee0-a738-d530f5532b2f", "progress": 100, "warnings": [], "daemon_id": "97bcf01f-d4f5-47de-b40b-4b652d4356c0", "session_id": "f2d5414b-3369-4280-929d-5422dd11a4b4", "started_at": "2026-01-23T03:55:10.297725045Z", "finished_at": "2026-01-23T03:55:10.328241754Z", "discovery_id": null, "daemon_version": "0.14.4", "discovery_type": {"type": "SelfReport", "host_id": "8f6b3991-b3ef-4d1d-9708-d2f57289a34f"}, "last_update_at": "2026-10-25T01:52:29.422305474Z", "hosts_discovered": null, "estimated_remaining_secs": null}}	{"type": "SelfReport", "host_id": "8f6b3991-b3ef-4d1d-9708-d2f57289a34f"}	Self Report — My Site	2026-01-23 03:55:10.297725+00	2026-10-25 01:52:29.425492+00	0	f	[]
f1eb9071-0cee-4815-969b-3703fa6320a2	cd02a406-fbab-4ee0-a738-d530f5532b2f	97bcf01f-d4f5-47de-b40b-4b652d4356c0	{"type": "Historical", "results": {"error": null, "phase": "Cancelled", "reason": "UserCancelled", "site_id": "cd02a406-fbab-4ee0-a738-d530f5532b2f", "progress": 0, "warnings": [], "daemon_id": "97bcf01f-d4f5-47de-b40b-4b652d4356c0", "session_id": "abba33fb-bf1f-4e8b-985f-6a2d0b5d0380", "started_at": "2026-03-11T06:32:27.975947389Z", "finished_at": "2026-03-11T06:32:27.976013693Z", "discovery_id": null, "daemon_version": "0.14.18", "discovery_type": {"type": "Network", "subnet_ids": null, "snmp_credentials": {"daemon_os": null, "ip_overrides": [], "default_credential": null, "default_credential_id": null}, "host_naming_fallback": "BestService"}, "last_update_at": "2026-10-25T01:52:28.280278756Z", "hosts_discovered": null, "estimated_remaining_secs": null}}	{"type": "Network", "subnet_ids": null, "snmp_credentials": {"daemon_os": null, "ip_overrides": [], "default_credential": null, "default_credential_id": null}, "host_naming_fallback": "BestService"}	Network Discovery — My Site	2026-03-11 06:32:27.975947+00	2026-10-25 01:52:28.283601+00	0	f	[]
7bb355fb-4463-44b9-a026-ad1f74b7cf08	cd02a406-fbab-4ee0-a738-d530f5532b2f	97bcf01f-d4f5-47de-b40b-4b652d4356c0	{"type": "Historical", "results": {"error": null, "phase": "Cancelled", "reason": "UserCancelled", "site_id": "cd02a406-fbab-4ee0-a738-d530f5532b2f", "progress": 0, "warnings": [], "daemon_id": "97bcf01f-d4f5-47de-b40b-4b652d4356c0", "session_id": "5e741620-3de9-476c-86e7-7ce656d0a5a8", "started_at": "2026-03-11T06:32:29.860793225Z", "finished_at": "2026-03-11T06:32:29.860865761Z", "discovery_id": null, "daemon_version": "0.14.18", "discovery_type": {"type": "Network", "subnet_ids": null, "snmp_credentials": {"daemon_os": null, "ip_overrides": [], "default_credential": null, "default_credential_id": null}, "host_naming_fallback": "BestService"}, "last_update_at": "2026-10-25T01:52:28.361200770Z", "hosts_discovered": null, "estimated_remaining_secs": null}}	{"type": "Network", "subnet_ids": null, "snmp_credentials": {"daemon_os": null, "ip_overrides": [], "default_credential": null, "default_credential_id": null}, "host_naming_fallback": "BestService"}	Network Discovery — My Site	2026-03-11 06:32:29.860793+00	2026-10-25 01:52:28.364395+00	0	f	[]
35ee9fc2-0e4b-4b42-9798-4911ac8f430c	cd02a406-fbab-4ee0-a738-d530f5532b2f	97bcf01f-d4f5-47de-b40b-4b652d4356c0	{"type": "Historical", "results": {"error": null, "phase": "Complete", "reason": "Completed", "site_id": "cd02a406-fbab-4ee0-a738-d530f5532b2f", "progress": 100, "warnings": [], "daemon_id": "97bcf01f-d4f5-47de-b40b-4b652d4356c0", "session_id": "b64df0ca-f173-4ba8-a48b-edf7e372974a", "started_at": "2026-01-07T19:14:20.155782339Z", "finished_at": "2026-01-07T19:14:20.224698505Z", "discovery_id": null, "daemon_version": "0.17.19", "discovery_type": {"type": "SelfReport", "host_id": "09900acc-93fd-4af9-8a9b-9f45ace7475c"}, "last_update_at": "2026-10-25T01:52:54.488220739Z", "hosts_discovered": null, "estimated_remaining_secs": null}}	{"type": "SelfReport", "host_id": "09900acc-93fd-4af9-8a9b-9f45ace7475c"}	Self Report — My Site	2026-01-07 19:14:20.155782+00	2026-10-25 01:52:54.491605+00	0	f	[]
c904216d-9fc1-422d-add5-5479d456bc0d	cd02a406-fbab-4ee0-a738-d530f5532b2f	97bcf01f-d4f5-47de-b40b-4b652d4356c0	{"type": "Historical", "results": {"error": null, "phase": "Cancelled", "reason": "UserCancelled", "site_id": "cd02a406-fbab-4ee0-a738-d530f5532b2f", "progress": 0, "warnings": [], "daemon_id": "97bcf01f-d4f5-47de-b40b-4b652d4356c0", "session_id": "27ff5779-08b0-4970-aebd-04649d27c725", "started_at": "2026-03-11T06:32:28.243891377Z", "finished_at": "2026-03-11T06:32:28.243957151Z", "discovery_id": null, "daemon_version": "0.14.18", "discovery_type": {"type": "Network", "subnet_ids": null, "snmp_credentials": {"daemon_os": null, "ip_overrides": [], "default_credential": null, "default_credential_id": null}, "host_naming_fallback": "BestService"}, "last_update_at": "2026-10-25T01:52:28.291840512Z", "hosts_discovered": null, "estimated_remaining_secs": null}}	{"type": "Network", "subnet_ids": null, "snmp_credentials": {"daemon_os": null, "ip_overrides": [], "default_credential": null, "default_credential_id": null}, "host_naming_fallback": "BestService"}	Network Discovery — My Site	2026-03-11 06:32:28.243891+00	2026-10-25 01:52:28.295229+00	0	f	[]
4ddb8ab5-d9a4-4ed9-b533-1745841031aa	cd02a406-fbab-4ee0-a738-d530f5532b2f	97bcf01f-d4f5-47de-b40b-4b652d4356c0	{"type": "Historical", "results": {"error": null, "phase": "Cancelled", "reason": "UserCancelled", "site_id": "cd02a406-fbab-4ee0-a738-d530f5532b2f", "progress": 0, "warnings": [], "daemon_id": "97bcf01f-d4f5-47de-b40b-4b652d4356c0", "session_id": "857fbace-0358-4642-8ca0-b14bb999f353", "started_at": "2026-03-11T06:32:28.512463426Z", "finished_at": "2026-03-11T06:32:28.512545810Z", "discovery_id": null, "daemon_version": "0.14.18", "discovery_type": {"type": "Network", "subnet_ids": null, "snmp_credentials": {"daemon_os": null, "ip_overrides": [], "default_credential": null, "default_credential_id": null}, "host_naming_fallback": "BestService"}, "last_update_at": "2026-10-25T01:52:28.303583104Z", "hosts_discovered": null, "estimated_remaining_secs": null}}	{"type": "Network", "subnet_ids": null, "snmp_credentials": {"daemon_os": null, "ip_overrides": [], "default_credential": null, "default_credential_id": null}, "host_naming_fallback": "BestService"}	Network Discovery — My Site	2026-03-11 06:32:28.512463+00	2026-10-25 01:52:28.306906+00	0	f	[]
af4ec1c5-d87e-4758-a47d-aab30cb46c86	cd02a406-fbab-4ee0-a738-d530f5532b2f	97bcf01f-d4f5-47de-b40b-4b652d4356c0	{"type": "Historical", "results": {"error": null, "phase": "Complete", "reason": "Completed", "site_id": "cd02a406-fbab-4ee0-a738-d530f5532b2f", "progress": 100, "warnings": [], "daemon_id": "97bcf01f-d4f5-47de-b40b-4b652d4356c0", "session_id": "7f54ecab-3771-4583-a7d0-f12569030e17", "started_at": "2026-01-23T03:48:00.469451847Z", "finished_at": "2026-01-23T03:48:00.553983097Z", "discovery_id": null, "daemon_version": "0.15.2", "discovery_type": {"type": "SelfReport", "host_id": "f738b076-a24e-4db2-800c-a0f10bb44b16"}, "last_update_at": "2026-10-25T01:52:33.877584821Z", "hosts_discovered": null, "estimated_remaining_secs": null}}	{"type": "SelfReport", "host_id": "f738b076-a24e-4db2-800c-a0f10bb44b16"}	Self Report — My Site	2026-01-23 03:48:00.469451+00	2026-10-25 01:52:33.880948+00	0	f	[]
170eb046-b78b-482b-ad0c-a3ec76797dc4	cd02a406-fbab-4ee0-a738-d530f5532b2f	97bcf01f-d4f5-47de-b40b-4b652d4356c0	{"type": "Historical", "results": {"error": null, "phase": "Cancelled", "reason": "UserCancelled", "site_id": "cd02a406-fbab-4ee0-a738-d530f5532b2f", "progress": 0, "warnings": [], "daemon_id": "97bcf01f-d4f5-47de-b40b-4b652d4356c0", "session_id": "5bd5e93e-a606-4c6e-b159-b04879bdd801", "started_at": "2026-03-15T19:48:14.853886058Z", "finished_at": "2026-03-15T19:48:14.853942173Z", "discovery_id": null, "daemon_version": "0.14.19", "discovery_type": {"type": "Network", "subnet_ids": null, "snmp_credentials": {"daemon_os": null, "ip_overrides": [], "default_credential": null, "default_credential_id": null}, "host_naming_fallback": "BestService"}, "last_update_at": "2026-10-25T01:52:45.086322890Z", "hosts_discovered": null, "estimated_remaining_secs": null}}	{"type": "Network", "subnet_ids": null, "snmp_credentials": {"daemon_os": null, "ip_overrides": [], "default_credential": null, "default_credential_id": null}, "host_naming_fallback": "BestService"}	Network Discovery — My Site	2026-03-15 19:48:14.853886+00	2026-10-25 01:52:45.089524+00	0	f	[]
fe601f3c-6b8d-4137-a6d4-e3c0a004a8b2	cd02a406-fbab-4ee0-a738-d530f5532b2f	97bcf01f-d4f5-47de-b40b-4b652d4356c0	{"type": "Historical", "results": {"error": null, "phase": "Cancelled", "reason": "UserCancelled", "site_id": "cd02a406-fbab-4ee0-a738-d530f5532b2f", "progress": 0, "warnings": [], "daemon_id": "97bcf01f-d4f5-47de-b40b-4b652d4356c0", "session_id": "e0ea6622-5f78-4cc3-b1f9-257946754dd8", "started_at": "2026-03-09T05:29:17.642983004Z", "finished_at": "2026-03-09T05:29:17.643045281Z", "discovery_id": null, "daemon_version": "0.14.17", "discovery_type": {"type": "Network", "subnet_ids": null, "snmp_credentials": {"daemon_os": null, "ip_overrides": [], "default_credential": null, "default_credential_id": null}, "host_naming_fallback": "BestService"}, "last_update_at": "2026-10-25T01:52:51.730283565Z", "hosts_discovered": null, "estimated_remaining_secs": null}}	{"type": "Network", "subnet_ids": null, "snmp_credentials": {"daemon_os": null, "ip_overrides": [], "default_credential": null, "default_credential_id": null}, "host_naming_fallback": "BestService"}	Network Discovery — My Site	2026-03-09 05:29:17.642983+00	2026-10-25 01:52:51.733808+00	0	f	[]
2be057e0-6071-4b82-a016-005ac8f433c6	cd02a406-fbab-4ee0-a738-d530f5532b2f	97bcf01f-d4f5-47de-b40b-4b652d4356c0	{"type": "Historical", "results": {"error": null, "phase": "Cancelled", "reason": "UserCancelled", "site_id": "cd02a406-fbab-4ee0-a738-d530f5532b2f", "progress": 0, "warnings": [], "daemon_id": "97bcf01f-d4f5-47de-b40b-4b652d4356c0", "session_id": "59fba01e-fefd-4c66-9ef2-a85c0e76a811", "started_at": "2026-03-11T06:32:29.322736685Z", "finished_at": "2026-03-11T06:32:29.322811406Z", "discovery_id": null, "daemon_version": "0.14.18", "discovery_type": {"type": "Network", "subnet_ids": null, "snmp_credentials": {"daemon_os": null, "ip_overrides": [], "default_credential": null, "default_credential_id": null}, "host_naming_fallback": "BestService"}, "last_update_at": "2026-10-25T01:52:28.337476729Z", "hosts_discovered": null, "estimated_remaining_secs": null}}	{"type": "Network", "subnet_ids": null, "snmp_credentials": {"daemon_os": null, "ip_overrides": [], "default_credential": null, "default_credential_id": null}, "host_naming_fallback": "BestService"}	Network Discovery — My Site	2026-03-11 06:32:29.322736+00	2026-10-25 01:52:28.340719+00	0	f	[]
\.


--
-- Data for Name: discovery_warnings; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.discovery_warnings (id, discovery_id, site_id, code, credential_id, host_id, remote_host_id, subnet_id, details, created_at) FROM stdin;
9e0674e2-ccc0-45c5-845a-45e65a0bb352	fb2efaaf-5c32-4d27-87c2-72506ba3fd74	cd02a406-fbab-4ee0-a738-d530f5532b2f	OutdatedDaemonFormat	\N	\N	\N	\N	{"code": "OutdatedDaemonFormat", "daemon_version": "0.14.18"}	2026-10-25 01:52:28.206754+00
4cdd4bce-dc2b-45b7-8aac-5c2de051a849	455b1e78-ed03-4697-a811-468309988cae	cd02a406-fbab-4ee0-a738-d530f5532b2f	OutdatedDaemonFormat	\N	\N	\N	\N	{"code": "OutdatedDaemonFormat", "daemon_version": "0.14.4"}	2026-10-25 01:52:29.426743+00
f7f07486-72e6-4357-a330-fec6a5e30d2d	af4ec1c5-d87e-4758-a47d-aab30cb46c86	cd02a406-fbab-4ee0-a738-d530f5532b2f	OutdatedDaemonFormat	\N	\N	\N	\N	{"code": "OutdatedDaemonFormat", "daemon_version": "0.15.2"}	2026-10-25 01:52:33.882234+00
af77e921-a4db-4f59-bb4f-9f2543627861	10262d32-a510-40d5-a054-85c15bdbe0e6	cd02a406-fbab-4ee0-a738-d530f5532b2f	OutdatedDaemonFormat	\N	\N	\N	\N	{"code": "OutdatedDaemonFormat", "daemon_version": "0.17.17"}	2026-10-25 01:52:46.549584+00
a20cf280-4460-4812-8dda-b71cebb6c05e	35ee9fc2-0e4b-4b42-9798-4911ac8f430c	cd02a406-fbab-4ee0-a738-d530f5532b2f	OutdatedDaemonFormat	\N	\N	\N	\N	{"code": "OutdatedDaemonFormat", "daemon_version": "0.17.19"}	2026-10-25 01:52:54.49284+00
8bcf9e7f-7f71-439b-bd0f-0a17facb686c	9759b2b8-94ab-4f28-9fb6-e9c09e73327d	cd02a406-fbab-4ee0-a738-d530f5532b2f	OutdatedDaemonFormat	\N	\N	\N	\N	{"code": "OutdatedDaemonFormat", "daemon_version": "0.17.2"}	2026-10-25 01:52:58.757367+00
\.


--
-- Data for Name: entity_tags; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.entity_tags (id, entity_id, entity_type, tag_id, created_at, valid_from, valid_to, lineage_id, snapshot_id) FROM stdin;
74b2054f-d3a3-4e18-9c06-779e00e8b0a4	d09241c6-df22-41e1-b6d7-62daab6fd51d	"Service"	8a5206f1-87ae-4c24-9b03-7b351b7ffc44	2026-10-25 01:52:23.67357+00	2026-10-25 01:52:23.67357+00	\N	\N	\N
\.


--
-- Data for Name: host_credentials; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.host_credentials (host_id, credential_id, ip_address_ids) FROM stdin;
\.


--
-- Data for Name: hosts; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.hosts (id, site_id, name, hostname, description, source, virtualization_metadata, created_at, updated_at, sys_descr, sys_object_id, sys_location, sys_contact, management_url, chassis_id, manufacturer, model, serial_number, sys_name, valid_from, valid_to, lineage_id, last_seen_at, last_discovery_id, first_discovery_id, snapshot_id, virtualization_service_id, firmware_revision, sys_descr_source, sys_object_id_source, sys_location_source, sys_contact_source, management_url_source, chassis_id_source, sys_name_source, manufacturer_source, model_source, serial_number_source, firmware_revision_source, name_source, software_revision, software_revision_source, hostname_source, os, os_source, virtualization_interface_id, asset_tag, asset_tag_source) FROM stdin;
07ca9676-4995-4fd4-b0a3-c60b0b356658	cd02a406-fbab-4ee0-a738-d530f5532b2f	a955a2a0d33b	a955a2a0d33b	Scanopy daemon	{"type": "Discovery"}	null	2026-10-25 01:53:00.348227+00	2026-10-25 01:53:00.348227+00	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	2026-10-25 01:53:00.348227+00	\N	\N	2026-10-25 01:53:00.348227+00	\N	\N	\N	\N	\N	"Unspecified"	"Unspecified"	"Unspecified"	"Unspecified"	"Unspecified"	"Unspecified"	"Unspecified"	"Unspecified"	"Unspecified"	"Unspecified"	"Unspecified"	"Unspecified"	\N	"Unspecified"	"Unspecified"	\N	"Unspecified"	\N	\N	"Unspecified"
\.


--
-- Data for Name: interface_neighbor_candidates; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.interface_neighbor_candidates (id, site_id, interface_id, lldp_chassis_id, lldp_port_id, lldp_sys_name, lldp_port_desc, lldp_mgmt_addr, lldp_sys_desc, cdp_device_id, cdp_port_id, cdp_platform, cdp_address, created_at, evidence_source) FROM stdin;
\.


--
-- Data for Name: interface_neighbor_hosts; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.interface_neighbor_hosts (id, site_id, interface_id, neighbor_host_id, neighbor_seen_at, created_at, updated_at, valid_from, valid_to, lineage_id, last_seen_at, last_discovery_id, first_discovery_id, snapshot_id) FROM stdin;
\.


--
-- Data for Name: interface_neighbor_interfaces; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.interface_neighbor_interfaces (id, site_id, interface_id, neighbor_interface_id, neighbor_seen_at, created_at, updated_at, valid_from, valid_to, lineage_id, last_seen_at, last_discovery_id, first_discovery_id, snapshot_id) FROM stdin;
\.


--
-- Data for Name: interfaces; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.interfaces (id, host_id, site_id, created_at, updated_at, if_index, if_descr, if_alias, if_type, speed_bps, admin_status, oper_status, mac_address, ip_address_id, if_name, fdb_macs, native_vlan_id, vlan_ids, valid_from, valid_to, lineage_id, last_seen_at, last_discovery_id, first_discovery_id, snapshot_id, mac_address_source, ip_configured, interface_source, fdb_macs_source, vlan_membership_source) FROM stdin;
\.


--
-- Data for Name: invites; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.invites (id, organization_id, permissions, site_ids, url, created_by, created_at, updated_at, expires_at, send_to) FROM stdin;
\.


--
-- Data for Name: ip_addresses; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.ip_addresses (id, site_id, host_id, subnet_id, ip_address, mac_address, name, "position", created_at, updated_at, valid_from, valid_to, lineage_id, last_seen_at, last_discovery_id, first_discovery_id, snapshot_id, mac_address_source, ip_address_source) FROM stdin;
618bc887-da03-48ca-8c41-542c0d06e3fa	cd02a406-fbab-4ee0-a738-d530f5532b2f	07ca9676-4995-4fd4-b0a3-c60b0b356658	a764172f-790f-4870-b9fe-aac03274c53c	172.25.0.4	b2:2b:55:c5:16:3d	eth0	0	2026-10-25 01:53:00.348227+00	2026-10-25 01:53:00.348227+00	2026-10-25 01:53:00.348227+00	\N	\N	2026-10-25 01:53:00.348227+00	\N	\N	\N	"Unspecified"	"Unspecified"
\.


--
-- Data for Name: license_instances; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.license_instances (id, organization_id, instance_id, server_version, last_seen_ip, non_production, first_seen_at, last_seen_at, active_since, created_at, updated_at) FROM stdin;
\.


--
-- Data for Name: organizations; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.organizations (id, name, stripe_customer_id, plan, plan_status, created_at, updated_at, onboarding, has_payment_method, trial_end_date, use_case, last_paused_at, trial_extended_used, last_downgrade_at, last_downgrade_from_plan, last_discount_at, discount_save_offer_percent_off, discount_save_offer_active_until, next_renewal_at, notifications, license_paid_through, license_checkin_at, license_key_version, license_entitlement, license_entitlement_at, license_key_issued_at, license_key_type, bills_by_invoice, license_server_version) FROM stdin;
35ff85ec-9fb5-46b6-a67d-944cbacc0277	My Organization	\N	{"rate": "Month", "type": "Community", "base_cents": 0, "host_cents": null, "seat_cents": null, "site_cents": null, "trial_days": 0, "included_orgs": null, "included_hosts": null, "included_seats": null, "included_sites": null}	active	2026-10-25 01:44:56.400883+00	2026-10-25 01:44:56.400883+00	["OrgCreated", "OnboardingModalCompleted", "FirstDaemonRegistered", "FirstHostDiscovered", "FirstDiscoveryCompleted", "FirstTagCreated", "FirstDependencyCreated", "FirstUserApiKeyCreated", "SecondSiteCreated"]	f	\N	other	\N	f	\N	null	\N	\N	\N	\N	{"hosts": "None", "seats": "None", "sites": "None", "license_over_instance_limit": false, "license_silence_reported_for": null, "airgap_expiry_notified_through": null, "sunset_notified_last_unsupported": null}	\N	\N	0	\N	\N	\N	\N	f	\N
\.


--
-- Data for Name: ports; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.ports (id, site_id, host_id, port_number, protocol, port_type, created_at, updated_at, valid_from, valid_to, lineage_id, last_seen_at, last_discovery_id, first_discovery_id, snapshot_id, port_source) FROM stdin;
fda4149b-1b8f-4143-a8fd-6326626f2c98	cd02a406-fbab-4ee0-a738-d530f5532b2f	07ca9676-4995-4fd4-b0a3-c60b0b356658	60073	Tcp	Custom	2026-10-25 01:53:00.348227+00	2026-10-25 01:53:00.348227+00	2026-10-25 01:53:00.348227+00	\N	\N	2026-10-25 01:53:00.348227+00	\N	\N	\N	"Unspecified"
\.


--
-- Data for Name: release_announcements; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.release_announcements (id, version, email_sent_at, created_at, updated_at) FROM stdin;
ae983250-bf6c-4ce0-8d13-f58e6586bd2b	0.19.1	\N	2026-10-25 01:44:53.693118+00	2026-10-25 01:44:53.693118+00
\.


--
-- Data for Name: server_instance; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.server_instance (id, created_at, updated_at) FROM stdin;
\.


--
-- Data for Name: services; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.services (id, site_id, created_at, updated_at, name, host_id, service_definition, virtualization_metadata, source, "position", valid_from, valid_to, lineage_id, last_seen_at, last_discovery_id, first_discovery_id, snapshot_id, virtualization_service_id) FROM stdin;
0b62457e-a3d6-4712-b086-b55ab8eceb97	cd02a406-fbab-4ee0-a738-d530f5532b2f	2026-10-25 01:53:00.348227+00	2026-10-25 01:53:00.348227+00	Scanopy Daemon	07ca9676-4995-4fd4-b0a3-c60b0b356658	"Scanopy Daemon"	null	{"type": "DiscoveryWithMatch", "details": {"reason": {"data": "Scanopy Daemon self-report", "type": "reason"}, "confidence": "Certain"}}	0	2026-10-25 01:53:00.348227+00	\N	\N	2026-10-25 01:53:00.348227+00	\N	\N	\N	\N
\.


--
-- Data for Name: shares; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.shares (id, topology_id, site_id, created_by, name, is_enabled, expires_at, password_hash, allowed_domains, options, created_at, updated_at, enabled_views) FROM stdin;
\.


--
-- Data for Name: site_credentials; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.site_credentials (site_id, credential_id) FROM stdin;
cd02a406-fbab-4ee0-a738-d530f5532b2f	018a3167-b654-4039-ad55-5767f7be50e9
\.


--
-- Data for Name: sites; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.sites (id, name, created_at, updated_at, organization_id, stale_after_hours) FROM stdin;
cd02a406-fbab-4ee0-a738-d530f5532b2f	My Site	2026-10-25 01:44:56.446807+00	2026-10-25 01:44:56.446807+00	35ff85ec-9fb5-46b6-a67d-944cbacc0277	\N
\.


--
-- Data for Name: snapshots; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.snapshots (id, site_id, taken_at, created_by_user_id, created_at, updated_at) FROM stdin;
\.


--
-- Data for Name: subnet_vlans; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.subnet_vlans (id, subnet_id, vlan_id, created_at, valid_from, valid_to, lineage_id, snapshot_id) FROM stdin;
\.


--
-- Data for Name: subnets; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.subnets (id, site_id, created_at, updated_at, cidr, name, description, subnet_type, source, valid_from, valid_to, lineage_id, last_seen_at, last_discovery_id, first_discovery_id, snapshot_id, virtualization_service_id, cidr_source, name_source, subnet_type_source) FROM stdin;
a764172f-790f-4870-b9fe-aac03274c53c	cd02a406-fbab-4ee0-a738-d530f5532b2f	1970-01-01 00:00:00+00	2026-03-06 07:04:08.161423+00	"172.25.0.0/28"	172.25.0.0/28	\N	Lan	{"type": "Discovery"}	1970-01-01 00:00:00+00	\N	\N	2026-10-25 01:53:12.010424+00	\N	\N	\N	\N	"DaemonSelfReport"	"Unspecified"	"InterfaceNameMatch"
9828043c-3f4e-4ae6-947a-8f340daa9642	cd02a406-fbab-4ee0-a738-d530f5532b2f	2026-10-25 01:53:00.450749+00	2026-10-25 01:53:00.450749+00	"127.0.0.0/8"	127.0.0.0/8	\N	Loopback	{"type": "Discovery"}	2026-10-25 01:53:00.450749+00	\N	\N	2026-10-25 01:53:12.01039+00	\N	\N	\N	\N	"DaemonSelfReport"	"Unspecified"	"InterfaceNameMatch"
7b347c03-d985-4164-a561-34d4edd230f3	cd02a406-fbab-4ee0-a738-d530f5532b2f	2026-10-25 01:53:14.142062+00	2026-10-25 01:53:14.142062+00	"10.1.0.0/24"	Blocked Subnet	\N	Lan	{"type": "Manual"}	2026-10-25 01:53:14.142062+00	\N	\N	2026-10-25 01:53:14.142062+00	\N	\N	\N	\N	"Manual"	"Manual"	"Manual"
\.


--
-- Data for Name: tag_groups; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.tag_groups (id, organization_id, name, description, group_type, entity_types, ordered, created_at, updated_at) FROM stdin;
69c58d06-b61a-4fd0-88ef-6ca583c37ee8	35ff85ec-9fb5-46b6-a67d-944cbacc0277	Application	What each entity is part of. A service inherits its host's application tag, and the Application view groups by it.	Application	\N	f	2026-10-25 01:44:56.428238+00	2026-10-25 01:44:56.428238+00
ad60184f-31ec-4ce4-881b-abed694cd069	35ff85ec-9fb5-46b6-a67d-944cbacc0277	Status	Where an asset is in its lifecycle.	Status	\N	t	2026-10-25 01:44:56.430524+00	2026-10-25 01:44:56.430524+00
3be8530d-4fe5-4df7-adbd-cd9595a129de	35ff85ec-9fb5-46b6-a67d-944cbacc0277	Visibility	Entities holding a tag of this group are left out of every topology.	Visibility	\N	f	2026-10-25 01:44:56.432758+00	2026-10-25 01:44:56.432758+00
2db50094-d16f-4b03-b1c6-36c4428f2d01	35ff85ec-9fb5-46b6-a67d-944cbacc0277	Environment	Which stage of delivery an asset serves.	Custom	\N	t	2026-10-25 01:44:56.53348+00	2026-10-25 01:44:56.53348+00
25292ddb-e123-434e-8f01-59a0e5d00226	35ff85ec-9fb5-46b6-a67d-944cbacc0277	Criticality	How much an outage of the asset costs.	Custom	\N	t	2026-10-25 01:44:56.534776+00	2026-10-25 01:44:56.534776+00
9c2aee97-0f7c-4e81-ac1f-a9ed099600f9	35ff85ec-9fb5-46b6-a67d-944cbacc0277	Management	Who administers the asset.	Custom	\N	t	2026-10-25 01:44:56.536775+00	2026-10-25 01:44:56.536775+00
\.


--
-- Data for Name: tags; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.tags (id, organization_id, name, description, created_at, updated_at, color, valid_from, valid_to, lineage_id, snapshot_id, icon, tag_group_id, "position") FROM stdin;
3191962a-69f5-4f56-80ad-bec77f19eff1	35ff85ec-9fb5-46b6-a67d-944cbacc0277	Hidden	Left out of topology.	2026-10-25 01:44:56.433955+00	2026-10-25 01:44:56.433955+00	Gray	2026-10-25 01:44:56.433955+00	\N	\N	\N	eye-off	3be8530d-4fe5-4df7-adbd-cd9595a129de	\N
d0adef75-b01a-40ba-bd5a-1494a044b627	35ff85ec-9fb5-46b6-a67d-944cbacc0277	Planned	Ordered or scheduled, not yet in service	2026-10-25 01:44:56.542466+00	2026-10-25 01:44:56.542466+00	Blue	2026-10-25 01:44:56.542466+00	\N	\N	\N	calendar-clock	ad60184f-31ec-4ce4-881b-abed694cd069	1
f9bd39a9-d2e5-4e3d-bd95-231fe13898f9	35ff85ec-9fb5-46b6-a67d-944cbacc0277	Active	In service	2026-10-25 01:44:56.542474+00	2026-10-25 01:44:56.542474+00	Green	2026-10-25 01:44:56.542474+00	\N	\N	\N	circle-check	ad60184f-31ec-4ce4-881b-abed694cd069	2
15e19bdc-3347-4611-a22b-2ed91dca4578	35ff85ec-9fb5-46b6-a67d-944cbacc0277	Decommissioning	Being retired; still on the network	2026-10-25 01:44:56.542476+00	2026-10-25 01:44:56.542476+00	Orange	2026-10-25 01:44:56.542476+00	\N	\N	\N	hourglass	ad60184f-31ec-4ce4-881b-abed694cd069	3
07b33d1b-9452-423d-a9bf-8b9b3d646fb1	35ff85ec-9fb5-46b6-a67d-944cbacc0277	Decommissioned	Retired from service; kept for the record	2026-10-25 01:44:56.542477+00	2026-10-25 01:44:56.542477+00	Gray	2026-10-25 01:44:56.542477+00	\N	\N	\N	archive	ad60184f-31ec-4ce4-881b-abed694cd069	4
4c04e091-b6af-45ba-b0ac-779afb04e000	35ff85ec-9fb5-46b6-a67d-944cbacc0277	Production	Serves live users and business traffic	2026-10-25 01:44:56.542483+00	2026-10-25 01:44:56.542483+00	Red	2026-10-25 01:44:56.542483+00	\N	\N	\N	server	2db50094-d16f-4b03-b1c6-36c4428f2d01	1
1ce6f3cb-9907-4b06-b003-f02bb411024a	35ff85ec-9fb5-46b6-a67d-944cbacc0277	Staging	A pre-release copy of production for final testing	2026-10-25 01:44:56.542487+00	2026-10-25 01:44:56.542487+00	Amber	2026-10-25 01:44:56.542487+00	\N	\N	\N	flask-conical	2db50094-d16f-4b03-b1c6-36c4428f2d01	2
79a30d35-2a55-4498-b237-0a606a4e0db4	35ff85ec-9fb5-46b6-a67d-944cbacc0277	Development	Used to build and test changes	2026-10-25 01:44:56.542489+00	2026-10-25 01:44:56.542489+00	Blue	2026-10-25 01:44:56.542489+00	\N	\N	\N	code	2db50094-d16f-4b03-b1c6-36c4428f2d01	3
085376d3-ff0f-4fa6-924e-044ceb8986d6	35ff85ec-9fb5-46b6-a67d-944cbacc0277	Lab	Test bench or experimental equipment	2026-10-25 01:44:56.54249+00	2026-10-25 01:44:56.54249+00	Purple	2026-10-25 01:44:56.54249+00	\N	\N	\N	test-tube	2db50094-d16f-4b03-b1c6-36c4428f2d01	4
9da0249c-fad5-4892-82f6-a57aad2de4f3	35ff85ec-9fb5-46b6-a67d-944cbacc0277	Critical	An outage stops the business	2026-10-25 01:44:56.542494+00	2026-10-25 01:44:56.542494+00	Red	2026-10-25 01:44:56.542494+00	\N	\N	\N	siren	25292ddb-e123-434e-8f01-59a0e5d00226	1
39dd51e0-41ca-4e7f-9af2-d926ebe28dab	35ff85ec-9fb5-46b6-a67d-944cbacc0277	High	An outage disrupts many people	2026-10-25 01:44:56.542496+00	2026-10-25 01:44:56.542496+00	Orange	2026-10-25 01:44:56.542496+00	\N	\N	\N	arrow-up	25292ddb-e123-434e-8f01-59a0e5d00226	2
dfd1d0a0-c01e-4e5f-b3a5-84278f67f85c	35ff85ec-9fb5-46b6-a67d-944cbacc0277	Medium	An outage disrupts some people	2026-10-25 01:44:56.542497+00	2026-10-25 01:44:56.542497+00	Yellow	2026-10-25 01:44:56.542497+00	\N	\N	\N	minus	25292ddb-e123-434e-8f01-59a0e5d00226	3
4ca16da7-d8d6-4d03-9be3-6955e4542a78	35ff85ec-9fb5-46b6-a67d-944cbacc0277	Low	An outage has little impact	2026-10-25 01:44:56.542498+00	2026-10-25 01:44:56.542498+00	Gray	2026-10-25 01:44:56.542498+00	\N	\N	\N	arrow-down	25292ddb-e123-434e-8f01-59a0e5d00226	4
a0e5376b-1b38-4a38-a87f-f2e7c1762d8d	35ff85ec-9fb5-46b6-a67d-944cbacc0277	Managed	Administered by your IT team	2026-10-25 01:44:56.542504+00	2026-10-25 01:44:56.542504+00	Green	2026-10-25 01:44:56.542504+00	\N	\N	\N	shield-check	9c2aee97-0f7c-4e81-ac1f-a9ed099600f9	1
5eb6199c-1f04-4b7c-a2a8-7258e51b68ae	35ff85ec-9fb5-46b6-a67d-944cbacc0277	Unmanaged	Administered by no one, such as a personal device	2026-10-25 01:44:56.542505+00	2026-10-25 01:44:56.542505+00	Orange	2026-10-25 01:44:56.542505+00	\N	\N	\N	shield-question-mark	9c2aee97-0f7c-4e81-ac1f-a9ed099600f9	2
28ae2964-bbf9-44e6-83ad-5830736c4966	35ff85ec-9fb5-46b6-a67d-944cbacc0277	Third-party	Administered by a vendor or another organization	2026-10-25 01:44:56.542506+00	2026-10-25 01:44:56.542506+00	Blue	2026-10-25 01:44:56.542506+00	\N	\N	\N	handshake	9c2aee97-0f7c-4e81-ac1f-a9ed099600f9	3
8a5206f1-87ae-4c24-9b03-7b351b7ffc44	35ff85ec-9fb5-46b6-a67d-944cbacc0277	Integration Test Tag	\N	2026-10-25 01:52:23.639897+00	2026-10-25 01:52:23.639897+00	Yellow	2026-10-25 01:52:23.639897+00	\N	\N	\N	\N	\N	\N
7cdd6c01-2986-410d-9635-43a5079d9c97	35ff85ec-9fb5-46b6-a67d-944cbacc0277	Test Tag	\N	2026-10-25 01:53:12.883541+00	2026-10-25 01:53:12.883541+00	Yellow	2026-10-25 01:53:12.883541+00	2026-10-25 01:53:12.914137+00	5cf5b8ca-6dd4-42e2-b98b-080ca3b738c9	\N	\N	\N	\N
5cf5b8ca-6dd4-42e2-b98b-080ca3b738c9	35ff85ec-9fb5-46b6-a67d-944cbacc0277	Updated Tag	\N	2026-10-25 01:53:12.883541+00	2026-10-25 01:53:12.883541+00	Yellow	2026-10-25 01:53:12.914137+00	2026-10-25 01:53:12.936923+00	\N	\N	\N	\N	\N
\.


--
-- Data for Name: topologies; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.topologies (id, site_id, options, created_at, updated_at) FROM stdin;
72a854aa-50fb-4ad1-bbda-73ef79136b7f	cd02a406-fbab-4ee0-a738-d530f5532b2f	{"local": {"tag_filter": {}, "bundle_edges": true, "show_minimap": true, "no_fade_edges": false, "hide_edge_types": {"L3Logical": ["Hypervisor", "SameContainer"], "Workloads": ["RequestPath", "HubAndSpoke", "PhysicalLink", "NeighborLink"], "L2Physical": ["SameHost"], "Application": ["ContainerRuntime"]}}, "request": {"element_sort": {}, "element_rules": [{"id": "843cb12a-b8a2-4831-8f6e-1e8c17a2b071", "rule": "ByTrunkPort"}, {"id": "4c400331-a837-4747-8e3c-4e6a1f847d44", "rule": "ByVLAN"}, {"id": "caf3213d-9314-4757-a535-85dad8d16fb6", "rule": "ByPortOpStatus"}, {"id": "73483d9e-f096-47d3-ae61-f4a8803d0acd", "rule": {"ByServiceCategory": {"title": "Infrastructure", "categories": ["NetworkCore", "NetworkAccess", "NetworkAppliance", "RemoteAccess", "Workstation", "Mobile", "IoT", "Printer", "OpenPorts", "NetworkIdentities"], "is_infra_rule": true}}}, {"id": "dca2ac2c-d7c7-4265-ac10-8f1d5bd0e2c1", "rule": {"ByTag": {"title": null, "tag_ids": []}}}, {"id": "d1f7635e-83de-4146-9715-015d63eb4b06", "rule": "ByDeviceType"}, {"id": "4d089cbb-5235-465a-89d8-824c1955ba2d", "rule": "ByHypervisor"}, {"id": "6bc26e34-431c-4ed5-8ca9-d56739c28183", "rule": "ByContainerRuntime"}, {"id": "c9fca011-9720-46b5-9300-0bc4f4f71d70", "rule": "ByStack"}], "hide_entities": {}, "container_rules": {"L3Logical": [{"id": "a3e146f5-81b9-4449-99ec-e5d189473e3a", "rule": "BySubnet"}, {"id": "2c452e53-83a4-4691-9ca7-5c24ff98f8e5", "rule": "MergeContainerBridges"}], "Workloads": [{"id": "dc0b165d-c630-4bae-aa3c-6a44026c6188", "rule": "ByHost"}], "L2Physical": [{"id": "dc0b165d-c630-4bae-aa3c-6a44026c6188", "rule": "ByHost"}], "Application": [{"id": "8071e60e-42ad-4e70-9378-05f4d4796192", "rule": {"ByApplication": {"tag_ids": []}}}]}, "hide_metadata_values": {"L3Logical": {"Service": {"Category": ["OpenPorts"]}}, "Workloads": {"Service": {"Category": ["OpenPorts"]}}, "L2Physical": {"Service": {"Category": ["OpenPorts"]}, "Interface": {"LinkState": ["Unlinked"]}}, "Application": {"Service": {"Category": ["OpenPorts"]}}}}}	2026-10-25 01:44:56.473398+00	2026-10-25 01:44:56.473398+00
\.


--
-- Data for Name: user_api_key_site_access; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.user_api_key_site_access (id, api_key_id, site_id, created_at) FROM stdin;
\.


--
-- Data for Name: user_api_keys; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.user_api_keys (id, key, user_id, organization_id, permissions, name, created_at, updated_at, last_used, expires_at, is_enabled) FROM stdin;
\.


--
-- Data for Name: user_site_access; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.user_site_access (id, user_id, site_id, created_at) FROM stdin;
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.users (id, created_at, updated_at, password_hash, oidc_provider, oidc_subject, oidc_linked_at, email, organization_id, permissions, tags, terms_accepted_at, email_verified, email_verification_token, email_verification_expires, password_reset_token, password_reset_expires, pending_email, email_settings, session_epoch, display_settings, email_unsubscribe_token, release_notes_seen_version, terms_notice_version) FROM stdin;
0f76ec49-3eef-434b-8b1e-9cf20c2c1a6b	2026-10-25 01:44:56.403184+00	2026-10-25 01:44:56.403184+00	$argon2id$v=19$m=19456,t=2,p=1$QsXoB3+i1tLtq1/cyOqXDw$XJTCZgg4zKsWojQsnBlXBWBAwl+FxrimzXLdAYc+drA	\N	\N	\N	user@gmail.com	35ff85ec-9fb5-46b6-a67d-944cbacc0277	Owner	{}	\N	t	\N	\N	\N	\N	\N	{"daemon_alerts": true, "product_updates": true, "trial_and_usage": true, "discovery_digest": true, "product_onboarding": true}	0	{"clock": "browser_default", "time_zone": null, "date_order": "browser_default", "timestamps": "relative", "week_start": "monday", "table_density": "compact"}	\N	0.19.1	\N
d779b4ab-ea60-4d83-ab01-47c4d7a966fd	2026-10-25 01:53:13.765246+00	2026-10-25 01:53:13.765246+00	\N	\N	\N	\N	user@example.com	35ff85ec-9fb5-46b6-a67d-944cbacc0277	Owner	{}	\N	f	\N	\N	\N	\N	\N	{"daemon_alerts": true, "product_updates": true, "trial_and_usage": true, "discovery_digest": true, "product_onboarding": true}	0	{"clock": "browser_default", "time_zone": null, "date_order": "browser_default", "timestamps": "relative", "week_start": "monday", "table_density": "compact"}	\N	\N	\N
\.


--
-- Data for Name: vlans; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.vlans (id, vlan_number, name, description, site_id, organization_id, source, created_at, updated_at, valid_from, valid_to, lineage_id, last_seen_at, last_discovery_id, first_discovery_id, snapshot_id, name_source) FROM stdin;
\.


--
-- Data for Name: session; Type: TABLE DATA; Schema: tower_sessions; Owner: postgres
--

COPY tower_sessions.session (id, data, expiry_date) FROM stdin;
EkeDZK3j3F1srWOhyxRJpg	\\x93c410a64914cba163ad6c5ddce3ad6483471282a7757365725f6964d92430663736656334392d336565662d343334622d386231652d396366323063326331613662ad73657373696f6e5f65706f63680099cd07eacd0131012c38ce1875625b000000	2026-11-01 01:44:56.410346+00
1sTykAZ2TMmztOE49V7nFA	\\x93c41014e75ef538e1b4b3c94c760690f2c4d683ad70656e64696e675f736574757083a86f72675f6e616d65af4d79204f7267616e697a6174696f6ea47369746582a46e616d65a74d792053697465a7736974655f6964d92434303965616535392d383366302d343835362d396236392d353639316430623565636131a87573655f63617365a56f74686572a7757365725f6964d92430663736656334392d336565662d343334622d386231652d396366323063326331613662ad73657373696f6e5f65706f63680099cd07eacd0131013418ce25561872000000	2026-11-01 01:52:24.626399+00
3PEAD66mvvBzb7kTTI8sxQ	\\x93c410c52c8f4c13b96f73f0bea6ae0f00f1dc83ad73657373696f6e5f65706f636800a7757365725f6964d92430663736656334392d336565662d343334622d386231652d396366323063326331613662ad70656e64696e675f736574757083a86f72675f6e616d65af4d79204f7267616e697a6174696f6ea47369746582a46e616d65a74d792053697465a7736974655f6964d92431653931353966362d646365382d346433392d613363632d656239343364316438343636a87573655f63617365a56f7468657299cd07eacd0131013500ce3797d688000000	2026-11-01 01:53:00.932697+00
120QId0-fUzeB1LiRcopOw	\\x93c4103b29ca45e25207de4c7d3edd21106dd783ad73657373696f6e5f65706f636800a7757365725f6964d92430663736656334392d336565662d343334622d386231652d396366323063326331613662ad70656e64696e675f736574757083a86f72675f6e616d65af4d79204f7267616e697a6174696f6ea47369746582a46e616d65a74d792053697465a7736974655f6964d92434663731656562362d306261662d346366662d393633622d393932653763633766333837a87573655f63617365a56f7468657299cd07eacd013101350cce2075bf9d000000	2026-11-01 01:53:12.544587+00
\.


--
-- Name: _sqlx_migrations _sqlx_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public._sqlx_migrations
    ADD CONSTRAINT _sqlx_migrations_pkey PRIMARY KEY (version);


--
-- Name: api_keys api_keys_key_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.api_keys
    ADD CONSTRAINT api_keys_key_key UNIQUE (key);


--
-- Name: api_keys api_keys_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.api_keys
    ADD CONSTRAINT api_keys_pkey PRIMARY KEY (id);


--
-- Name: bindings bindings_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.bindings
    ADD CONSTRAINT bindings_pkey PRIMARY KEY (id);


--
-- Name: credentials credentials_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.credentials
    ADD CONSTRAINT credentials_pkey PRIMARY KEY (id);


--
-- Name: daemon_interfaced_subnets daemon_interfaced_subnets_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.daemon_interfaced_subnets
    ADD CONSTRAINT daemon_interfaced_subnets_pkey PRIMARY KEY (daemon_id, subnet_id);


--
-- Name: daemons daemons_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.daemons
    ADD CONSTRAINT daemons_pkey PRIMARY KEY (id);


--
-- Name: discovery discovery_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.discovery
    ADD CONSTRAINT discovery_pkey PRIMARY KEY (id);


--
-- Name: discovery_warnings discovery_warnings_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.discovery_warnings
    ADD CONSTRAINT discovery_warnings_pkey PRIMARY KEY (id);


--
-- Name: entity_tags entity_tags_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.entity_tags
    ADD CONSTRAINT entity_tags_pkey PRIMARY KEY (id);


--
-- Name: dependency_members group_bindings_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dependency_members
    ADD CONSTRAINT group_bindings_pkey PRIMARY KEY (id);


--
-- Name: dependencies groups_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dependencies
    ADD CONSTRAINT groups_pkey PRIMARY KEY (id);


--
-- Name: host_credentials host_credentials_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.host_credentials
    ADD CONSTRAINT host_credentials_pkey PRIMARY KEY (host_id, credential_id);


--
-- Name: hosts hosts_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.hosts
    ADD CONSTRAINT hosts_pkey PRIMARY KEY (id);


--
-- Name: interface_neighbor_candidates interface_neighbor_candidates_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.interface_neighbor_candidates
    ADD CONSTRAINT interface_neighbor_candidates_pkey PRIMARY KEY (id);


--
-- Name: interface_neighbor_hosts interface_neighbor_hosts_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.interface_neighbor_hosts
    ADD CONSTRAINT interface_neighbor_hosts_pkey PRIMARY KEY (id);


--
-- Name: interface_neighbor_interfaces interface_neighbor_interfaces_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.interface_neighbor_interfaces
    ADD CONSTRAINT interface_neighbor_interfaces_pkey PRIMARY KEY (id);


--
-- Name: interfaces interfaces_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.interfaces
    ADD CONSTRAINT interfaces_pkey PRIMARY KEY (id);


--
-- Name: invites invites_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.invites
    ADD CONSTRAINT invites_pkey PRIMARY KEY (id);


--
-- Name: ip_addresses ip_addresses_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.ip_addresses
    ADD CONSTRAINT ip_addresses_pkey PRIMARY KEY (id);


--
-- Name: license_instances license_instances_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.license_instances
    ADD CONSTRAINT license_instances_pkey PRIMARY KEY (id);


--
-- Name: organizations organizations_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.organizations
    ADD CONSTRAINT organizations_pkey PRIMARY KEY (id);


--
-- Name: ports ports_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.ports
    ADD CONSTRAINT ports_pkey PRIMARY KEY (id);


--
-- Name: release_announcements release_announcements_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.release_announcements
    ADD CONSTRAINT release_announcements_pkey PRIMARY KEY (id);


--
-- Name: server_instance server_instance_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.server_instance
    ADD CONSTRAINT server_instance_pkey PRIMARY KEY (id);


--
-- Name: services services_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.services
    ADD CONSTRAINT services_pkey PRIMARY KEY (id);


--
-- Name: shares shares_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.shares
    ADD CONSTRAINT shares_pkey PRIMARY KEY (id);


--
-- Name: site_credentials site_credentials_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.site_credentials
    ADD CONSTRAINT site_credentials_pkey PRIMARY KEY (site_id, credential_id);


--
-- Name: sites sites_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.sites
    ADD CONSTRAINT sites_pkey PRIMARY KEY (id);


--
-- Name: snapshots snapshots_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.snapshots
    ADD CONSTRAINT snapshots_pkey PRIMARY KEY (id);


--
-- Name: subnet_vlans subnet_vlans_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.subnet_vlans
    ADD CONSTRAINT subnet_vlans_pkey PRIMARY KEY (id);


--
-- Name: subnets subnets_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.subnets
    ADD CONSTRAINT subnets_pkey PRIMARY KEY (id);


--
-- Name: tag_groups tag_groups_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.tag_groups
    ADD CONSTRAINT tag_groups_pkey PRIMARY KEY (id);


--
-- Name: tags tags_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.tags
    ADD CONSTRAINT tags_pkey PRIMARY KEY (id);


--
-- Name: topologies topologies_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.topologies
    ADD CONSTRAINT topologies_pkey PRIMARY KEY (id);


--
-- Name: user_api_key_site_access user_api_key_site_access_api_key_id_site_id_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_api_key_site_access
    ADD CONSTRAINT user_api_key_site_access_api_key_id_site_id_key UNIQUE (api_key_id, site_id);


--
-- Name: user_api_key_site_access user_api_key_site_access_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_api_key_site_access
    ADD CONSTRAINT user_api_key_site_access_pkey PRIMARY KEY (id);


--
-- Name: user_api_keys user_api_keys_key_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_api_keys
    ADD CONSTRAINT user_api_keys_key_key UNIQUE (key);


--
-- Name: user_api_keys user_api_keys_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_api_keys
    ADD CONSTRAINT user_api_keys_pkey PRIMARY KEY (id);


--
-- Name: user_site_access user_site_access_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_site_access
    ADD CONSTRAINT user_site_access_pkey PRIMARY KEY (id);


--
-- Name: user_site_access user_site_access_user_id_site_id_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_site_access
    ADD CONSTRAINT user_site_access_user_id_site_id_key UNIQUE (user_id, site_id);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: vlans vlans_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.vlans
    ADD CONSTRAINT vlans_pkey PRIMARY KEY (id);


--
-- Name: session session_pkey; Type: CONSTRAINT; Schema: tower_sessions; Owner: postgres
--

ALTER TABLE ONLY tower_sessions.session
    ADD CONSTRAINT session_pkey PRIMARY KEY (id);


--
-- Name: idx_api_keys_daemon_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE UNIQUE INDEX idx_api_keys_daemon_id ON public.api_keys USING btree (daemon_id) WHERE (daemon_id IS NOT NULL);


--
-- Name: idx_api_keys_key; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_api_keys_key ON public.api_keys USING btree (key);


--
-- Name: idx_api_keys_site; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_api_keys_site ON public.api_keys USING btree (site_id);


--
-- Name: idx_bindings_as_of; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_bindings_as_of ON public.bindings USING btree (site_id, valid_from, valid_to);


--
-- Name: idx_bindings_ip_address; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_bindings_ip_address ON public.bindings USING btree (ip_address_id);


--
-- Name: idx_bindings_lineage; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_bindings_lineage ON public.bindings USING btree (lineage_id) WHERE (valid_to IS NOT NULL);


--
-- Name: idx_bindings_live; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_bindings_live ON public.bindings USING btree (site_id) WHERE (valid_to IS NULL);


--
-- Name: idx_bindings_port; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_bindings_port ON public.bindings USING btree (port_id);


--
-- Name: idx_bindings_service; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_bindings_service ON public.bindings USING btree (service_id);


--
-- Name: idx_bindings_site; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_bindings_site ON public.bindings USING btree (site_id);


--
-- Name: idx_bindings_snapshot_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_bindings_snapshot_id ON public.bindings USING btree (snapshot_id) WHERE (snapshot_id IS NOT NULL);


--
-- Name: idx_credentials_org; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_credentials_org ON public.credentials USING btree (organization_id);


--
-- Name: idx_credentials_type; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_credentials_type ON public.credentials USING btree (((credential_type ->> 'type'::text)));


--
-- Name: idx_daemon_host_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_daemon_host_id ON public.daemons USING btree (host_id);


--
-- Name: idx_daemon_interfaced_subnets_subnet_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_daemon_interfaced_subnets_subnet_id ON public.daemon_interfaced_subnets USING btree (subnet_id);


--
-- Name: idx_daemons_api_key; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_daemons_api_key ON public.daemons USING btree (api_key_id) WHERE (api_key_id IS NOT NULL);


--
-- Name: idx_daemons_site; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_daemons_site ON public.daemons USING btree (site_id);


--
-- Name: idx_dependencies_as_of; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_dependencies_as_of ON public.dependencies USING btree (site_id, valid_from, valid_to);


--
-- Name: idx_dependencies_lineage; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_dependencies_lineage ON public.dependencies USING btree (lineage_id) WHERE (valid_to IS NOT NULL);


--
-- Name: idx_dependencies_live; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_dependencies_live ON public.dependencies USING btree (site_id) WHERE (valid_to IS NULL);


--
-- Name: idx_dependencies_site; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_dependencies_site ON public.dependencies USING btree (site_id);


--
-- Name: idx_dependencies_snapshot_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_dependencies_snapshot_id ON public.dependencies USING btree (snapshot_id) WHERE (snapshot_id IS NOT NULL);


--
-- Name: idx_dependency_members_as_of; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_dependency_members_as_of ON public.dependency_members USING btree (dependency_id, valid_from, valid_to);


--
-- Name: idx_dependency_members_binding; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_dependency_members_binding ON public.dependency_members USING btree (binding_id) WHERE (binding_id IS NOT NULL);


--
-- Name: idx_dependency_members_dependency; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_dependency_members_dependency ON public.dependency_members USING btree (dependency_id);


--
-- Name: idx_dependency_members_lineage; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_dependency_members_lineage ON public.dependency_members USING btree (lineage_id) WHERE (valid_to IS NOT NULL);


--
-- Name: idx_dependency_members_live; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_dependency_members_live ON public.dependency_members USING btree (dependency_id) WHERE (valid_to IS NULL);


--
-- Name: idx_dependency_members_service; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_dependency_members_service ON public.dependency_members USING btree (service_id);


--
-- Name: idx_dependency_members_snapshot_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_dependency_members_snapshot_id ON public.dependency_members USING btree (snapshot_id) WHERE (snapshot_id IS NOT NULL);


--
-- Name: idx_dependency_members_unique_live; Type: INDEX; Schema: public; Owner: postgres
--

CREATE UNIQUE INDEX idx_dependency_members_unique_live ON public.dependency_members USING btree (dependency_id, service_id) WHERE (valid_to IS NULL);


--
-- Name: idx_discovery_daemon; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_discovery_daemon ON public.discovery USING btree (daemon_id);


--
-- Name: idx_discovery_site; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_discovery_site ON public.discovery USING btree (site_id);


--
-- Name: idx_discovery_warnings_credential; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_discovery_warnings_credential ON public.discovery_warnings USING btree (credential_id) WHERE (credential_id IS NOT NULL);


--
-- Name: idx_discovery_warnings_discovery; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_discovery_warnings_discovery ON public.discovery_warnings USING btree (discovery_id);


--
-- Name: idx_discovery_warnings_host; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_discovery_warnings_host ON public.discovery_warnings USING btree (host_id) WHERE (host_id IS NOT NULL);


--
-- Name: idx_discovery_warnings_remote_host; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_discovery_warnings_remote_host ON public.discovery_warnings USING btree (remote_host_id) WHERE (remote_host_id IS NOT NULL);


--
-- Name: idx_discovery_warnings_site; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_discovery_warnings_site ON public.discovery_warnings USING btree (site_id);


--
-- Name: idx_discovery_warnings_subnet; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_discovery_warnings_subnet ON public.discovery_warnings USING btree (subnet_id) WHERE (subnet_id IS NOT NULL);


--
-- Name: idx_entity_tags_as_of; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_entity_tags_as_of ON public.entity_tags USING btree (entity_id, entity_type, valid_from, valid_to);


--
-- Name: idx_entity_tags_entity; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_entity_tags_entity ON public.entity_tags USING btree (entity_id, entity_type);


--
-- Name: idx_entity_tags_lineage; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_entity_tags_lineage ON public.entity_tags USING btree (lineage_id) WHERE (valid_to IS NOT NULL);


--
-- Name: idx_entity_tags_live; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_entity_tags_live ON public.entity_tags USING btree (entity_id, entity_type) WHERE (valid_to IS NULL);


--
-- Name: idx_entity_tags_snapshot_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_entity_tags_snapshot_id ON public.entity_tags USING btree (snapshot_id) WHERE (snapshot_id IS NOT NULL);


--
-- Name: idx_entity_tags_tag_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_entity_tags_tag_id ON public.entity_tags USING btree (tag_id);


--
-- Name: idx_entity_tags_unique_live; Type: INDEX; Schema: public; Owner: postgres
--

CREATE UNIQUE INDEX idx_entity_tags_unique_live ON public.entity_tags USING btree (entity_id, entity_type, tag_id) WHERE (valid_to IS NULL);


--
-- Name: idx_hosts_as_of; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_hosts_as_of ON public.hosts USING btree (site_id, valid_from, valid_to);


--
-- Name: idx_hosts_chassis_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_hosts_chassis_id ON public.hosts USING btree (chassis_id);


--
-- Name: idx_hosts_lineage; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_hosts_lineage ON public.hosts USING btree (lineage_id) WHERE (valid_to IS NOT NULL);


--
-- Name: idx_hosts_live; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_hosts_live ON public.hosts USING btree (site_id) WHERE (valid_to IS NULL);


--
-- Name: idx_hosts_site; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_hosts_site ON public.hosts USING btree (site_id);


--
-- Name: idx_hosts_snapshot_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_hosts_snapshot_id ON public.hosts USING btree (snapshot_id) WHERE (snapshot_id IS NOT NULL);


--
-- Name: idx_hosts_virtualization_interface_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_hosts_virtualization_interface_id ON public.hosts USING btree (virtualization_interface_id) WHERE (virtualization_interface_id IS NOT NULL);


--
-- Name: idx_hosts_virtualization_service_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_hosts_virtualization_service_id ON public.hosts USING btree (virtualization_service_id) WHERE (virtualization_service_id IS NOT NULL);


--
-- Name: idx_interface_neighbor_candidates_interface; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_interface_neighbor_candidates_interface ON public.interface_neighbor_candidates USING btree (interface_id);


--
-- Name: idx_interface_neighbor_candidates_site; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_interface_neighbor_candidates_site ON public.interface_neighbor_candidates USING btree (site_id);


--
-- Name: idx_interface_neighbor_hosts_as_of; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_interface_neighbor_hosts_as_of ON public.interface_neighbor_hosts USING btree (site_id, valid_from, valid_to);


--
-- Name: idx_interface_neighbor_hosts_lineage; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_interface_neighbor_hosts_lineage ON public.interface_neighbor_hosts USING btree (lineage_id) WHERE (valid_to IS NOT NULL);


--
-- Name: idx_interface_neighbor_hosts_live; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_interface_neighbor_hosts_live ON public.interface_neighbor_hosts USING btree (site_id) WHERE (valid_to IS NULL);


--
-- Name: idx_interface_neighbor_hosts_natural_key; Type: INDEX; Schema: public; Owner: postgres
--

CREATE UNIQUE INDEX idx_interface_neighbor_hosts_natural_key ON public.interface_neighbor_hosts USING btree (interface_id, neighbor_host_id) WHERE (valid_to IS NULL);


--
-- Name: idx_interface_neighbor_hosts_neighbor; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_interface_neighbor_hosts_neighbor ON public.interface_neighbor_hosts USING btree (neighbor_host_id);


--
-- Name: idx_interface_neighbor_hosts_snapshot_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_interface_neighbor_hosts_snapshot_id ON public.interface_neighbor_hosts USING btree (snapshot_id) WHERE (snapshot_id IS NOT NULL);


--
-- Name: idx_interface_neighbor_interfaces_as_of; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_interface_neighbor_interfaces_as_of ON public.interface_neighbor_interfaces USING btree (site_id, valid_from, valid_to);


--
-- Name: idx_interface_neighbor_interfaces_lineage; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_interface_neighbor_interfaces_lineage ON public.interface_neighbor_interfaces USING btree (lineage_id) WHERE (valid_to IS NOT NULL);


--
-- Name: idx_interface_neighbor_interfaces_live; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_interface_neighbor_interfaces_live ON public.interface_neighbor_interfaces USING btree (site_id) WHERE (valid_to IS NULL);


--
-- Name: idx_interface_neighbor_interfaces_natural_key; Type: INDEX; Schema: public; Owner: postgres
--

CREATE UNIQUE INDEX idx_interface_neighbor_interfaces_natural_key ON public.interface_neighbor_interfaces USING btree (interface_id, neighbor_interface_id) WHERE (valid_to IS NULL);


--
-- Name: idx_interface_neighbor_interfaces_neighbor; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_interface_neighbor_interfaces_neighbor ON public.interface_neighbor_interfaces USING btree (neighbor_interface_id);


--
-- Name: idx_interface_neighbor_interfaces_snapshot_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_interface_neighbor_interfaces_snapshot_id ON public.interface_neighbor_interfaces USING btree (snapshot_id) WHERE (snapshot_id IS NOT NULL);


--
-- Name: idx_interfaces_as_of; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_interfaces_as_of ON public.interfaces USING btree (site_id, valid_from, valid_to);


--
-- Name: idx_interfaces_host; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_interfaces_host ON public.interfaces USING btree (host_id);


--
-- Name: idx_interfaces_host_if_index; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_interfaces_host_if_index ON public.interfaces USING btree (host_id, if_index);


--
-- Name: idx_interfaces_host_name_live; Type: INDEX; Schema: public; Owner: postgres
--

CREATE UNIQUE INDEX idx_interfaces_host_name_live ON public.interfaces USING btree (host_id, if_name) WHERE ((if_name IS NOT NULL) AND (valid_to IS NULL));


--
-- Name: idx_interfaces_ip_address; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_interfaces_ip_address ON public.interfaces USING btree (ip_address_id);


--
-- Name: idx_interfaces_lineage; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_interfaces_lineage ON public.interfaces USING btree (lineage_id) WHERE (valid_to IS NOT NULL);


--
-- Name: idx_interfaces_live; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_interfaces_live ON public.interfaces USING btree (site_id) WHERE (valid_to IS NULL);


--
-- Name: idx_interfaces_mac_address; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_interfaces_mac_address ON public.interfaces USING btree (mac_address);


--
-- Name: idx_interfaces_site; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_interfaces_site ON public.interfaces USING btree (site_id);


--
-- Name: idx_interfaces_snapshot_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_interfaces_snapshot_id ON public.interfaces USING btree (snapshot_id) WHERE (snapshot_id IS NOT NULL);


--
-- Name: idx_invites_expires_at; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_invites_expires_at ON public.invites USING btree (expires_at);


--
-- Name: idx_invites_organization; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_invites_organization ON public.invites USING btree (organization_id);


--
-- Name: idx_ip_addresses_as_of; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_ip_addresses_as_of ON public.ip_addresses USING btree (site_id, valid_from, valid_to);


--
-- Name: idx_ip_addresses_host; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_ip_addresses_host ON public.ip_addresses USING btree (host_id);


--
-- Name: idx_ip_addresses_host_mac; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_ip_addresses_host_mac ON public.ip_addresses USING btree (host_id, mac_address) WHERE (mac_address IS NOT NULL);


--
-- Name: idx_ip_addresses_lineage; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_ip_addresses_lineage ON public.ip_addresses USING btree (lineage_id) WHERE (valid_to IS NOT NULL);


--
-- Name: idx_ip_addresses_live; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_ip_addresses_live ON public.ip_addresses USING btree (site_id) WHERE (valid_to IS NULL);


--
-- Name: idx_ip_addresses_site; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_ip_addresses_site ON public.ip_addresses USING btree (site_id);


--
-- Name: idx_ip_addresses_site_mac; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_ip_addresses_site_mac ON public.ip_addresses USING btree (site_id, mac_address) WHERE (mac_address IS NOT NULL);


--
-- Name: idx_ip_addresses_snapshot_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_ip_addresses_snapshot_id ON public.ip_addresses USING btree (snapshot_id) WHERE (snapshot_id IS NOT NULL);


--
-- Name: idx_ip_addresses_subnet; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_ip_addresses_subnet ON public.ip_addresses USING btree (subnet_id);


--
-- Name: idx_ip_addresses_unique_live; Type: INDEX; Schema: public; Owner: postgres
--

CREATE UNIQUE INDEX idx_ip_addresses_unique_live ON public.ip_addresses USING btree (host_id, subnet_id, ip_address) WHERE (valid_to IS NULL);


--
-- Name: idx_license_instances_organization; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_license_instances_organization ON public.license_instances USING btree (organization_id);


--
-- Name: idx_organizations_stripe_customer; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_organizations_stripe_customer ON public.organizations USING btree (stripe_customer_id);


--
-- Name: idx_ports_as_of; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_ports_as_of ON public.ports USING btree (site_id, valid_from, valid_to);


--
-- Name: idx_ports_host; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_ports_host ON public.ports USING btree (host_id);


--
-- Name: idx_ports_lineage; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_ports_lineage ON public.ports USING btree (lineage_id) WHERE (valid_to IS NOT NULL);


--
-- Name: idx_ports_live; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_ports_live ON public.ports USING btree (site_id) WHERE (valid_to IS NULL);


--
-- Name: idx_ports_number; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_ports_number ON public.ports USING btree (port_number);


--
-- Name: idx_ports_site; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_ports_site ON public.ports USING btree (site_id);


--
-- Name: idx_ports_snapshot_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_ports_snapshot_id ON public.ports USING btree (snapshot_id) WHERE (snapshot_id IS NOT NULL);


--
-- Name: idx_ports_unique_live; Type: INDEX; Schema: public; Owner: postgres
--

CREATE UNIQUE INDEX idx_ports_unique_live ON public.ports USING btree (host_id, port_number, protocol) WHERE (valid_to IS NULL);


--
-- Name: idx_release_announcements_version; Type: INDEX; Schema: public; Owner: postgres
--

CREATE UNIQUE INDEX idx_release_announcements_version ON public.release_announcements USING btree (version);


--
-- Name: idx_services_as_of; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_services_as_of ON public.services USING btree (site_id, valid_from, valid_to);


--
-- Name: idx_services_host_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_services_host_id ON public.services USING btree (host_id);


--
-- Name: idx_services_host_position; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_services_host_position ON public.services USING btree (host_id, "position");


--
-- Name: idx_services_lineage; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_services_lineage ON public.services USING btree (lineage_id) WHERE (valid_to IS NOT NULL);


--
-- Name: idx_services_live; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_services_live ON public.services USING btree (site_id) WHERE (valid_to IS NULL);


--
-- Name: idx_services_site; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_services_site ON public.services USING btree (site_id);


--
-- Name: idx_services_snapshot_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_services_snapshot_id ON public.services USING btree (snapshot_id) WHERE (snapshot_id IS NOT NULL);


--
-- Name: idx_services_virtualization_service_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_services_virtualization_service_id ON public.services USING btree (virtualization_service_id) WHERE (virtualization_service_id IS NOT NULL);


--
-- Name: idx_shares_enabled; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_shares_enabled ON public.shares USING btree (is_enabled) WHERE (is_enabled = true);


--
-- Name: idx_shares_site; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_shares_site ON public.shares USING btree (site_id);


--
-- Name: idx_shares_topology; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_shares_topology ON public.shares USING btree (topology_id);


--
-- Name: idx_sites_owner_organization; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_sites_owner_organization ON public.sites USING btree (organization_id);


--
-- Name: idx_snapshots_site_taken_at; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_snapshots_site_taken_at ON public.snapshots USING btree (site_id, taken_at DESC);


--
-- Name: idx_subnet_vlans_as_of; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_subnet_vlans_as_of ON public.subnet_vlans USING btree (subnet_id, valid_from, valid_to);


--
-- Name: idx_subnet_vlans_lineage; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_subnet_vlans_lineage ON public.subnet_vlans USING btree (lineage_id) WHERE (valid_to IS NOT NULL);


--
-- Name: idx_subnet_vlans_live; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_subnet_vlans_live ON public.subnet_vlans USING btree (subnet_id) WHERE (valid_to IS NULL);


--
-- Name: idx_subnet_vlans_snapshot_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_subnet_vlans_snapshot_id ON public.subnet_vlans USING btree (snapshot_id) WHERE (snapshot_id IS NOT NULL);


--
-- Name: idx_subnet_vlans_subnet; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_subnet_vlans_subnet ON public.subnet_vlans USING btree (subnet_id);


--
-- Name: idx_subnet_vlans_unique_live; Type: INDEX; Schema: public; Owner: postgres
--

CREATE UNIQUE INDEX idx_subnet_vlans_unique_live ON public.subnet_vlans USING btree (subnet_id, vlan_id) WHERE (valid_to IS NULL);


--
-- Name: idx_subnet_vlans_vlan; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_subnet_vlans_vlan ON public.subnet_vlans USING btree (vlan_id);


--
-- Name: idx_subnets_as_of; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_subnets_as_of ON public.subnets USING btree (site_id, valid_from, valid_to);


--
-- Name: idx_subnets_lineage; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_subnets_lineage ON public.subnets USING btree (lineage_id) WHERE (valid_to IS NOT NULL);


--
-- Name: idx_subnets_live; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_subnets_live ON public.subnets USING btree (site_id) WHERE (valid_to IS NULL);


--
-- Name: idx_subnets_site; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_subnets_site ON public.subnets USING btree (site_id);


--
-- Name: idx_subnets_snapshot_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_subnets_snapshot_id ON public.subnets USING btree (snapshot_id) WHERE (snapshot_id IS NOT NULL);


--
-- Name: idx_subnets_virtualization_service_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_subnets_virtualization_service_id ON public.subnets USING btree (virtualization_service_id) WHERE (virtualization_service_id IS NOT NULL);


--
-- Name: idx_tag_groups_org_name; Type: INDEX; Schema: public; Owner: postgres
--

CREATE UNIQUE INDEX idx_tag_groups_org_name ON public.tag_groups USING btree (organization_id, name);


--
-- Name: idx_tag_groups_org_system; Type: INDEX; Schema: public; Owner: postgres
--

CREATE UNIQUE INDEX idx_tag_groups_org_system ON public.tag_groups USING btree (organization_id, group_type) WHERE (group_type <> 'Custom'::text);


--
-- Name: idx_tags_as_of; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_tags_as_of ON public.tags USING btree (organization_id, valid_from, valid_to);


--
-- Name: idx_tags_lineage; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_tags_lineage ON public.tags USING btree (lineage_id) WHERE (valid_to IS NOT NULL);


--
-- Name: idx_tags_live; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_tags_live ON public.tags USING btree (organization_id) WHERE (valid_to IS NULL);


--
-- Name: idx_tags_org_name_live; Type: INDEX; Schema: public; Owner: postgres
--

CREATE UNIQUE INDEX idx_tags_org_name_live ON public.tags USING btree (organization_id, name) WHERE (valid_to IS NULL);


--
-- Name: idx_tags_organization; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_tags_organization ON public.tags USING btree (organization_id);


--
-- Name: idx_tags_snapshot_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_tags_snapshot_id ON public.tags USING btree (snapshot_id) WHERE (snapshot_id IS NOT NULL);


--
-- Name: idx_tags_tag_group_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_tags_tag_group_id ON public.tags USING btree (tag_group_id);


--
-- Name: idx_topologies_site; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_topologies_site ON public.topologies USING btree (site_id);


--
-- Name: idx_user_api_key_site_access_key; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_user_api_key_site_access_key ON public.user_api_key_site_access USING btree (api_key_id);


--
-- Name: idx_user_api_key_site_access_site; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_user_api_key_site_access_site ON public.user_api_key_site_access USING btree (site_id);


--
-- Name: idx_user_api_keys_key; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_user_api_keys_key ON public.user_api_keys USING btree (key);


--
-- Name: idx_user_api_keys_org; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_user_api_keys_org ON public.user_api_keys USING btree (organization_id);


--
-- Name: idx_user_api_keys_user; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_user_api_keys_user ON public.user_api_keys USING btree (user_id);


--
-- Name: idx_user_site_access_site; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_user_site_access_site ON public.user_site_access USING btree (site_id);


--
-- Name: idx_user_site_access_user; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_user_site_access_user ON public.user_site_access USING btree (user_id);


--
-- Name: idx_users_email_lower; Type: INDEX; Schema: public; Owner: postgres
--

CREATE UNIQUE INDEX idx_users_email_lower ON public.users USING btree (lower(email));


--
-- Name: idx_users_email_unsubscribe_token; Type: INDEX; Schema: public; Owner: postgres
--

CREATE UNIQUE INDEX idx_users_email_unsubscribe_token ON public.users USING btree (email_unsubscribe_token);


--
-- Name: idx_users_email_verification_token; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_users_email_verification_token ON public.users USING btree (email_verification_token) WHERE (email_verification_token IS NOT NULL);


--
-- Name: idx_users_oidc_provider_subject; Type: INDEX; Schema: public; Owner: postgres
--

CREATE UNIQUE INDEX idx_users_oidc_provider_subject ON public.users USING btree (oidc_provider, oidc_subject) WHERE ((oidc_provider IS NOT NULL) AND (oidc_subject IS NOT NULL));


--
-- Name: idx_users_organization; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_users_organization ON public.users USING btree (organization_id);


--
-- Name: idx_users_password_reset_token; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_users_password_reset_token ON public.users USING btree (password_reset_token) WHERE (password_reset_token IS NOT NULL);


--
-- Name: idx_vlans_as_of; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_vlans_as_of ON public.vlans USING btree (site_id, valid_from, valid_to);


--
-- Name: idx_vlans_lineage; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_vlans_lineage ON public.vlans USING btree (lineage_id) WHERE (valid_to IS NOT NULL);


--
-- Name: idx_vlans_live; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_vlans_live ON public.vlans USING btree (site_id) WHERE (valid_to IS NULL);


--
-- Name: idx_vlans_organization; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_vlans_organization ON public.vlans USING btree (organization_id);


--
-- Name: idx_vlans_site; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_vlans_site ON public.vlans USING btree (site_id);


--
-- Name: idx_vlans_site_number_live; Type: INDEX; Schema: public; Owner: postgres
--

CREATE UNIQUE INDEX idx_vlans_site_number_live ON public.vlans USING btree (site_id, vlan_number) WHERE (valid_to IS NULL);


--
-- Name: idx_vlans_snapshot_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_vlans_snapshot_id ON public.vlans USING btree (snapshot_id) WHERE (snapshot_id IS NOT NULL);


--
-- Name: users reassign_daemons_before_user_delete; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER reassign_daemons_before_user_delete BEFORE DELETE ON public.users FOR EACH ROW EXECUTE FUNCTION public.reassign_daemons_on_user_delete();


--
-- Name: api_keys api_keys_daemon_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.api_keys
    ADD CONSTRAINT api_keys_daemon_id_fkey FOREIGN KEY (daemon_id) REFERENCES public.daemons(id) ON DELETE CASCADE;


--
-- Name: api_keys api_keys_site_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.api_keys
    ADD CONSTRAINT api_keys_site_id_fkey FOREIGN KEY (site_id) REFERENCES public.sites(id) ON DELETE CASCADE;


--
-- Name: bindings bindings_first_discovery_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.bindings
    ADD CONSTRAINT bindings_first_discovery_fk FOREIGN KEY (first_discovery_id) REFERENCES public.discovery(id) ON DELETE SET NULL;


--
-- Name: bindings bindings_ip_address_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.bindings
    ADD CONSTRAINT bindings_ip_address_id_fkey FOREIGN KEY (ip_address_id) REFERENCES public.ip_addresses(id) ON DELETE CASCADE;


--
-- Name: bindings bindings_last_discovery_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.bindings
    ADD CONSTRAINT bindings_last_discovery_fk FOREIGN KEY (last_discovery_id) REFERENCES public.discovery(id) ON DELETE SET NULL;


--
-- Name: bindings bindings_port_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.bindings
    ADD CONSTRAINT bindings_port_id_fkey FOREIGN KEY (port_id) REFERENCES public.ports(id) ON DELETE CASCADE;


--
-- Name: bindings bindings_service_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.bindings
    ADD CONSTRAINT bindings_service_id_fkey FOREIGN KEY (service_id) REFERENCES public.services(id) ON DELETE CASCADE;


--
-- Name: bindings bindings_site_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.bindings
    ADD CONSTRAINT bindings_site_id_fkey FOREIGN KEY (site_id) REFERENCES public.sites(id) ON DELETE CASCADE;


--
-- Name: bindings bindings_snapshot_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.bindings
    ADD CONSTRAINT bindings_snapshot_id_fkey FOREIGN KEY (snapshot_id) REFERENCES public.snapshots(id) ON DELETE CASCADE;


--
-- Name: credentials credentials_organization_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.credentials
    ADD CONSTRAINT credentials_organization_id_fkey FOREIGN KEY (organization_id) REFERENCES public.organizations(id) ON DELETE CASCADE;


--
-- Name: daemon_interfaced_subnets daemon_interfaced_subnets_daemon_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.daemon_interfaced_subnets
    ADD CONSTRAINT daemon_interfaced_subnets_daemon_id_fkey FOREIGN KEY (daemon_id) REFERENCES public.daemons(id) ON DELETE CASCADE;


--
-- Name: daemon_interfaced_subnets daemon_interfaced_subnets_subnet_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.daemon_interfaced_subnets
    ADD CONSTRAINT daemon_interfaced_subnets_subnet_id_fkey FOREIGN KEY (subnet_id) REFERENCES public.subnets(id) ON DELETE CASCADE;


--
-- Name: daemons daemons_api_key_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.daemons
    ADD CONSTRAINT daemons_api_key_id_fkey FOREIGN KEY (api_key_id) REFERENCES public.api_keys(id) ON DELETE SET NULL;


--
-- Name: daemons daemons_site_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.daemons
    ADD CONSTRAINT daemons_site_id_fkey FOREIGN KEY (site_id) REFERENCES public.sites(id) ON DELETE CASCADE;


--
-- Name: daemons daemons_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.daemons
    ADD CONSTRAINT daemons_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id);


--
-- Name: dependencies dependencies_site_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dependencies
    ADD CONSTRAINT dependencies_site_id_fkey FOREIGN KEY (site_id) REFERENCES public.sites(id) ON DELETE CASCADE;


--
-- Name: dependencies dependencies_snapshot_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dependencies
    ADD CONSTRAINT dependencies_snapshot_id_fkey FOREIGN KEY (snapshot_id) REFERENCES public.snapshots(id) ON DELETE CASCADE;


--
-- Name: dependency_members dependency_members_snapshot_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dependency_members
    ADD CONSTRAINT dependency_members_snapshot_id_fkey FOREIGN KEY (snapshot_id) REFERENCES public.snapshots(id) ON DELETE CASCADE;


--
-- Name: discovery discovery_daemon_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.discovery
    ADD CONSTRAINT discovery_daemon_id_fkey FOREIGN KEY (daemon_id) REFERENCES public.daemons(id) ON DELETE CASCADE;


--
-- Name: discovery discovery_site_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.discovery
    ADD CONSTRAINT discovery_site_id_fkey FOREIGN KEY (site_id) REFERENCES public.sites(id) ON DELETE CASCADE;


--
-- Name: discovery_warnings discovery_warnings_credential_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.discovery_warnings
    ADD CONSTRAINT discovery_warnings_credential_id_fkey FOREIGN KEY (credential_id) REFERENCES public.credentials(id) ON DELETE SET NULL;


--
-- Name: discovery_warnings discovery_warnings_discovery_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.discovery_warnings
    ADD CONSTRAINT discovery_warnings_discovery_id_fkey FOREIGN KEY (discovery_id) REFERENCES public.discovery(id) ON DELETE CASCADE;


--
-- Name: discovery_warnings discovery_warnings_host_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.discovery_warnings
    ADD CONSTRAINT discovery_warnings_host_id_fkey FOREIGN KEY (host_id) REFERENCES public.hosts(id) ON DELETE SET NULL;


--
-- Name: discovery_warnings discovery_warnings_remote_host_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.discovery_warnings
    ADD CONSTRAINT discovery_warnings_remote_host_id_fkey FOREIGN KEY (remote_host_id) REFERENCES public.hosts(id) ON DELETE SET NULL;


--
-- Name: discovery_warnings discovery_warnings_site_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.discovery_warnings
    ADD CONSTRAINT discovery_warnings_site_id_fkey FOREIGN KEY (site_id) REFERENCES public.sites(id) ON DELETE CASCADE;


--
-- Name: discovery_warnings discovery_warnings_subnet_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.discovery_warnings
    ADD CONSTRAINT discovery_warnings_subnet_id_fkey FOREIGN KEY (subnet_id) REFERENCES public.subnets(id) ON DELETE SET NULL;


--
-- Name: entity_tags entity_tags_snapshot_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.entity_tags
    ADD CONSTRAINT entity_tags_snapshot_id_fkey FOREIGN KEY (snapshot_id) REFERENCES public.snapshots(id) ON DELETE CASCADE;


--
-- Name: entity_tags entity_tags_tag_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.entity_tags
    ADD CONSTRAINT entity_tags_tag_id_fkey FOREIGN KEY (tag_id) REFERENCES public.tags(id) ON DELETE CASCADE;


--
-- Name: dependency_members group_bindings_group_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dependency_members
    ADD CONSTRAINT group_bindings_group_id_fkey FOREIGN KEY (dependency_id) REFERENCES public.dependencies(id) ON DELETE CASCADE;


--
-- Name: host_credentials host_credentials_credential_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.host_credentials
    ADD CONSTRAINT host_credentials_credential_id_fkey FOREIGN KEY (credential_id) REFERENCES public.credentials(id) ON DELETE CASCADE;


--
-- Name: host_credentials host_credentials_host_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.host_credentials
    ADD CONSTRAINT host_credentials_host_id_fkey FOREIGN KEY (host_id) REFERENCES public.hosts(id) ON DELETE CASCADE;


--
-- Name: hosts hosts_first_discovery_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.hosts
    ADD CONSTRAINT hosts_first_discovery_fk FOREIGN KEY (first_discovery_id) REFERENCES public.discovery(id) ON DELETE SET NULL;


--
-- Name: hosts hosts_last_discovery_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.hosts
    ADD CONSTRAINT hosts_last_discovery_fk FOREIGN KEY (last_discovery_id) REFERENCES public.discovery(id) ON DELETE SET NULL;


--
-- Name: hosts hosts_site_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.hosts
    ADD CONSTRAINT hosts_site_id_fkey FOREIGN KEY (site_id) REFERENCES public.sites(id) ON DELETE CASCADE;


--
-- Name: hosts hosts_snapshot_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.hosts
    ADD CONSTRAINT hosts_snapshot_id_fkey FOREIGN KEY (snapshot_id) REFERENCES public.snapshots(id) ON DELETE CASCADE;


--
-- Name: hosts hosts_virtualization_interface_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.hosts
    ADD CONSTRAINT hosts_virtualization_interface_id_fkey FOREIGN KEY (virtualization_interface_id) REFERENCES public.interfaces(id) ON DELETE SET NULL;


--
-- Name: hosts hosts_virtualization_service_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.hosts
    ADD CONSTRAINT hosts_virtualization_service_id_fkey FOREIGN KEY (virtualization_service_id) REFERENCES public.services(id) ON DELETE SET NULL;


--
-- Name: interface_neighbor_candidates interface_neighbor_candidates_interface_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.interface_neighbor_candidates
    ADD CONSTRAINT interface_neighbor_candidates_interface_id_fkey FOREIGN KEY (interface_id) REFERENCES public.interfaces(id) ON DELETE CASCADE;


--
-- Name: interface_neighbor_candidates interface_neighbor_candidates_site_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.interface_neighbor_candidates
    ADD CONSTRAINT interface_neighbor_candidates_site_id_fkey FOREIGN KEY (site_id) REFERENCES public.sites(id) ON DELETE CASCADE;


--
-- Name: interface_neighbor_hosts interface_neighbor_hosts_interface_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.interface_neighbor_hosts
    ADD CONSTRAINT interface_neighbor_hosts_interface_id_fkey FOREIGN KEY (interface_id) REFERENCES public.interfaces(id) ON DELETE CASCADE;


--
-- Name: interface_neighbor_hosts interface_neighbor_hosts_neighbor_host_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.interface_neighbor_hosts
    ADD CONSTRAINT interface_neighbor_hosts_neighbor_host_id_fkey FOREIGN KEY (neighbor_host_id) REFERENCES public.hosts(id) ON DELETE CASCADE;


--
-- Name: interface_neighbor_hosts interface_neighbor_hosts_site_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.interface_neighbor_hosts
    ADD CONSTRAINT interface_neighbor_hosts_site_id_fkey FOREIGN KEY (site_id) REFERENCES public.sites(id) ON DELETE CASCADE;


--
-- Name: interface_neighbor_hosts interface_neighbor_hosts_snapshot_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.interface_neighbor_hosts
    ADD CONSTRAINT interface_neighbor_hosts_snapshot_id_fkey FOREIGN KEY (snapshot_id) REFERENCES public.snapshots(id) ON DELETE CASCADE;


--
-- Name: interface_neighbor_interfaces interface_neighbor_interfaces_interface_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.interface_neighbor_interfaces
    ADD CONSTRAINT interface_neighbor_interfaces_interface_id_fkey FOREIGN KEY (interface_id) REFERENCES public.interfaces(id) ON DELETE CASCADE;


--
-- Name: interface_neighbor_interfaces interface_neighbor_interfaces_neighbor_interface_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.interface_neighbor_interfaces
    ADD CONSTRAINT interface_neighbor_interfaces_neighbor_interface_id_fkey FOREIGN KEY (neighbor_interface_id) REFERENCES public.interfaces(id) ON DELETE CASCADE;


--
-- Name: interface_neighbor_interfaces interface_neighbor_interfaces_site_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.interface_neighbor_interfaces
    ADD CONSTRAINT interface_neighbor_interfaces_site_id_fkey FOREIGN KEY (site_id) REFERENCES public.sites(id) ON DELETE CASCADE;


--
-- Name: interface_neighbor_interfaces interface_neighbor_interfaces_snapshot_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.interface_neighbor_interfaces
    ADD CONSTRAINT interface_neighbor_interfaces_snapshot_id_fkey FOREIGN KEY (snapshot_id) REFERENCES public.snapshots(id) ON DELETE CASCADE;


--
-- Name: interfaces interfaces_first_discovery_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.interfaces
    ADD CONSTRAINT interfaces_first_discovery_fk FOREIGN KEY (first_discovery_id) REFERENCES public.discovery(id) ON DELETE SET NULL;


--
-- Name: interfaces interfaces_host_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.interfaces
    ADD CONSTRAINT interfaces_host_id_fkey FOREIGN KEY (host_id) REFERENCES public.hosts(id) ON DELETE CASCADE;


--
-- Name: interfaces interfaces_ip_address_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.interfaces
    ADD CONSTRAINT interfaces_ip_address_id_fkey FOREIGN KEY (ip_address_id) REFERENCES public.ip_addresses(id) ON DELETE SET NULL;


--
-- Name: interfaces interfaces_last_discovery_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.interfaces
    ADD CONSTRAINT interfaces_last_discovery_fk FOREIGN KEY (last_discovery_id) REFERENCES public.discovery(id) ON DELETE SET NULL;


--
-- Name: interfaces interfaces_native_vlan_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.interfaces
    ADD CONSTRAINT interfaces_native_vlan_id_fkey FOREIGN KEY (native_vlan_id) REFERENCES public.vlans(id) ON DELETE SET NULL;


--
-- Name: interfaces interfaces_site_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.interfaces
    ADD CONSTRAINT interfaces_site_id_fkey FOREIGN KEY (site_id) REFERENCES public.sites(id) ON DELETE CASCADE;


--
-- Name: interfaces interfaces_snapshot_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.interfaces
    ADD CONSTRAINT interfaces_snapshot_id_fkey FOREIGN KEY (snapshot_id) REFERENCES public.snapshots(id) ON DELETE CASCADE;


--
-- Name: invites invites_created_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.invites
    ADD CONSTRAINT invites_created_by_fkey FOREIGN KEY (created_by) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: invites invites_organization_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.invites
    ADD CONSTRAINT invites_organization_id_fkey FOREIGN KEY (organization_id) REFERENCES public.organizations(id) ON DELETE CASCADE;


--
-- Name: ip_addresses ip_addresses_first_discovery_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.ip_addresses
    ADD CONSTRAINT ip_addresses_first_discovery_fk FOREIGN KEY (first_discovery_id) REFERENCES public.discovery(id) ON DELETE SET NULL;


--
-- Name: ip_addresses ip_addresses_host_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.ip_addresses
    ADD CONSTRAINT ip_addresses_host_id_fkey FOREIGN KEY (host_id) REFERENCES public.hosts(id) ON DELETE CASCADE;


--
-- Name: ip_addresses ip_addresses_last_discovery_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.ip_addresses
    ADD CONSTRAINT ip_addresses_last_discovery_fk FOREIGN KEY (last_discovery_id) REFERENCES public.discovery(id) ON DELETE SET NULL;


--
-- Name: ip_addresses ip_addresses_site_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.ip_addresses
    ADD CONSTRAINT ip_addresses_site_id_fkey FOREIGN KEY (site_id) REFERENCES public.sites(id) ON DELETE CASCADE;


--
-- Name: ip_addresses ip_addresses_snapshot_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.ip_addresses
    ADD CONSTRAINT ip_addresses_snapshot_id_fkey FOREIGN KEY (snapshot_id) REFERENCES public.snapshots(id) ON DELETE CASCADE;


--
-- Name: ip_addresses ip_addresses_subnet_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.ip_addresses
    ADD CONSTRAINT ip_addresses_subnet_id_fkey FOREIGN KEY (subnet_id) REFERENCES public.subnets(id) ON DELETE CASCADE;


--
-- Name: license_instances license_instances_organization_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.license_instances
    ADD CONSTRAINT license_instances_organization_id_fkey FOREIGN KEY (organization_id) REFERENCES public.organizations(id) ON DELETE CASCADE;


--
-- Name: sites organization_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.sites
    ADD CONSTRAINT organization_id_fkey FOREIGN KEY (organization_id) REFERENCES public.organizations(id) ON DELETE CASCADE;


--
-- Name: ports ports_first_discovery_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.ports
    ADD CONSTRAINT ports_first_discovery_fk FOREIGN KEY (first_discovery_id) REFERENCES public.discovery(id) ON DELETE SET NULL;


--
-- Name: ports ports_host_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.ports
    ADD CONSTRAINT ports_host_id_fkey FOREIGN KEY (host_id) REFERENCES public.hosts(id) ON DELETE CASCADE;


--
-- Name: ports ports_last_discovery_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.ports
    ADD CONSTRAINT ports_last_discovery_fk FOREIGN KEY (last_discovery_id) REFERENCES public.discovery(id) ON DELETE SET NULL;


--
-- Name: ports ports_site_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.ports
    ADD CONSTRAINT ports_site_id_fkey FOREIGN KEY (site_id) REFERENCES public.sites(id) ON DELETE CASCADE;


--
-- Name: ports ports_snapshot_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.ports
    ADD CONSTRAINT ports_snapshot_id_fkey FOREIGN KEY (snapshot_id) REFERENCES public.snapshots(id) ON DELETE CASCADE;


--
-- Name: services services_first_discovery_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.services
    ADD CONSTRAINT services_first_discovery_fk FOREIGN KEY (first_discovery_id) REFERENCES public.discovery(id) ON DELETE SET NULL;


--
-- Name: services services_host_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.services
    ADD CONSTRAINT services_host_id_fkey FOREIGN KEY (host_id) REFERENCES public.hosts(id) ON DELETE CASCADE;


--
-- Name: services services_last_discovery_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.services
    ADD CONSTRAINT services_last_discovery_fk FOREIGN KEY (last_discovery_id) REFERENCES public.discovery(id) ON DELETE SET NULL;


--
-- Name: services services_site_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.services
    ADD CONSTRAINT services_site_id_fkey FOREIGN KEY (site_id) REFERENCES public.sites(id) ON DELETE CASCADE;


--
-- Name: services services_snapshot_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.services
    ADD CONSTRAINT services_snapshot_id_fkey FOREIGN KEY (snapshot_id) REFERENCES public.snapshots(id) ON DELETE CASCADE;


--
-- Name: services services_virtualization_service_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.services
    ADD CONSTRAINT services_virtualization_service_id_fkey FOREIGN KEY (virtualization_service_id) REFERENCES public.services(id) ON DELETE SET NULL;


--
-- Name: shares shares_created_by_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.shares
    ADD CONSTRAINT shares_created_by_fkey FOREIGN KEY (created_by) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: shares shares_site_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.shares
    ADD CONSTRAINT shares_site_id_fkey FOREIGN KEY (site_id) REFERENCES public.sites(id) ON DELETE CASCADE;


--
-- Name: shares shares_topology_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.shares
    ADD CONSTRAINT shares_topology_id_fkey FOREIGN KEY (topology_id) REFERENCES public.topologies(id) ON DELETE CASCADE;


--
-- Name: site_credentials site_credentials_credential_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.site_credentials
    ADD CONSTRAINT site_credentials_credential_id_fkey FOREIGN KEY (credential_id) REFERENCES public.credentials(id) ON DELETE CASCADE;


--
-- Name: site_credentials site_credentials_site_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.site_credentials
    ADD CONSTRAINT site_credentials_site_id_fkey FOREIGN KEY (site_id) REFERENCES public.sites(id) ON DELETE CASCADE;


--
-- Name: snapshots snapshots_created_by_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.snapshots
    ADD CONSTRAINT snapshots_created_by_user_id_fkey FOREIGN KEY (created_by_user_id) REFERENCES public.users(id) ON DELETE SET NULL;


--
-- Name: snapshots snapshots_site_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.snapshots
    ADD CONSTRAINT snapshots_site_id_fkey FOREIGN KEY (site_id) REFERENCES public.sites(id) ON DELETE CASCADE;


--
-- Name: subnet_vlans subnet_vlans_snapshot_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.subnet_vlans
    ADD CONSTRAINT subnet_vlans_snapshot_id_fkey FOREIGN KEY (snapshot_id) REFERENCES public.snapshots(id) ON DELETE CASCADE;


--
-- Name: subnet_vlans subnet_vlans_subnet_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.subnet_vlans
    ADD CONSTRAINT subnet_vlans_subnet_id_fkey FOREIGN KEY (subnet_id) REFERENCES public.subnets(id) ON DELETE CASCADE;


--
-- Name: subnet_vlans subnet_vlans_vlan_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.subnet_vlans
    ADD CONSTRAINT subnet_vlans_vlan_id_fkey FOREIGN KEY (vlan_id) REFERENCES public.vlans(id) ON DELETE CASCADE;


--
-- Name: subnets subnets_first_discovery_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.subnets
    ADD CONSTRAINT subnets_first_discovery_fk FOREIGN KEY (first_discovery_id) REFERENCES public.discovery(id) ON DELETE SET NULL;


--
-- Name: subnets subnets_last_discovery_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.subnets
    ADD CONSTRAINT subnets_last_discovery_fk FOREIGN KEY (last_discovery_id) REFERENCES public.discovery(id) ON DELETE SET NULL;


--
-- Name: subnets subnets_site_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.subnets
    ADD CONSTRAINT subnets_site_id_fkey FOREIGN KEY (site_id) REFERENCES public.sites(id) ON DELETE CASCADE;


--
-- Name: subnets subnets_snapshot_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.subnets
    ADD CONSTRAINT subnets_snapshot_id_fkey FOREIGN KEY (snapshot_id) REFERENCES public.snapshots(id) ON DELETE CASCADE;


--
-- Name: subnets subnets_virtualization_service_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.subnets
    ADD CONSTRAINT subnets_virtualization_service_id_fkey FOREIGN KEY (virtualization_service_id) REFERENCES public.services(id) ON DELETE SET NULL;


--
-- Name: tag_groups tag_groups_organization_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.tag_groups
    ADD CONSTRAINT tag_groups_organization_id_fkey FOREIGN KEY (organization_id) REFERENCES public.organizations(id) ON DELETE CASCADE;


--
-- Name: tags tags_organization_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.tags
    ADD CONSTRAINT tags_organization_id_fkey FOREIGN KEY (organization_id) REFERENCES public.organizations(id) ON DELETE CASCADE;


--
-- Name: tags tags_snapshot_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.tags
    ADD CONSTRAINT tags_snapshot_id_fkey FOREIGN KEY (snapshot_id) REFERENCES public.snapshots(id) ON DELETE CASCADE;


--
-- Name: tags tags_tag_group_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.tags
    ADD CONSTRAINT tags_tag_group_id_fkey FOREIGN KEY (tag_group_id) REFERENCES public.tag_groups(id) ON DELETE SET NULL;


--
-- Name: topologies topologies_site_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.topologies
    ADD CONSTRAINT topologies_site_id_fkey FOREIGN KEY (site_id) REFERENCES public.sites(id) ON DELETE CASCADE;


--
-- Name: user_api_key_site_access user_api_key_site_access_api_key_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_api_key_site_access
    ADD CONSTRAINT user_api_key_site_access_api_key_id_fkey FOREIGN KEY (api_key_id) REFERENCES public.user_api_keys(id) ON DELETE CASCADE;


--
-- Name: user_api_key_site_access user_api_key_site_access_site_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_api_key_site_access
    ADD CONSTRAINT user_api_key_site_access_site_id_fkey FOREIGN KEY (site_id) REFERENCES public.sites(id) ON DELETE CASCADE;


--
-- Name: user_api_keys user_api_keys_organization_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_api_keys
    ADD CONSTRAINT user_api_keys_organization_id_fkey FOREIGN KEY (organization_id) REFERENCES public.organizations(id) ON DELETE CASCADE;


--
-- Name: user_api_keys user_api_keys_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_api_keys
    ADD CONSTRAINT user_api_keys_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: user_site_access user_site_access_site_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_site_access
    ADD CONSTRAINT user_site_access_site_id_fkey FOREIGN KEY (site_id) REFERENCES public.sites(id) ON DELETE CASCADE;


--
-- Name: user_site_access user_site_access_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_site_access
    ADD CONSTRAINT user_site_access_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: users users_organization_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_organization_id_fkey FOREIGN KEY (organization_id) REFERENCES public.organizations(id) ON DELETE CASCADE;


--
-- Name: vlans vlans_first_discovery_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.vlans
    ADD CONSTRAINT vlans_first_discovery_fk FOREIGN KEY (first_discovery_id) REFERENCES public.discovery(id) ON DELETE SET NULL;


--
-- Name: vlans vlans_last_discovery_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.vlans
    ADD CONSTRAINT vlans_last_discovery_fk FOREIGN KEY (last_discovery_id) REFERENCES public.discovery(id) ON DELETE SET NULL;


--
-- Name: vlans vlans_organization_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.vlans
    ADD CONSTRAINT vlans_organization_id_fkey FOREIGN KEY (organization_id) REFERENCES public.organizations(id) ON DELETE CASCADE;


--
-- Name: vlans vlans_site_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.vlans
    ADD CONSTRAINT vlans_site_id_fkey FOREIGN KEY (site_id) REFERENCES public.sites(id) ON DELETE CASCADE;


--
-- Name: vlans vlans_snapshot_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.vlans
    ADD CONSTRAINT vlans_snapshot_id_fkey FOREIGN KEY (snapshot_id) REFERENCES public.snapshots(id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

\unrestrict 8nJiNRZbRQoO2iZFcKZ0YLbyYEz6pSrJxxjWUWMvJQoeN5k0NaaYiH834yma8QG

