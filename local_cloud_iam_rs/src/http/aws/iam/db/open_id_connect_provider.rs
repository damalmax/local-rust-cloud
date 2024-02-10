use sqlx::{sqlite::SqliteRow, Error, Row, Sqlite, Transaction};

use super::types::open_id_connect_provider::InsertOpenIdConnectProvider;

pub(crate) async fn create<'a>(
    tx: &mut Transaction<'a, Sqlite>, provider: &mut InsertOpenIdConnectProvider,
) -> Result<(), Error> {
    let result = sqlx::query(
        r#"INSERT INTO open_id_connect_providers (account_id, arn, url, create_date) VALUES ($1, $2, $3, $4) RETURNING id"#,
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
