use aws_sdk_iam::types::Policy;
use aws_smithy_types::DateTime;
use derive_builder::Builder;
use sqlx::FromRow;

use crate::http::aws::iam::db::types::policy_tag::DbPolicyTag;
use crate::http::aws::iam::db::types::policy_type::PolicyType;
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

#[derive(Debug, FromRow, Clone)]
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
}

#[derive(Debug)]
pub(crate) struct SelectPolicyWithTags {
    pub(crate) policy: SelectPolicy,
    pub(crate) tags: Vec<DbPolicyTag>,
}

impl Into<Policy> for &SelectPolicyWithTags {
    fn into(self) -> Policy {
        let policy = &self.policy;
        let tags = if self.tags.len() > 0 {
            Some(self.tags.iter().map(|tag| tag.into()).collect())
        } else {
            None
        };
        let description = match &policy.description {
            None => None,
            Some(desc) => Some(desc.to_owned()),
        };

        Policy::builder()
            .policy_name(&policy.policy_name)
            .policy_id(&policy.policy_id)
            .arn(&policy.arn)
            .set_description(description)
            .path(&policy.path)
            .default_version_id(format!("v{}", &policy.version))
            .attachment_count(0) // TODO: Populate value from DB
            .permissions_boundary_usage_count(0) // TODO: Populate value from DB
            .is_attachable(policy.is_attachable)
            .create_date(DateTime::from_secs(policy.create_date))
            .update_date(DateTime::from_secs(policy.update_date))
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
