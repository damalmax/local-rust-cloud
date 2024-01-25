use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Row, Sqlite, Transaction};

use crate::http::aws::iam::db::types::user_tag::DbUserTag;

pub(crate) async fn save<'a>(tx: &mut Transaction<'a, Sqlite>, tag: &mut DbUserTag) -> Result<(), Error> {
    let result = sqlx::query(
        r#"INSERT INTO user_tags
                (user_id, key, value)
                VALUES ($1, $2, $3)
                ON CONFLICT(user_id, key) DO UPDATE SET value=$3
                RETURNING id"#,
    )
    .bind(&tag.user_id)
    .bind(&tag.key)
    .bind(&tag.value)
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_one(tx.as_mut())
    .await?;

    tag.id = Some(result);
    Ok(())
}

pub(crate) async fn save_all<'a>(tx: &mut Transaction<'a, Sqlite>, tags: &mut Vec<DbUserTag>) -> Result<(), Error> {
    for tag in tags {
        save(tx, tag).await?;
    }
    return Ok(());
}
