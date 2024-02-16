use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Row, Sqlite, Transaction};

use crate::http::aws::iam::db::types::mfa_device::InsertMfaDevice;

pub(crate) async fn create<'a>(
    tx: &mut Transaction<'a, Sqlite>, mfa_device: &mut InsertMfaDevice,
) -> Result<(), Error> {
    let result = sqlx::query(
        r#"INSERT INTO mfa_devices (
                    account_id,
                    serial_number,
                    path,
                    name,
                    unique_name,
                    seed,
                    create_date
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING id"#,
    )
    .bind(mfa_device.account_id)
    .bind(&mfa_device.serial_number)
    .bind(&mfa_device.path)
    .bind(&mfa_device.name)
    .bind(&mfa_device.name.to_uppercase())
    .bind(&mfa_device.seed) // encrypt sensitive data (seed is sensitive)
    .bind(mfa_device.create_date)
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_one(tx.as_mut())
    .await?;

    mfa_device.id = Some(result);
    Ok(())
}
