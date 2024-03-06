use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Executor, FromRow, Row, Sqlite, Transaction};

use crate::http::aws::iam::db::types::open_id_connect_provider::SelectOpenIdConnectProviderClientId;
use crate::http::aws::iam::types::client_id_type::ClientIdType;

pub(crate) async fn create<'a>(
    tx: &mut Transaction<'a, Sqlite>, open_id_connect_provider_id: i64, client_id_type: &str,
) -> Result<i64, Error> {
    let result = sqlx::query(
        "INSERT INTO open_id_connect_provider_client_ids (provider_id, client_id) \
             VALUES ($1, $2) ON CONFLICT (provider_id, client_id) DO UPDATE SET client_id = $2 RETURNING id",
    )
    .bind(open_id_connect_provider_id)
    .bind(client_id_type)
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_one(tx.as_mut())
    .await?;
    Ok(result)
}

pub(crate) async fn create_all<'a>(
    tx: &mut Transaction<'a, Sqlite>, open_id_connect_provider_id: i64, client_id_list: &[ClientIdType],
) -> Result<(), Error> {
    for client_id in client_id_list {
        create(tx, open_id_connect_provider_id, client_id).await?;
    }
    Ok(())
}

pub(crate) async fn list<'a, E>(
    executor: E, open_id_connect_provider_id: i64,
) -> Result<Vec<SelectOpenIdConnectProviderClientId>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let result = sqlx::query(
        "SELECT \
            id, \
            client_id \
        FROM open_id_connect_provider_client_ids \
        WHERE provider_id = $1 \
        ORDER BY client_id ASC",
    )
    .bind(open_id_connect_provider_id)
    .map(|row: SqliteRow| SelectOpenIdConnectProviderClientId::from_row(&row).unwrap())
    .fetch_all(executor)
    .await?;
    Ok(result)
}
