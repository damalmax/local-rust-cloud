use derive_builder::Builder;

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
