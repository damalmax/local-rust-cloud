use aws_sdk_iam::types::{AttachedPermissionsBoundary, PermissionsBoundaryAttachmentType, Role, RoleLastUsed};
use aws_smithy_types::DateTime;
use derive_builder::Builder;
use sqlx::sqlite::SqliteRow;
use sqlx::{Error, FromRow, Row};

use crate::http::aws::iam::db::types::common::Pageable;
use crate::http::aws::iam::types::list_roles::ListRolesRequest;

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
    pub(crate) max_session_duration: i32,
    pub(crate) assume_role_policy_document: String,
    pub(crate) arn: String,
    pub(crate) path: String,
    pub(crate) role_id: String,
    pub(crate) policy_id: Option<i64>,
    pub(crate) create_date: i64,
}

impl From<&SelectRole> for Role {
    fn from(value: &SelectRole) -> Self {
        Role::builder()
            .role_id(&value.role_id)
            .assume_role_policy_document(&value.assume_role_policy_document)
            .role_name(&value.role_name)
            .path(&value.path)
            .arn(&value.arn)
            .max_session_duration(value.max_session_duration)
            .set_description(value.description.as_ref().map(|s| s.to_owned()))
            .create_date(DateTime::from_secs(value.create_date))
            .build()
            .unwrap()
    }
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
        let max_session_duration: i32 = row.try_get("max_session_duration")?;
        let assume_role_policy_document: String = row.try_get("assume_role_policy_document")?;
        let create_date: i64 = row.try_get("create_date")?;
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
        })
    }
}

#[derive(Debug)]
pub(crate) struct SelectRoleWithDetails {
    pub(crate) id: i64,
    pub(crate) account_id: i64,
    pub(crate) role_name: String,
    pub(crate) description: Option<String>,
    pub(crate) max_session_duration: i32,
    pub(crate) assume_role_policy_document: String,
    pub(crate) arn: String,
    pub(crate) path: String,
    pub(crate) role_id: String,
    pub(crate) policy_id: Option<i64>,
    pub(crate) policy_arn: Option<String>,
    pub(crate) create_date: i64,
    pub(crate) last_used_date: Option<i64>,
    pub(crate) last_used_region_id: Option<i64>,
    pub(crate) last_used_region: Option<String>,
}

impl From<&SelectRoleWithDetails> for Role {
    fn from(value: &SelectRoleWithDetails) -> Self {
        let role_last_used = match value.last_used_date {
            None => None,
            Some(last_used_date) => Some(
                RoleLastUsed::builder()
                    .last_used_date(DateTime::from_secs(last_used_date))
                    .set_region(value.last_used_region.clone())
                    .build(),
            ),
        };
        let permissions_boundary = match &value.policy_arn {
            None => None,
            Some(policy_arn) => Some(
                AttachedPermissionsBoundary::builder()
                    .permissions_boundary_type(PermissionsBoundaryAttachmentType::Policy)
                    .permissions_boundary_arn(policy_arn)
                    .build(),
            ),
        };
        Role::builder()
            .role_id(&value.role_id)
            .assume_role_policy_document(&value.assume_role_policy_document)
            .role_name(&value.role_name)
            .arn(&value.arn)
            .path(&value.path)
            .set_permissions_boundary(permissions_boundary)
            .set_role_last_used(role_last_used)
            .max_session_duration(value.max_session_duration)
            .set_description(value.description.as_ref().map(|s| s.to_owned()))
            .create_date(DateTime::from_secs(value.create_date))
            .build()
            .unwrap()
    }
}

impl<'r> FromRow<'r, SqliteRow> for SelectRoleWithDetails {
    fn from_row(row: &'r SqliteRow) -> Result<Self, Error> {
        let id: i64 = row.try_get("id")?;
        let account_id: i64 = row.try_get("account_id")?;
        let arn: String = row.try_get("arn")?;
        let role_name: String = row.try_get("role_name")?;
        let role_id: String = row.try_get("role_id")?;
        let policy_id: Option<i64> = row.try_get("policy_id")?;
        let policy_arn: Option<String> = row.try_get("policy_arn")?;
        let description: Option<String> = row.try_get("description")?;
        let path: String = row.try_get("path")?;
        let max_session_duration: i32 = row.try_get("max_session_duration")?;
        let assume_role_policy_document: String = row.try_get("assume_role_policy_document")?;
        let create_date: i64 = row.try_get("create_date")?;
        let last_used_date: Option<i64> = row.try_get("last_used_date")?;
        let last_used_region_id: Option<i64> = row.try_get("last_used_region_id")?;
        let last_used_region: Option<String> = row.try_get("last_used_region")?;
        Ok(SelectRoleWithDetails {
            id,
            account_id,
            role_name,
            description,
            max_session_duration,
            assume_role_policy_document,
            arn,
            path,
            role_id,
            policy_id,
            policy_arn,
            create_date,
            last_used_date,
            last_used_region_id,
            last_used_region,
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
