use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Row, Sqlite, Transaction};

use crate::http::aws::iam::db::types::policy_version::InsertPolicyVersion;

pub(crate) async fn create<'a>(
    tx: &mut Transaction<'a, Sqlite>, policy_version: &mut InsertPolicyVersion,
) -> Result<(), Error> {
    let result = sqlx::query(
        r#"INSERT INTO policy_versions (
                        account_id,
                        policy_id,
                        policy_version_id,
                        policy_document,
                        create_date,
                        is_default
                )
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING id, version"#,
    )
    .bind(&policy_version.account_id)
    .bind(&policy_version.policy_id)
    .bind(&policy_version.policy_version_id)
    .bind(&policy_version.policy_document)
    .bind(&policy_version.create_date)
    .bind(&policy_version.is_default)
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_one(tx.as_mut())
    .await?;

    policy_version.id = Some(result);
    // populate version
    let version = sqlx::query("SELECT version FROM policy_versions WHERE id = $1")
        .bind(result)
        .map(|row: SqliteRow| row.get::<i16, &str>("version"))
        .fetch_one(tx.as_mut())
        .await?;
    policy_version.version = Some(version);
    Ok(())
}

pub(crate) async fn count_by_policy_id<'a>(tx: &mut Transaction<'a, Sqlite>, policy_id: i64) -> Result<usize, Error> {
    let result = sqlx::query("SELECT count(*) as items_count FROM policy_versions WHERE policy_id = $1")
        .bind(policy_id)
        .map(|row: SqliteRow| row.get::<i16, &str>("items_count"))
        .fetch_one(tx.as_mut())
        .await?;
    Ok(result as usize)
}

pub(crate) async fn disable_default_by_policy_id<'a>(
    tx: &mut Transaction<'a, Sqlite>, policy_id: i64,
) -> Result<(), Error> {
    sqlx::query("UPDATE policy_versions SET is_default=false WHERE policy_id=$1 AND is_default=true")
        .bind(policy_id)
        .execute(tx.as_mut())
        .await?;
    Ok(())
}
