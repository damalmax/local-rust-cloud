use sqlx::{sqlite::SqliteRow, Error, Executor, FromRow, Row, Sqlite, Transaction};

use super::types::open_id_connect_provider::{InsertOpenIdConnectProvider, SelectOpenIdConnectProvider};

pub(crate) async fn create<'a>(
    tx: &mut Transaction<'a, Sqlite>, provider: &mut InsertOpenIdConnectProvider,
) -> Result<(), Error> {
    let result = sqlx::query(
        "INSERT INTO open_id_connect_providers (account_id, arn, url, create_date) \
        VALUES ($1, $2, $3, $4) RETURNING id",
    )
    .bind(provider.account_id)
    .bind(&provider.arn)
    .bind(&provider.url)
    .bind(provider.create_date)
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_one(tx.as_mut())
    .await?;
    provider.id = Some(result);
    Ok(())
}

pub(crate) async fn find_id_by_arn<'a, E>(executor: E, account_id: i64, arn: &str) -> Result<Option<i64>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let result = sqlx::query("SELECT id FROM open_id_connect_providers WHERE account_id = $1 AND arn = $2")
        .bind(account_id)
        .bind(arn)
        .map(|row: SqliteRow| row.get::<i64, &str>("id"))
        .fetch_optional(executor)
        .await?;
    Ok(result)
}

pub(crate) async fn list<'a, E>(executor: E, account_id: i64) -> Result<Vec<SelectOpenIdConnectProvider>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let result = sqlx::query(
        "SELECT id, account_id, arn, url, create_date \
        FROM open_id_connect_providers WHERE account_id = $1",
    )
    .bind(account_id)
    .map(|row: SqliteRow| SelectOpenIdConnectProvider::from_row(&row).unwrap())
    .fetch_all(executor)
    .await?;
    Ok(result)
}
