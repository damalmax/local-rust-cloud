use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Row, Sqlite, Transaction};

use crate::http::aws::iam::db::types::user::InsertUser;

pub(crate) async fn create<'a>(tx: &mut Transaction<'a, Sqlite>, user: &mut InsertUser) -> Result<(), Error> {
    let result = sqlx::query(
        r#"INSERT INTO users (
                        account_id,
                        username,
                        unique_username,
                        arn,
                        path,
                        user_id,
                        policy_id,
                        create_date
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                RETURNING id"#,
    )
    .bind(&user.account_id)
    .bind(&user.username)
    .bind(&user.username.to_uppercase())
    .bind(&user.arn)
    .bind(&user.path)
    .bind(&user.user_id)
    .bind(&user.policy_id)
    .bind(&user.create_date)
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_one(tx.as_mut())
    .await?;

    user.id = Some(result);
    Ok(())
}
