use sqlx::FromRow;

pub(crate) struct InsertSamlProvider {
    pub(crate) id: Option<i64>,
    pub(crate) account_id: i64,
    pub(crate) name: String,
    pub(crate) arn: String,
    pub(crate) create_date: i64,
    pub(crate) valid_until: Option<i64>,
    pub(crate) metadata_document: String,
}

#[derive(Debug, FromRow)]
pub(crate) struct SelectSamlProvider {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) arn: String,
    pub(crate) create_date: i64,
    pub(crate) valid_until: Option<i64>,
    pub(crate) metadata_document: String,
}
