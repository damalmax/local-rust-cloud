use sqlx::sqlite::SqliteRow;
use sqlx::{Error, FromRow, Row, Sqlite, Transaction};

use crate::http::aws::iam::db::types::policy_tag::DbPolicyTag;

pub async fn find_by_policy<'a>(tx: &mut Transaction<'a, Sqlite>, policy_id: i64) -> Result<Vec<DbPolicyTag>, Error> {
    sqlx::query("SELECT id, policy_id, key, value FROM policy_tags WHERE policy_id=$1")
        .bind(policy_id)
        .map(|row: SqliteRow| DbPolicyTag::from_row(&row).unwrap())
        .fetch_all(tx.as_mut())
        .await
}

pub async fn save<'a>(tx: &mut Transaction<'a, Sqlite>, tag: &mut DbPolicyTag) -> Result<(), Error> {
    let result = sqlx::query(
        r#"INSERT INTO policy_tags
                (policy_id, key, value)
                VALUES ($1, $2, $3)
                ON CONFLICT(policy_id, key) DO UPDATE SET value=$3
                RETURNING id"#,
    )
    .bind(&tag.policy_id)
    .bind(&tag.key)
    .bind(&tag.value)
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_one(tx.as_mut())
    .await;

    tag.id = Some(result.unwrap());
    Ok(())
}

pub async fn save_all<'a>(tx: &mut Transaction<'a, Sqlite>, tags: &mut Vec<DbPolicyTag>) -> Result<(), Error> {
    for tag in tags {
        save(tx, tag).await?;
    }
    return Ok(());
}

pub async fn delete_by_policy<'a>(tx: &mut Transaction<'a, Sqlite>, policy_id: i64) -> Result<(), Error> {
    sqlx::query("DELETE * FROM policy_tags WHERE policy_id=$1")
        .bind(policy_id)
        .execute(tx.as_mut())
        .await
        .map(|_| ())
}
