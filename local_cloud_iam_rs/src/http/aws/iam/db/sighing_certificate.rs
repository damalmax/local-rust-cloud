use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Executor, Row, Sqlite, Transaction};

use crate::http::aws::iam::db::types::signing_certificate::{InsertSigningCertificate, UpdateSigningCertificateQuery};

pub(crate) async fn create<'a>(
    tx: &mut Transaction<'a, Sqlite>, cert: &mut InsertSigningCertificate,
) -> Result<(), Error> {
    let result = sqlx::query(
        "INSERT INTO signing_certificates ( \
            account_id, \
            certificate_id, \
            certificate_body, \
            status, \
            upload_date, \
            user_id \
        ) \
        VALUES ($1, $2, $3, $4, $5, $6) \
        RETURNING id",
    )
    .bind(cert.account_id)
    .bind(&cert.certificate_id)
    .bind(&cert.certificate_body)
    .bind(cert.status.as_i32())
    .bind(cert.upload_date)
    .bind(cert.user_id)
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_one(tx.as_mut())
    .await?;

    cert.id = Some(result);
    Ok(())
}

pub(crate) async fn update<'a, E>(executor: E, query: &UpdateSigningCertificateQuery) -> Result<bool, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let result = sqlx::query(
        "UPDATE signing_certificates \
            SET status=$1 \
            WHERE certificate_id=$2 AND user_id=$3",
    )
    .bind(query.status.as_i32())
    .bind(&query.certificate_id)
    .bind(query.user_id)
    .execute(executor)
    .await?;

    Ok(result.rows_affected() == 1)
}
