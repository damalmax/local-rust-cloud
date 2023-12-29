use sqlx::FromRow;

#[derive(Clone, FromRow, Debug)]
pub struct DbStsRole {
    pub arn: String,
}
