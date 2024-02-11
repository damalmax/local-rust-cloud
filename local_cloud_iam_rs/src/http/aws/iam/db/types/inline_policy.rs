use sqlx::FromRow;

#[derive(Clone, FromRow, Debug)]
pub(crate) struct DbInlinePolicy {
    pub(crate) id: Option<i64>,
    pub(crate) parent_id: i64,
    pub(crate) policy_name: String,
    pub(crate) policy_document: String,
}

impl DbInlinePolicy {
    pub(crate) fn new(parent_id: i64, policy_name: &str, policy_document: &str) -> Self {
        DbInlinePolicy {
            id: None,
            parent_id,
            policy_name: policy_name.to_owned(),
            policy_document: policy_document.to_owned(),
        }
    }
}
