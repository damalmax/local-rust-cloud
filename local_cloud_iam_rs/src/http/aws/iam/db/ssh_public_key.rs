use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Row, Sqlite, Transaction};

use crate::http::aws::iam::db::types::ssh_public_key::InsertSshPublicKey;

pub(crate) async fn upload<'a>(tx: &mut Transaction<'a, Sqlite>, key: &mut InsertSshPublicKey) -> Result<(), Error> {
    let result = sqlx::query(
        "INSERT INTO user_ssh_public_keys (user_id, key_id, body, status, upload_date) \
        VALUES ($1, $2, $3, $4, $5) \
        RETURNING id",
    )
    .bind(key.user_id)
    .bind(&key.key_id)
    .bind(&key.body)
    .bind(key.status.as_i32())
    .bind(key.upload_date)
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_one(tx.as_mut())
    .await?;
    key.id = Some(result);
    Ok(())
}
