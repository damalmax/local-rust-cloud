use aws_sdk_iam::types::{AttachedPermissionsBoundary, PermissionsBoundaryAttachmentType, User};
use aws_smithy_types::DateTime;
use derive_builder::Builder;
use sqlx::sqlite::SqliteRow;
use sqlx::{Error, FromRow, Row};

use crate::http::aws::iam::db::types::common::Pageable;
use crate::http::aws::iam::db::types::tag::DbTag;
use crate::http::aws::iam::operations;

#[derive(Debug, Builder)]
pub(crate) struct InsertUser {
    pub(crate) id: Option<i64>,
    pub(crate) account_id: i64,
    pub(crate) username: String,
    pub(crate) arn: String,
    pub(crate) path: String,
    pub(crate) user_id: String,
    pub(crate) policy_id: Option<i64>,
    pub(crate) create_date: i64,
}

#[derive(Debug, Clone)]
pub(crate) struct SelectUser {
    pub(crate) id: i64,
    pub(crate) account_id: i64,
    pub(crate) username: String,
    pub(crate) arn: String,
    pub(crate) path: String,
    pub(crate) user_id: String,
    pub(crate) policy_id: Option<i64>,
    pub(crate) policy_arn: Option<String>,
    pub(crate) create_date: i64,
    pub(crate) tags: Option<Vec<DbTag>>,
}

impl<'r> FromRow<'r, SqliteRow> for SelectUser {
    fn from_row(row: &'r SqliteRow) -> Result<Self, Error> {
        let id: i64 = row.try_get("id")?;
        let account_id: i64 = row.try_get("account_id")?;
        let arn: String = row.try_get("arn")?;
        let user_id: String = row.try_get("user_id")?;
        let path: String = row.try_get("path")?;
        let create_date: i64 = row.try_get("create_date")?;
        let username: String = row.try_get("username")?;
        let policy_id: Option<i64> = row.try_get("policy_id")?;
        let policy_arn: Option<String> = row.try_get("policy_arn")?;
        let raw_tags: Option<String> = row.try_get("tags")?;
        let tags = match raw_tags {
            None => None,
            Some(raw) => Some(super::tag::parse_tags_from_raw(&raw)?),
        };
        Ok(SelectUser {
            id,
            account_id,
            username,
            arn,
            path,
            user_id,
            policy_id,
            policy_arn,
            create_date,
            tags,
        })
    }
}

#[derive(Debug)]
pub(crate) struct ListUsersByGroupQuery {
    pub(crate) group_id: i64,
    pub(crate) limit: i32,
    pub(crate) skip: i32,
}

impl Pageable for &ListUsersByGroupQuery {
    fn limit(&self) -> i32 {
        self.limit
    }

    fn skip(&self) -> i32 {
        self.skip
    }
}

impl Into<User> for SelectUser {
    fn into(self) -> User {
        let tags = match &self.tags {
            None => None,
            Some(tags) => operations::common::prepare_tags_for_output(tags),
        };

        let permissions_boundary = match &self.policy_arn {
            None => None,
            Some(arn) => Some(
                AttachedPermissionsBoundary::builder()
                    .permissions_boundary_type(PermissionsBoundaryAttachmentType::Policy)
                    .permissions_boundary_arn(arn)
                    .build(),
            ),
        };

        User::builder()
            .path(&self.path)
            .user_name(&self.username)
            .user_id(&self.user_id)
            .create_date(DateTime::from_secs(self.create_date))
            .arn(&self.arn)
            .set_permissions_boundary(permissions_boundary)
            .set_tags(tags)
            .build()
            .unwrap()
    }
}
