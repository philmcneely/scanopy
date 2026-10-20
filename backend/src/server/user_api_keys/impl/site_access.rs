use std::fmt::Display;

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use sqlx::postgres::PgRow;
use uuid::Uuid;

use crate::server::shared::storage::{
    child::ChildStorableEntity,
    traits::{SqlValue, Storable},
};

/// The base data for a UserApiKeySiteAccess junction record
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserApiKeySiteAccessBase {
    pub api_key_id: Uuid,
    pub site_id: Uuid,
}

impl UserApiKeySiteAccessBase {
    pub fn new(api_key_id: Uuid, site_id: Uuid) -> Self {
        Self {
            api_key_id,
            site_id,
        }
    }
}

/// A junction record linking a user API key to a site it has access to
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserApiKeySiteAccess {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub base: UserApiKeySiteAccessBase,
}

impl UserApiKeySiteAccess {
    pub fn new(base: UserApiKeySiteAccessBase) -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            base,
        }
    }

    pub fn api_key_id(&self) -> Uuid {
        self.base.api_key_id
    }

    pub fn site_id(&self) -> Uuid {
        self.base.site_id
    }
}

impl Display for UserApiKeySiteAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UserApiKeySiteAccess(api_key={}, site={})",
            self.base.api_key_id, self.base.site_id
        )
    }
}

impl Storable for UserApiKeySiteAccess {
    const HAS_SCD2: bool = false;
    type BaseData = UserApiKeySiteAccessBase;

    fn table_name() -> &'static str {
        "user_api_key_site_access"
    }

    fn new(base: Self::BaseData) -> Self {
        UserApiKeySiteAccess::new(base)
    }

    fn get_base(&self) -> Self::BaseData {
        self.base.clone()
    }

    fn to_params(&self) -> Result<(Vec<&'static str>, Vec<SqlValue>)> {
        Ok((
            vec!["id", "api_key_id", "site_id", "created_at"],
            vec![
                SqlValue::Uuid(self.id),
                SqlValue::Uuid(self.base.api_key_id),
                SqlValue::Uuid(self.base.site_id),
                SqlValue::Timestamp(self.created_at),
            ],
        ))
    }

    fn from_row(row: &PgRow) -> Result<Self> {
        Ok(UserApiKeySiteAccess {
            id: row.get("id"),
            created_at: row.get("created_at"),
            base: UserApiKeySiteAccessBase {
                api_key_id: row.get("api_key_id"),
                site_id: row.get("site_id"),
            },
        })
    }
}

impl ChildStorableEntity for UserApiKeySiteAccess {
    fn parent_column() -> &'static str {
        "api_key_id"
    }

    fn parent_id(&self) -> Uuid {
        self.base.api_key_id
    }
}
