use aws_sdk_iam::operation::create_policy::CreatePolicyInput;
use sqlx::FromRow;

use crate::error::IamError;

#[derive(Clone, FromRow, Debug)]
pub struct Policy {
    pub id: Option<i64>,
    pub account_id: i64,
    pub policy_name: Option<String>,
    pub policy_id: Option<String>,
    pub arn: String,
    pub path: Option<String>,
    pub policy_document: Option<String>,
    pub default_version_id: Option<String>,
    pub attachment_count: Option<i32>,
    pub permissions_boundary_usage_count: Option<i32>,
    pub is_attachable: bool,
    pub description: Option<String>,
    pub create_date: i64,
    pub update_date: i64,
}

impl Policy {
    pub fn builder() -> PolicyBuilder {
        PolicyBuilder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct PolicyBuilder {
    pub id: Option<i64>,
    pub account_id: Option<i64>,
    pub policy_name: Option<String>,
    pub policy_id: Option<String>,
    pub arn: Option<String>,
    pub path: Option<String>,
    pub policy_document: Option<String>,
    pub default_version_id: Option<String>,
    pub attachment_count: Option<i32>,
    pub permissions_boundary_usage_count: Option<i32>,
    pub is_attachable: Option<bool>,
    pub description: Option<String>,
    pub create_date: Option<i64>,
    pub update_date: Option<i64>,
}

impl PolicyBuilder {
    pub fn id(mut self, input: i64) -> Self {
        self.id = Some(input);
        self
    }

    pub fn account_id(mut self, input: i64) -> Self {
        self.account_id = Some(input);
        self
    }

    pub fn policy_name(mut self, input: impl Into<String>) -> Self {
        self.policy_name = Some(input.into());
        self
    }

    pub fn policy_id(mut self, input: impl Into<String>) -> Self {
        self.policy_id = Some(input.into());
        self
    }

    pub fn arn(mut self, input: impl Into<String>) -> Self {
        self.arn = Some(input.into());
        self
    }

    pub fn path(mut self, input: impl Into<String>) -> Self {
        self.path = Some(input.into());
        self
    }

    pub fn policy_document(mut self, input: impl Into<String>) -> Self {
        self.policy_document = Some(input.into());
        self
    }

    pub fn default_version_id(mut self, input: impl Into<String>) -> Self {
        self.default_version_id = Some(input.into());
        self
    }

    pub fn attachment_count(mut self, input: i32) -> Self {
        self.attachment_count = Some(input);
        self
    }

    pub fn permissions_boundary_usage_count(mut self, input: i32) -> Self {
        self.permissions_boundary_usage_count = Some(input);
        self
    }

    pub fn is_attachable(mut self, input: bool) -> Self {
        self.is_attachable = Some(input);
        self
    }

    pub fn description(mut self, input: impl Into<String>) -> Self {
        self.description = Some(input.into());
        self
    }

    pub fn create_date(mut self, input: i64) -> Self {
        self.create_date = Some(input);
        self
    }

    pub fn update_date(mut self, input: i64) -> Self {
        self.update_date = Some(input);
        self
    }

    pub fn from_policy_input(mut self, input: &CreatePolicyInput) -> Self {
        self.policy_name(input.policy_name().unwrap_or(""))
            .path(input.path().unwrap_or("/"))
            // `unwrap` is safe because of request input validator
            .policy_document(input.policy_document().unwrap())
            .description(input.description().unwrap_or(""))
    }

    pub fn build(self) -> Result<Policy, IamError> {
        Result::Ok(Policy {
            id: self.id,
            account_id: self.account_id.expect("account_id is not set"),
            policy_name: self.policy_name,
            policy_id: self.policy_id,
            arn: self.arn.expect("arn is not set"),
            path: self.path,
            policy_document: self.policy_document,
            default_version_id: self.default_version_id,
            attachment_count: self.attachment_count,
            permissions_boundary_usage_count: self.permissions_boundary_usage_count,
            is_attachable: self.is_attachable.expect("is_attachable is not set"),
            description: self.description,
            create_date: self.create_date.expect("create_date is not set"),
            update_date: self.update_date.expect("update_date is not set"),
        })
    }
}
