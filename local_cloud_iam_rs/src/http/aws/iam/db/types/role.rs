use derive_builder::Builder;
use sqlx::sqlite::SqliteRow;
use sqlx::{Error, FromRow, Row};

use crate::http::aws::iam::db::types::common::Pageable;
use crate::http::aws::iam::db::types::tags::DbTag;
use crate::http::aws::iam::types::list_roles_request::ListRolesRequest;

#[derive(Debug, Builder)]
pub(crate) struct InsertRole {
    pub(crate) id: Option<i64>,
    pub(crate) account_id: i64,
    pub(crate) role_id: String,
    pub(crate) role_name: String,
    pub(crate) description: Option<String>,
    pub(crate) assume_role_policy_document: String,
    pub(crate) arn: String,
    pub(crate) path: String,
    pub(crate) policy_id: Option<i64>,
    pub(crate) max_session_duration: i64,
    pub(crate) create_date: i64,
}

#[derive(Debug)]
pub(crate) struct SelectRole {
    pub(crate) id: i64,
    pub(crate) account_id: i64,
    pub(crate) role_name: String,
    pub(crate) description: Option<String>,
    pub(crate) max_session_duration: i64,
    pub(crate) assume_role_policy_document: String,
    pub(crate) arn: String,
    pub(crate) path: String,
    pub(crate) role_id: String,
    pub(crate) policy_id: Option<i64>,
    pub(crate) create_date: i64,
    pub(crate) tags: Option<Vec<DbTag>>,
    pub(crate) last_used_date: Option<i64>,
    pub(crate) last_used_region_id: Option<i64>,
}

impl<'r> FromRow<'r, SqliteRow> for SelectRole {
    fn from_row(row: &'r SqliteRow) -> Result<Self, Error> {
        let id: i64 = row.try_get("id")?;
        let account_id: i64 = row.try_get("account_id")?;
        let arn: String = row.try_get("arn")?;
        let role_name: String = row.try_get("role_name")?;
        let role_id: String = row.try_get("role_id")?;
        let description: Option<String> = row.try_get("description")?;
        let path: String = row.try_get("path")?;
        let max_session_duration: i64 = row.try_get("max_session_duration")?;
        let assume_role_policy_document: String = row.try_get("assume_role_policy_document")?;
        let create_date: i64 = row.try_get("create_date")?;
        let last_used_date: Option<i64> = row.try_get("last_used_date")?;
        let tags = super::tags::from_row(&row, "tags")?;
        Ok(SelectRole {
            id,
            account_id,
            role_name,
            description,
            max_session_duration,
            assume_role_policy_document,
            arn,
            path,
            role_id,
            policy_id: None,
            create_date,
            tags,
            last_used_date,
            last_used_region_id: None,
        })
    }
}

#[derive(Debug)]
pub(crate) struct ListRolesQuery {
    pub(crate) path_prefix: String,
    pub(crate) limit: i32,
    pub(crate) skip: i32,
}

impl Pageable for &ListRolesQuery {
    fn limit(&self) -> i32 {
        self.limit
    }

    fn skip(&self) -> i32 {
        self.skip
    }
}

impl Into<ListRolesQuery> for &ListRolesRequest {
    fn into(self) -> ListRolesQuery {
        let limit = match self.max_items() {
            None => 10,
            Some(v) => *v,
        };

        let skip = match self.marker_type() {
            None => 0,
            // unwrap is safe since marker must be validated before DB query preparation
            Some(marker_type) => marker_type.marker().unwrap().truncate_amount,
        };

        ListRolesQuery {
            path_prefix: self.path_prefix().unwrap_or("/").to_owned(),
            limit: if limit < 1 { 10 } else { limit },
            skip,
        }
    }
}
