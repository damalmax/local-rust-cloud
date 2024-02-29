use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Executor, Row, Sqlite, Transaction};

use crate::http::aws::iam::db::types::ssh_public_key::{InsertSshPublicKey, UpdateSshPublicKeyQuery};

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

pub(crate) async fn update<'a, E>(executor: E, query: &UpdateSshPublicKeyQuery) -> Result<bool, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let result = sqlx::query(
        "UPDATE user_ssh_public_keys \
            SET status=$1 \
            WHERE key_id=$2 AND user_id=$3",
    )
    .bind(query.status.as_i32())
    .bind(&query.key_id)
    .bind(query.user_id)
    .execute(executor)
    .await?;

    Ok(result.rows_affected() == 1)
}
