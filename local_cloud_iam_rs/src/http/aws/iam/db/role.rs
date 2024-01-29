use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Executor, Row, Sqlite, Transaction};

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

pub(crate) async fn find_id_by_name<'a, E>(executor: E, account_id: i64, role_name: &str) -> Result<Option<i64>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let group = sqlx::query(
        r#"
            SELECT 
                id
            FROM roles
            WHERE account_id = $1 AND unique_role_name = $2
    "#,
    )
    .bind(account_id)
    .bind(role_name.to_uppercase())
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_optional(executor)
    .await?;

    Ok(group)
}

pub(crate) async fn assign_policy_to_role<'a>(
    tx: &mut Transaction<'a, Sqlite>, role_id: i64, policy_id: i64,
) -> Result<(), Error> {
    sqlx::query(r#"INSERT INTO policy_roles (role_id, policy_id) VALUES ($1, $2)"#)
        .bind(role_id)
        .bind(policy_id)
        .execute(tx.as_mut())
        .await?;
    Ok(())
}
