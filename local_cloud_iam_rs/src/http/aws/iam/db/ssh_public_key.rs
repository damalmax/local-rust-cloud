use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Executor, FromRow, Row, Sqlite, Transaction};

use crate::http::aws::iam::db::types::common::{ListByIdQuery, Pageable};
use crate::http::aws::iam::db::types::ssh_public_key::{
    InsertSshPublicKey, SelectSshPublicKey, UpdateSshPublicKeyQuery,
};

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

pub(crate) async fn find_by_user_id<'a, E>(executor: E, query: &ListByIdQuery) -> Result<Vec<SelectSshPublicKey>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let policies = sqlx::query(
        "SELECT \
            k.id AS id, \
            k.user_id AS user_id, \
            u.username AS user_name, \
            k.key_id AS key_id, \
            k.body AS body, \
            k.status AS status, \
            k.upload_date AS upload_date \
        FROM user_ssh_public_keys k LEFT JOIN users u ON k.user_id = u.id \
        WHERE k.user_id = $1 \
        ORDER BY k.key_id \
        LIMIT $2 OFFSET $3",
    )
    .bind(query.parent_id)
    .bind(query.limit() + 1)
    .bind(query.skip())
    .map(|row: SqliteRow| SelectSshPublicKey::from_row(&row).unwrap())
    .fetch_all(executor)
    .await?;

    Ok(policies)
}
