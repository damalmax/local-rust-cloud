use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Executor, Row, Sqlite, Transaction};

use crate::http::aws::iam::db::types::saml_provider::InsertSamlProvider;

pub(crate) async fn create<'a>(
    tx: &mut Transaction<'a, Sqlite>, provider: &mut InsertSamlProvider,
) -> Result<(), Error> {
    let id = sqlx::query(
        r#"INSERT INTO saml_providers (
                        account_id,
                        name,
                        unique_name,
                        arn,
                        create_date,
                        valid_until,
                        metadata_document
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING id"#,
    )
    .bind(provider.account_id)
    .bind(&provider.name)
    .bind(&provider.name.to_uppercase())
    .bind(&provider.arn)
    .bind(provider.create_date)
    .bind(provider.valid_until)
    .bind(&provider.metadata_document)
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_one(tx.as_mut())
    .await?;

    provider.id = Some(id);
    Ok(())
}

pub(crate) async fn find_id_by_arn<'a, E>(executor: E, account_id: i64, arn: &str) -> Result<Option<i64>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let result = sqlx::query("SELECT id FROM saml_providers WHERE account_id = $1 AND arn = $2")
        .bind(account_id)
        .bind(arn)
        .map(|row: SqliteRow| row.get::<i64, &str>("id"))
        .fetch_optional(executor)
        .await?;
    Ok(result)
}
