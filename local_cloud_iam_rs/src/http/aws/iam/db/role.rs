use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Row, Sqlite, Transaction};

use crate::http::aws::iam::db::types::role::InsertRole;

pub(crate) async fn create<'a>(tx: &mut Transaction<'a, Sqlite>, role: &mut InsertRole) -> Result<(), Error> {
    let result = sqlx::query(
        r#"INSERT INTO roles (
                    account_id,
                    role_name,
                    unique_role_name,
                    description,
                    max_session_duration,
                    arn,
                    path,
                    role_id,
                    policy_id,
                    create_date
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                RETURNING id"#,
    )
    .bind(&role.account_id)
    .bind(&role.role_name)
    .bind(&role.role_name.to_uppercase())
    .bind(&role.description)
    .bind(&role.max_session_duration)
    .bind(&role.arn)
    .bind(&role.path)
    .bind(&role.role_id)
    .bind(&role.policy_id)
    .bind(&role.create_date)
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_one(tx.as_mut())
    .await?;
    role.id = Some(result);
    Ok(())
}
