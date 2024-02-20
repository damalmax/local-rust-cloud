use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Row, Sqlite, Transaction};

use crate::http::aws::iam::db::types::server_certificate::InsertServerCertificate;

pub(crate) async fn create<'a>(
    tx: &mut Transaction<'a, Sqlite>, cert: &mut InsertServerCertificate,
) -> Result<(), Error> {
    let result = sqlx::query(
        "INSERT INTO server_certificates (
            account_id, \
            arn, \
            path, \
            certificate_body, \
            certificate_chain, \
            server_certificate_name, \
            unique_server_certificate_name, \
            server_certificate_id, \
            upload_date, \
            expiration \
        ) \
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) \
        RETURNING id",
    )
    .bind(cert.account_id)
    .bind(&cert.arn)
    .bind(&cert.path)
    .bind(&cert.certificate_body)
    .bind(cert.certificate_chain.as_ref())
    .bind(&cert.server_certificate_name)
    .bind(cert.server_certificate_name.to_uppercase())
    .bind(&cert.server_certificate_id)
    .bind(cert.upload_date)
    .bind(cert.expiration)
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_one(tx.as_mut())
    .await?;

    cert.id = Some(result);
    Ok(())
}
