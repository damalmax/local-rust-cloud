use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Row, Sqlite, Transaction};

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
