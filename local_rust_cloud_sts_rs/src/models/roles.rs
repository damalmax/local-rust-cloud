use sqlx::FromRow;

#[derive(Clone, FromRow, Debug)]
pub struct StsRole {
    pub arn: String,
}
