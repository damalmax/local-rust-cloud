use aws_sdk_iam::types::PolicyVersion;
use aws_smithy_types::DateTime;
use derive_builder::Builder;
use sqlx::FromRow;

use crate::http::aws::iam::db::types::common::Pageable;

#[derive(Debug, Builder)]
pub(crate) struct InsertPolicyVersion {
    pub(crate) id: Option<i64>,
    pub(crate) account_id: i64,
    pub(crate) policy_id: i64,
    pub(crate) policy_version_id: String,
    pub(crate) policy_document: String,
    pub(crate) create_date: i64,
    pub(crate) version: Option<i16>,
    pub(crate) is_default: bool,
}

#[derive(Debug, FromRow)]
pub(crate) struct SelectPolicyVersion {
    pub(crate) id: i64,
    pub(crate) account_id: i64,
    pub(crate) policy_id: i64,
    pub(crate) policy_version_id: String,
    pub(crate) policy_document: String,
    pub(crate) create_date: i64,
    pub(crate) version: i16,
    pub(crate) is_default: bool,
}

impl Into<PolicyVersion> for &SelectPolicyVersion {
    fn into(self) -> PolicyVersion {
        PolicyVersion::builder()
            .version_id(format!("v{}", self.version))
            .document(&self.policy_document)
            .is_default_version(self.is_default)
            .create_date(DateTime::from_secs(self.create_date))
            .build()
    }
}

#[derive(Debug)]
pub(crate) struct ListPolicyVersionsQuery {
    pub(crate) policy_id: i64,
    pub(crate) limit: i32,
    pub(crate) skip: i32,
}

impl Pageable for &ListPolicyVersionsQuery {
    fn limit(&self) -> i32 {
        self.limit
    }

    fn skip(&self) -> i32 {
        self.skip
    }
}
