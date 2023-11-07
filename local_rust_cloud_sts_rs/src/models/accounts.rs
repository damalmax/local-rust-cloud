use sqlx::FromRow;

#[derive(Clone, FromRow, Debug)]
pub struct Account {
    pub id: i64,
    pub account_id: i64,
    pub alias: String,
}
