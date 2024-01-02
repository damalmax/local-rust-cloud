use derive_builder::Builder;

#[derive(Debug, Builder)]
pub(crate) struct InsertPolicyVersion {
    pub(crate) id: Option<i64>,
    pub(crate) account_id: i64,
    pub(crate) policy_id: i64,
    pub(crate) policy_version_id: String,
    pub(crate) policy_document: String,
    pub(crate) create_date: i64,
    pub(crate) version: i16,
    pub(crate) is_default: bool,
}
