use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Row, postgres::PgRow};
use std::fmt::Display;
use uuid::Uuid;

use crate::server::shared::storage::{
    child::ChildStorableEntity,
    traits::{SqlValue, Storable},
};

/// The base data for a UserSiteAccess junction record
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserSiteAccessBase {
    pub user_id: Uuid,
    pub site_id: Uuid,
}

impl UserSiteAccessBase {
    pub fn new(user_id: Uuid, site_id: Uuid) -> Self {
        Self { user_id, site_id }
    }
}

/// A junction record linking a user to a site they have access to
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserSiteAccess {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub base: UserSiteAccessBase,
}

impl UserSiteAccess {
    pub fn new(base: UserSiteAccessBase) -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            base,
        }
    }

    pub fn user_id(&self) -> Uuid {
        self.base.user_id
    }

    pub fn site_id(&self) -> Uuid {
        self.base.site_id
    }
}

impl Display for UserSiteAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UserSiteAccess(user={}, site={})",
            self.base.user_id, self.base.site_id
        )
    }
}

impl Storable for UserSiteAccess {
    const HAS_SCD2: bool = false;
    type BaseData = UserSiteAccessBase;

    fn table_name() -> &'static str {
        "user_site_access"
    }

    fn new(base: Self::BaseData) -> Self {
        UserSiteAccess::new(base)
    }

    fn get_base(&self) -> Self::BaseData {
        self.base.clone()
    }

    fn to_params(&self) -> Result<(Vec<&'static str>, Vec<SqlValue>)> {
        Ok((
            vec!["id", "user_id", "site_id", "created_at"],
            vec![
                SqlValue::Uuid(self.id),
                SqlValue::Uuid(self.base.user_id),
                SqlValue::Uuid(self.base.site_id),
                SqlValue::Timestamp(self.created_at),
            ],
        ))
    }

    fn from_row(row: &PgRow) -> Result<Self> {
        Ok(UserSiteAccess {
            id: row.get("id"),
            created_at: row.get("created_at"),
            base: UserSiteAccessBase {
                user_id: row.get("user_id"),
                site_id: row.get("site_id"),
            },
        })
    }
}

impl ChildStorableEntity for UserSiteAccess {
    fn parent_column() -> &'static str {
        "user_id"
    }

    fn parent_id(&self) -> Uuid {
        self.base.user_id
    }
}
