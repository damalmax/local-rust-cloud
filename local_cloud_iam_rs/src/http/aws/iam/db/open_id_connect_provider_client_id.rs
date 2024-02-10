use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Row, Sqlite, Transaction};

use crate::http::aws::iam::types::client_id_type::ClientIdType;

pub(crate) async fn create<'a>(
    tx: &mut Transaction<'a, Sqlite>, open_id_connect_provider_id: i64, client_id_type: &str,
) -> Result<i64, Error> {
    let result = sqlx::query(
        "INSERT INTO open_id_connect_provider_client_ids (provider_id, client_id) VALUES ($1, $2) RETURNING id",
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
