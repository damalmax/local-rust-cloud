use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Executor, Row, Sqlite, Transaction};

use crate::http::aws::iam::db::types::instance_profile::InsertInstanceProfile;

pub(crate) async fn create<'a>(
    tx: &mut Transaction<'a, Sqlite>, instance_profile: &mut InsertInstanceProfile,
) -> Result<(), Error> {
    let result = sqlx::query(
        r#"INSERT INTO instance_profiles (
                    account_id,
                    instance_profile_name,
                    unique_instance_profile_name,
                    instance_profile_id,
                    arn,
                    path,
                    create_date
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING id"#,
    )
    .bind(instance_profile.account_id)
    .bind(&instance_profile.instance_profile_name)
    .bind(&instance_profile.instance_profile_name.to_uppercase())
    .bind(&instance_profile.instance_profile_id)
    .bind(&instance_profile.arn)
    .bind(&instance_profile.path)
    .bind(instance_profile.create_date)
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_one(tx.as_mut())
    .await?;
    instance_profile.id = Some(result);
    Ok(())
}

pub(crate) async fn find_id_by_name<'a, E>(
    executor: E, account_id: i64, instance_profile_name: &str,
) -> Result<Option<i64>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let group = sqlx::query(
        r#"
            SELECT 
                id
            FROM instance_profiles
            WHERE account_id = $1 AND unique_instance_profile_name = $2
    "#,
    )
    .bind(account_id)
    .bind(instance_profile_name.to_uppercase())
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_optional(executor)
    .await?;

    Ok(group)
}

pub(crate) async fn assign_role_to_instance_profile<'a>(
    tx: &mut Transaction<'a, Sqlite>, instance_profile_id: i64, role_id: i64,
) -> Result<(), Error> {
    sqlx::query(r#"INSERT INTO instance_profile_roles (instance_profile_id, role_id) VALUES ($1, $2)"#)
        .bind(instance_profile_id)
        .bind(role_id)
        .execute(tx.as_mut())
        .await?;
    Ok(())
}
