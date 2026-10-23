//! Credential junction table types.
//!
//! Models the `site_credentials` and `host_credentials` junction tables as
//! `ChildStorableEntity` rows, stored through `GenericChildStorage`.

use anyhow::Result;
use sqlx::{Row, postgres::PgRow};
use std::fmt::Display;
use uuid::Uuid;

use crate::server::shared::storage::{
    child::ChildStorableEntity,
    traits::{SqlValue, Storable},
};

// =============================================================================
// SiteCredential (Junction Table)
// =============================================================================

/// A junction record linking a site to a credential.
#[derive(Debug, Clone, Default)]
pub struct SiteCredential {
    pub site_id: Uuid,
    pub credential_id: Uuid,
}

impl Display for SiteCredential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SiteCredential(site={}, credential={})",
            self.site_id, self.credential_id
        )
    }
}

impl Storable for SiteCredential {
    const HAS_SCD2: bool = false;
    type BaseData = (Uuid, Uuid);

    fn table_name() -> &'static str {
        "site_credentials"
    }

    fn new(base: Self::BaseData) -> Self {
        Self {
            site_id: base.0,
            credential_id: base.1,
        }
    }

    fn get_base(&self) -> Self::BaseData {
        (self.site_id, self.credential_id)
    }

    fn to_params(&self) -> Result<(Vec<&'static str>, Vec<SqlValue>)> {
        Ok((
            vec!["site_id", "credential_id"],
            vec![
                SqlValue::Uuid(self.site_id),
                SqlValue::Uuid(self.credential_id),
            ],
        ))
    }

    fn from_row(row: &PgRow) -> Result<Self> {
        Ok(Self {
            site_id: row.get("site_id"),
            credential_id: row.get("credential_id"),
        })
    }
}

// =============================================================================
// HostCredential (Junction Table)
// =============================================================================

/// A junction record linking a host to a credential, optionally scoped to ip_addresses.
#[derive(Debug, Clone, Default)]
pub struct HostCredential {
    pub host_id: Uuid,
    pub credential_id: Uuid,
    pub ip_address_ids: Option<Vec<Uuid>>,
}

impl Display for HostCredential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HostCredential(host={}, credential={})",
            self.host_id, self.credential_id
        )
    }
}

impl Storable for HostCredential {
    const HAS_SCD2: bool = false;
    type BaseData = (Uuid, Uuid, Option<Vec<Uuid>>);

    fn table_name() -> &'static str {
        "host_credentials"
    }

    fn new(base: Self::BaseData) -> Self {
        Self {
            host_id: base.0,
            credential_id: base.1,
            ip_address_ids: base.2,
        }
    }

    fn get_base(&self) -> Self::BaseData {
        (
            self.host_id,
            self.credential_id,
            self.ip_address_ids.clone(),
        )
    }

    fn to_params(&self) -> Result<(Vec<&'static str>, Vec<SqlValue>)> {
        Ok((
            vec!["host_id", "credential_id", "ip_address_ids"],
            vec![
                SqlValue::Uuid(self.host_id),
                SqlValue::Uuid(self.credential_id),
                SqlValue::OptionalUuidVec(self.ip_address_ids.clone()),
            ],
        ))
    }

    fn from_row(row: &PgRow) -> Result<Self> {
        Ok(Self {
            host_id: row.get("host_id"),
            credential_id: row.get("credential_id"),
            ip_address_ids: row.get("ip_address_ids"),
        })
    }
}

// Both tables are written from either end: the site's (or host's) credentials, and a
// credential's sites (or hosts). The declared parent is the site or host; the credential side
// replaces its rows through `GenericChildStorage::replace_matching`.

impl ChildStorableEntity for SiteCredential {
    fn parent_column() -> &'static str {
        "site_id"
    }

    fn parent_id(&self) -> Uuid {
        self.site_id
    }
}

impl ChildStorableEntity for HostCredential {
    fn parent_column() -> &'static str {
        "host_id"
    }

    fn parent_id(&self) -> Uuid {
        self.host_id
    }
}
