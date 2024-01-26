use derive_builder::Builder;

#[derive(Debug, Builder)]
pub(crate) struct InsertGroup {
    pub(crate) id: Option<i64>,
    pub(crate) account_id: i64,
    pub(crate) group_id: String,
    pub(crate) group_name: String,
    pub(crate) arn: String,
    pub(crate) path: String,
    pub(crate) create_date: i64,
}
