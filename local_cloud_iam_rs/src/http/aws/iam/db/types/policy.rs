use derive_builder::Builder;

use crate::http::aws::iam::db::types::policy_type::PolicyType;

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

#[derive(Debug)]
pub(crate) struct ReadPolicy {
    pub(crate) id: i64,
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
