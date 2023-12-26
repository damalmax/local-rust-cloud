use sqlx::FromRow;

#[derive(Clone, FromRow, Debug)]
pub struct Region {
    pub id: i64,
    pub name: String,
}
