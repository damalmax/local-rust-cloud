use aws_sdk_iam::types::Policy;
use aws_smithy_types::DateTime;
use derive_builder::Builder;
use sqlx::sqlite::SqliteRow;
use sqlx::{Error, FromRow, Row};

use crate::http::aws::iam::db::types::common::Pageable;
use crate::http::aws::iam::db::types::policy_type::PolicyType;
use crate::http::aws::iam::db::types::tags::DbTag;
use crate::http::aws::iam::types::list_policies::ListPoliciesRequest;
use crate::http::aws::iam::types::policy_scope_type::PolicyScopeType;

#[derive(Debug, Builder)]
pub(crate) struct InsertPolicy {
    pub(crate) id: Option<i64>,
    pub(crate) account_id: i64,
    pub(crate) arn: String,
    pub(crate) policy_id: String,
    pub(crate) path: String,
    pub(crate) create_date: i64,
    pub(crate) update_date: i64,
    pub(crate) policy_name: String,
    pub(crate) policy_type: PolicyType,
    pub(crate) description: Option<String>,
    pub(crate) attachable: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct SelectPolicy {
    pub(crate) id: i64,
    pub(crate) account_id: i64,
    pub(crate) arn: String,
    pub(crate) policy_id: String,
    pub(crate) path: String,
    pub(crate) create_date: i64,
    pub(crate) version: i32,
    pub(crate) update_date: i64,
    pub(crate) policy_name: String,
    pub(crate) policy_type: PolicyType,
    pub(crate) description: Option<String>,
    pub(crate) is_attachable: bool,
    pub(crate) attachment_count: i32,
    pub(crate) permissions_boundary_usage_count: i32,
    pub(crate) tags: Option<Vec<DbTag>>,
}

impl<'r> FromRow<'r, SqliteRow> for SelectPolicy {
    fn from_row(row: &'r SqliteRow) -> Result<Self, Error> {
        let id: i64 = row.try_get("id")?;
        let account_id: i64 = row.try_get("account_id")?;
        let arn: String = row.try_get("arn")?;
        let policy_id: String = row.try_get("policy_id")?;
        let path: String = row.try_get("path")?;
        let create_date: i64 = row.try_get("create_date")?;
        let version: i32 = row.try_get("version")?;
        let update_date: i64 = row.try_get("update_date")?;
        let policy_name: String = row.try_get("policy_name")?;
        let attachment_count: i32 = row.try_get("attachment_count")?;
        let permissions_boundary_usage_count: i32 = row.try_get("permissions_boundary_usage_count")?;
        let policy_type: i32 = row.try_get("policy_type")?;
        let description: Option<String> = row.try_get("description")?;
        let is_attachable: bool = row.try_get("is_attachable")?;
        let tags = super::tags::from_row(&row, "tags")?;

        Ok(SelectPolicy {
            id,
            account_id,
            arn,
            policy_id,
            path,
            create_date,
            version,
            update_date,
            policy_name,
            policy_type: PolicyType::from(policy_type),
            description,
            is_attachable,
            attachment_count,
            permissions_boundary_usage_count,
            tags,
        })
    }
}

impl From<&SelectPolicy> for Policy {
    fn from(value: &SelectPolicy) -> Self {
        let tags = match &value.tags {
            None => None,
            Some(tags) => {
                if tags.is_empty() {
                    None
                } else {
                    Some(tags.iter().map(|tag| tag.into()).collect())
                }
            }
        };
        let description = match &value.description {
            None => None,
            Some(desc) => Some(desc.to_owned()),
        };

        Policy::builder()
            .policy_name(&value.policy_name)
            .policy_id(&value.policy_id)
            .arn(&value.arn)
            .set_description(description)
            .path(&value.path)
            .default_version_id(format!("v{}", &value.version))
            .attachment_count(value.attachment_count)
            .permissions_boundary_usage_count(value.permissions_boundary_usage_count)
            .is_attachable(value.is_attachable)
            .create_date(DateTime::from_secs(value.create_date))
            .update_date(DateTime::from_secs(value.update_date))
            .set_tags(tags)
            .build()
    }
}

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) enum PolicyScope {
    Local,
    All,
    Aws,
}

#[derive(Debug)]
pub(crate) struct ListPoliciesQuery {
    pub(crate) path_prefix: String,
    pub(crate) limit: i32,
    pub(crate) skip: i32,
    pub(crate) is_attached: bool,
    pub(crate) policy_scope_types: Vec<PolicyType>,
}

impl Pageable for &ListPoliciesQuery {
    fn limit(&self) -> i32 {
        self.limit
    }

    fn skip(&self) -> i32 {
        self.skip
    }
}

impl Into<ListPoliciesQuery> for &ListPoliciesRequest {
    fn into(self) -> ListPoliciesQuery {
        let limit = match self.max_items() {
            None => 10,
            Some(v) => *v,
        };

        let skip = match self.marker_type() {
            None => 0,
            // unwrap is safe since marker must be validated before DB query preparation
            Some(marker_type) => marker_type.marker().unwrap().truncate_amount,
        };

        let policy_scopes_types = match &self.scope() {
            None => {
                vec![PolicyType::CustomerManaged, PolicyType::LocalCloudManaged]
            }
            Some(scope_type) => match scope_type {
                PolicyScopeType::Local => vec![PolicyType::CustomerManaged],
                PolicyScopeType::All => vec![PolicyType::CustomerManaged, PolicyType::LocalCloudManaged],
                PolicyScopeType::Aws => vec![PolicyType::LocalCloudManaged],
            },
        };

        ListPoliciesQuery {
            path_prefix: self.path_prefix().unwrap_or("/").to_owned(),
            limit: if limit < 1 { 10 } else { limit },
            skip,
            is_attached: self.only_attached().unwrap_or(false),
            policy_scope_types: policy_scopes_types,
        }
    }
}
