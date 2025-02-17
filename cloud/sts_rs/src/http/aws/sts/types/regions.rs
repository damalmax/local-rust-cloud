use sqlx::FromRow;

#[derive(Clone, FromRow, Debug)]
pub struct DbRegion {
    pub id: i64,
    pub name: String,
}
