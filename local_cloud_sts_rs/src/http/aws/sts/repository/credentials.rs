use crate::http::aws::sts::types::credentials::DbCredentials;
use futures::executor::block_on;
use sqlx::sqlite::SqliteRow;
use sqlx::{Row, Sqlite, Transaction};

pub fn create<'a>(tx: &mut Transaction<'a, Sqlite>, credentials: &mut DbCredentials) {
    let result = block_on(async {
        sqlx::query(
            r#"insert into credentials
            (access_key_id, secret_access_key, session_token, expiration, account_id, region_id)
            values ($1, $2, $3, $4, $5, $6) returning id"#,
        )
        .bind(&credentials.access_key_id)
        .bind(&credentials.secret_access_key)
        .bind(&credentials.session_token)
        .bind(&credentials.expiration)
        .bind(&credentials.account_id)
        .bind(&credentials.region_id)
        .map(|row: SqliteRow| row.get::<i64, &str>("id"))
        .fetch_one(tx.as_mut())
        .await
    });

    credentials.id = Some(result.unwrap());
}
