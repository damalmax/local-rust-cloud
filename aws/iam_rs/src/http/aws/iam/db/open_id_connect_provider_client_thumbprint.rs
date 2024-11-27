use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Executor, FromRow, Row, Sqlite, Transaction};

use crate::http::aws::iam::db::types::open_id_connect_provider::SelectOpenIdConnectProviderThumbprint;
use crate::http::aws::iam::types::thumbprint_type::ThumbprintType;

pub(crate) async fn create<'a>(
    tx: &mut Transaction<'a, Sqlite>, open_id_connect_provider_id: i64, thumbprint: &str,
) -> Result<i64, Error> {
    let result = sqlx::query(
        "INSERT INTO open_id_connect_provider_thumbprints (provider_id, thumbprint) VALUES ($1, $2) RETURNING id",
    )
    .bind(open_id_connect_provider_id)
    .bind(thumbprint)
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_one(tx.as_mut())
    .await?;
    Ok(result)
}

pub(crate) async fn create_all<'a>(
    tx: &mut Transaction<'a, Sqlite>, open_id_connect_provider_id: i64, thumbprints: &[ThumbprintType],
) -> Result<(), Error> {
    for thumbprint in thumbprints {
        create(tx, open_id_connect_provider_id, thumbprint).await?;
    }
    Ok(())
}

pub(crate) async fn list<'a, E>(
    executor: E, open_id_connect_provider_id: i64,
) -> Result<Vec<SelectOpenIdConnectProviderThumbprint>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let result = sqlx::query(
        "SELECT \
            id, \
            thumbprint \
        FROM open_id_connect_provider_thumbprints \
        WHERE provider_id = $1",
    )
    .bind(open_id_connect_provider_id)
    .map(|row: SqliteRow| SelectOpenIdConnectProviderThumbprint::from_row(&row).unwrap())
    .fetch_all(executor)
    .await?;
    Ok(result)
}
