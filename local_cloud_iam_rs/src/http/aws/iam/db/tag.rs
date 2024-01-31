use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Executor, FromRow, Row, Sqlite, Transaction};

use crate::http::aws::iam::db::types::tag::DbTag;

pub(super) async fn find_by_parent_id<'a, E>(executor: E, parent_id: i64, table_name: &str) -> Result<Vec<DbTag>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    sqlx::query(format!("SELECT id, parent_id, key, value FROM {table_name} WHERE parent_id=$1").as_str())
        .bind(parent_id)
        .map(|row: SqliteRow| DbTag::from_row(&row).unwrap())
        .fetch_all(executor)
        .await
}

pub(super) async fn save<'a>(tx: &mut Transaction<'a, Sqlite>, tag: &mut DbTag, table_name: &str) -> Result<(), Error> {
    let result = sqlx::query(
        format!(
            "INSERT INTO {table_name}
                (parent_id, key, value)
                VALUES ($1, $2, $3)
                ON CONFLICT(parent_id, key) DO UPDATE SET value=$3
                RETURNING id"
        )
        .as_str(),
    )
    .bind(tag.parent_id)
    .bind(&tag.key)
    .bind(&tag.value)
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_one(tx.as_mut())
    .await?;

    tag.id = Some(result);
    Ok(())
}

pub(super) async fn save_all<'a>(
    tx: &mut Transaction<'a, Sqlite>, tags: &mut Vec<DbTag>, table_name: &str,
) -> Result<(), Error> {
    for tag in tags {
        save(tx, tag, table_name).await?;
    }
    return Ok(());
}

pub(crate) async fn delete_by_parent_id<'a>(
    tx: &mut Transaction<'a, Sqlite>, parent_id: i64, table_name: &str,
) -> Result<(), Error> {
    sqlx::query(format!("DELETE * FROM {table_name} WHERE parent_id=$1").as_str())
        .bind(parent_id)
        .execute(tx.as_mut())
        .await
        .map(|_| ())
}
