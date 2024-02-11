use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Executor, FromRow, Row, Sqlite, Transaction};

use crate::http::aws::iam::db::types::inline_policy::DbInlinePolicy;

pub(crate) async fn save<'a>(
    tx: &mut Transaction<'a, Sqlite>, table_name: &str, inline_policy: &mut DbInlinePolicy,
) -> Result<(), Error> {
    let result = sqlx::query(
        format!(
            "INSERT INTO {table_name}\
                (parent_id, policy_name, unique_policy_name, policy_document) \
             VALUES ($1, $2, $3, $4) \
             ON CONFLICT (parent_id, unique_policy_name) DO UPDATE SET policy_document=$4 \
             RETURNING id"
        )
        .as_str(),
    )
    .bind(inline_policy.parent_id)
    .bind(&inline_policy.policy_name)
    .bind(&inline_policy.policy_name.to_uppercase())
    .bind(&inline_policy.policy_document)
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_one(tx.as_mut())
    .await?;

    inline_policy.id = Some(result);

    Ok(())
}

pub(super) async fn save_all<'a>(
    tx: &mut Transaction<'a, Sqlite>, table_name: &str, policies: &mut Vec<DbInlinePolicy>,
) -> Result<(), Error> {
    for policy in policies {
        save(tx, table_name, policy).await?;
    }
    return Ok(());
}

pub(crate) async fn find_by_parent_id_and_name<'a, E>(
    executor: E, table_name: &str, parent_id: i64, policy_name: &str,
) -> Result<Option<DbInlinePolicy>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let policy = sqlx::query(
        format!(
            "SELECT id, parent_id, policy_name, policy_document \
             FROM {table_name} \
             WHERE parent_id = $1 AND unique_policy_name = $2"
        )
        .as_str(),
    )
    .bind(parent_id)
    .bind(policy_name.to_uppercase())
    .map(|row: SqliteRow| DbInlinePolicy::from_row(&row).unwrap())
    .fetch_optional(executor)
    .await?;

    Ok(policy)
}
