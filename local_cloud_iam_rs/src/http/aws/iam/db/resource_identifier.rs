use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Row, Sqlite, Transaction};

use crate::http::aws::iam::db::types::resource_identifier::ResourceIdentifier;

pub(crate) async fn create<'a>(
    tx: &mut Transaction<'a, Sqlite>, resource_identifier: &mut ResourceIdentifier,
) -> Result<(), Error> {
    let resource_type: i16 = resource_identifier.resource_type.into();
    let result = sqlx::query(
        r#"INSERT INTO unique_identifiers (
                    unique_id,
                    resource_type
                )
                VALUES ($1, $2)
                RETURNING id"#,
    )
    .bind(&resource_identifier.unique_id)
    .bind(resource_type)
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_one(tx.as_mut())
    .await?;

    resource_identifier.id = Some(result);
    Ok(())
}
