use aws_sdk_iam::types::Policy;
use aws_smithy_types::DateTime;
use derive_builder::Builder;
use sqlx::sqlite::SqliteRow;
use sqlx::{Error, FromRow, Row};

use crate::http::aws::iam::db::types::common::Pageable;
use crate::http::aws::iam::db::types::policy_type::PolicyType;
use crate::http::aws::iam::db::types::tag::DbTag;
use crate::http::aws::iam::types::list_policies_request::ListPoliciesRequest;
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
        let policy_type: i32 = row.try_get("policy_type")?;
        let description: Option<String> = row.try_get("description")?;
        let is_attachable: bool = row.try_get("is_attachable")?;
        let raw_tags: Option<String> = row.try_get("tags")?;
        let tags = match raw_tags {
            None => None,
            Some(raw) => Some(super::tag::parse_tags_from_raw(&raw)?),
        };
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
            tags,
        })
    }
}

impl Into<Policy> for &SelectPolicy {
    fn into(self) -> Policy {
        let tags = match &self.tags {
            None => None,
            Some(tags) => {
                if tags.is_empty() {
                    None
                } else {
                    Some(tags.iter().map(|tag| tag.into()).collect())
                }
            }
        };
        let description = match &self.description {
            None => None,
            Some(desc) => Some(desc.to_owned()),
        };

        Policy::builder()
            .policy_name(&self.policy_name)
            .policy_id(&self.policy_id)
            .arn(&self.arn)
            .set_description(description)
            .path(&self.path)
            .default_version_id(format!("v{}", &self.version))
            .attachment_count(0) // TODO: Populate value from DB
            .permissions_boundary_usage_count(0) // TODO: Populate value from DB
            .is_attachable(self.is_attachable)
            .create_date(DateTime::from_secs(self.create_date))
            .update_date(DateTime::from_secs(self.update_date))
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
