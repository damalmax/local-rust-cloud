use derive_builder::Builder;

#[derive(Debug, Builder)]
pub(crate) struct InsertRole {
    pub(crate) id: Option<i64>,
    pub(crate) account_id: i64,
    pub(crate) role_id: String,
    pub(crate) role_name: String,
    pub(crate) description: Option<String>,
    pub(crate) arn: String,
    pub(crate) path: String,
    pub(crate) policy_id: Option<i64>,
    pub(crate) max_session_duration: i64,
    pub(crate) create_date: i64,
}
