use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Row, Sqlite, Transaction};

use crate::http::aws::iam::db::types::role_tag::DbRoleTag;

pub(crate) async fn save<'a>(tx: &mut Transaction<'a, Sqlite>, tag: &mut DbRoleTag) -> Result<(), Error> {
    let result = sqlx::query(
        r#"INSERT INTO role_tags
                (role_id, key, value)
                VALUES ($1, $2, $3)
                ON CONFLICT(role_id, key) DO UPDATE SET value=$3
                RETURNING id"#,
    )
    .bind(&tag.role_id)
    .bind(&tag.key)
    .bind(&tag.value)
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_one(tx.as_mut())
    .await?;

    tag.id = Some(result);
    Ok(())
}

pub(crate) async fn save_all<'a>(tx: &mut Transaction<'a, Sqlite>, tags: &mut Vec<DbRoleTag>) -> Result<(), Error> {
    for tag in tags {
        save(tx, tag).await?;
    }
    return Ok(());
}
