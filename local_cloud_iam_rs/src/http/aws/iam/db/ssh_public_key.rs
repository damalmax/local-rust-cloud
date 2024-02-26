use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Executor, QueryBuilder, Row, Sqlite, Transaction};

use crate::http::aws::iam::db::types::ssh_public_key::InsertSshPublicKey;
use crate::http::aws::iam::db::types::user::UpdateUserQuery;

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

pub(crate) async fn update<'a, E>(executor: E, account_id: i64, query: &UpdateUserQuery) -> Result<bool, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new("UPDATE users SET");
    let mut added = false;
    if let Some(new_user_name) = &query.new_user_name {
        query_builder
            .push(" username=")
            .push_bind(new_user_name)
            .push(" , unique_username=")
            .push_bind(new_user_name.to_uppercase());
        added = true;
    }
    if let Some(new_path) = &query.new_path {
        if added {
            query_builder.push(" ,");
        }
        query_builder.push(" path=").push_bind(new_path);
    }

    let result = query_builder
        .push(" WHERE account_id=")
        .push_bind(account_id)
        .push(" AND unique_username=")
        .push_bind(&query.user_name)
        .build()
        .execute(executor)
        .await?;

    Ok(result.rows_affected() == 1)
}
