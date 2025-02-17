use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Executor, FromRow, QueryBuilder, Row, Sqlite, Transaction};

use crate::http::aws::iam::db::types::mfa_device::{
    EnableMfaDeviceQuery, InsertMfaDevice, ListVirtualMfaDevicesQuery, SelectMfaDevice,
};
use crate::http::aws::iam::types::assignment_status_type::AssignmentStatusType;

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

pub(crate) async fn find_id_by_serial_number<'a, E>(
    executor: E, account_id: i64, serial_number: &str,
) -> Result<Option<i64>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let result = sqlx::query("SELECT id FROM mfa_devices WHERE account_id = $1 AND serial_number = $2")
        .bind(account_id)
        .bind(serial_number)
        .map(|row: SqliteRow| row.get::<i64, &str>("id"))
        .fetch_optional(executor)
        .await?;
    Ok(result)
}

pub(crate) async fn find_by_serial_number<'a, E>(
    executor: E, account_id: i64, serial_number: &str,
) -> Result<Option<SelectMfaDevice>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let result = sqlx::query(
        "SELECT md.id AS id, \
                    md.account_id AS account_id, \
                    md.serial_number AS serial_number, \
                    md.path AS path, \
                    md.name AS name, \
                    md.unique_name AS unique_name, \
                    md.seed AS seed, \
                    md.create_date AS create_date, \
                    md.enable_date AS enable_date, \
                    md.user_id AS user_id, \
                    u.user_id AS user_user_id, \
                    u.username AS user_name, \
                    u.arn AS user_arn, \
                    u.path AS user_path, \
                    u.create_date AS user_create_date, \
                    u.last_used_date AS user_password_last_used \
            FROM mfa_devices md LEFT JOIN users u ON md.user_id = u.id \
            WHERE md.account_id = $1 AND md.serial_number = $2",
    )
    .bind(account_id)
    .bind(serial_number)
    .map(|row: SqliteRow| SelectMfaDevice::from_row(&row).unwrap())
    .fetch_optional(executor)
    .await?;
    Ok(result)
}

pub(crate) async fn count<'a, E>(executor: E, account_id: i64, user_name: Option<&'a str>) -> Result<usize, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        "SELECT COUNT(md.id) AS count \
            FROM mfa_devices md LEFT JOIN users u ON md.user_id = u.id \
            WHERE md.account_id = ",
    );
    query_builder.push_bind(account_id);

    if let Some(user_name) = user_name {
        query_builder.push(" AND u.username = ").push_bind(user_name);
    }
    let result = query_builder
        .build()
        .map(|row: SqliteRow| row.get::<i64, &str>("count"))
        .fetch_one(executor)
        .await?;
    Ok(result as usize)
}

pub(crate) async fn enable<'a>(tx: &mut Transaction<'a, Sqlite>, query: &EnableMfaDeviceQuery) -> Result<(), Error> {
    sqlx::query(
        "UPDATE mfa_devices \
        SET \
            user_id = $1, \
            enable_date = $2, \
            code1 = $3, \
            code2 = $4 \
        WHERE id = $5 ",
    )
    .bind(query.user_id)
    .bind(query.enable_date)
    .bind(&query.code1)
    .bind(&query.code2)
    .bind(query.id)
    .execute(tx.as_mut())
    .await?;

    Ok(())
}

pub(crate) async fn disable<'a>(
    tx: &mut Transaction<'a, Sqlite>, mfa_device_id: i64, user_id: i64,
) -> Result<bool, Error> {
    let result = sqlx::query(
        "UPDATE mfa_devices \
        SET user_id = NULL, enable_date = NULL, code1 = NULL, code2 = NULL \
        WHERE id = $1 AND user_id = $2",
    )
    .bind(mfa_device_id)
    .bind(user_id)
    .execute(tx.as_mut())
    .await?;
    Ok(result.rows_affected() == 1)
}

pub(crate) async fn list_virtual<'a, E>(
    executor: E, account_id: i64, query: &ListVirtualMfaDevicesQuery,
) -> Result<Vec<SelectMfaDevice>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        "
            SELECT md.id AS id, \
                    md.account_id AS account_id, \
                    md.serial_number AS serial_number, \
                    md.path AS path, \
                    md.name AS name, \
                    md.unique_name AS unique_name, \
                    md.seed AS seed, \
                    md.create_date AS create_date, \
                    md.enable_date AS enable_date, \
                    md.user_id AS user_id, \
                    u.user_id AS user_user_id, \
                    u.username AS user_name, \
                    u.arn AS user_arn, \
                    u.path AS user_path, \
                    u.create_date AS user_create_date, \
                    u.last_used_date AS user_password_last_used \
            FROM mfa_devices md LEFT JOIN users u ON md.user_id = u.id \
            WHERE md.account_id = ",
    );
    query_builder.push_bind(account_id);

    if AssignmentStatusType::Assigned == query.assignment_status {
        query_builder.push(" AND md.user_id IS NOT NULL");
    } else if AssignmentStatusType::Unassigned == query.assignment_status {
        query_builder.push(" AND md.user_id IS NULL");
    }

    let result = query_builder
        .push(" ORDER BY unique_name ASC")
        .push(" LIMIT ")
        .push_bind(query.limit + 1) // request more elements than we need to return. used to identify if NextPage token needs to be generated
        .push(" OFFSET ")
        .push_bind(query.skip)
        .build()
        .map(|row: SqliteRow| SelectMfaDevice::from_row(&row).unwrap())
        .fetch_all(executor)
        .await?;

    Ok(result)
}
