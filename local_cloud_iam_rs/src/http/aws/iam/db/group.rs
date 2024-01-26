use crate::http::aws::iam::db::types::group::InsertGroup;
use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Row, Sqlite, Transaction};

pub(crate) async fn create<'a>(tx: &mut Transaction<'a, Sqlite>, group: &mut InsertGroup) -> Result<(), Error> {
    let result = sqlx::query(
        r#"INSERT INTO groups (
                    account_id,
                    group_name,
                    unique_group_name,
                    arn,
                    path,
                    group_id,
                    create_date
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING id"#,
    )
    .bind(&group.account_id)
    .bind(&group.group_name)
    .bind(&group.group_name.to_uppercase())
    .bind(&group.arn)
    .bind(&group.path)
    .bind(&group.group_id)
    .bind(&group.create_date)
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_one(tx.as_mut())
    .await?;
    group.id = Some(result);
    Ok(())
}
