use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Executor, FromRow, QueryBuilder, Row, Sqlite, Transaction};

use crate::http::aws::iam::db::types::common::ListByPathQuery;
use crate::http::aws::iam::db::types::server_certificate::{
    InsertServerCertificate, SelectServerCertificate, UpdateServerCertificateQuery,
};

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

pub(crate) async fn find_id_by_name<'a, E>(
    executor: E, account_id: i64, server_certificate_name: &str,
) -> Result<Option<i64>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let id = sqlx::query(
        r#"
            SELECT
                id
            FROM server_certificates
            WHERE account_id = $1 AND unique_server_certificate_name = $2
    "#,
    )
    .bind(account_id)
    .bind(server_certificate_name.to_uppercase())
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_optional(executor)
    .await?;

    Ok(id)
}

pub(crate) async fn update<'a, E>(
    executor: E, account_id: i64, query: &UpdateServerCertificateQuery,
) -> Result<bool, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new("UPDATE server_certificates SET");
    let mut added = false;
    if let Some(new_server_certificate_name) = &query.new_server_certificate_name {
        query_builder
            .push(" server_certificate_name=")
            .push_bind(new_server_certificate_name)
            .push(" , unique_server_certificate_name=")
            .push_bind(new_server_certificate_name.to_uppercase());
        added = true;
    }
    if let Some(new_path) = &query.new_path {
        if added {
            query_builder.push(" ,");
        }
        query_builder.push(" path=").push_bind(new_path);
    }

    let result = query_builder
        .push(" WHERE account_id=")
        .push_bind(account_id)
        .push(" AND unique_server_certificate_name=")
        .push_bind(&query.server_certificate_name.to_uppercase())
        .build()
        .execute(executor)
        .await?;

    Ok(result.rows_affected() == 1)
}

pub(crate) async fn list<'a, E>(
    executor: E, account_id: i64, query: &ListByPathQuery,
) -> Result<Vec<SelectServerCertificate>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        r#"
        SELECT
            sc.id AS id,
            sc.account_id AS account_id,
            sc.arn AS arn,
            sc.path AS path,
            sc.certificate_body AS certificate_body,
            sc.certificate_chain AS certificate_chain,
            sc.server_certificate_name AS server_certificate_name,
            sc.server_certificate_id AS server_certificate_id,
            sc.upload_date as upload_date,
            sc.expiration as expiration
        FROM server_certificates sc
        WHERE sc.account_id = "#,
    );
    let certificates = query_builder
        .push_bind(account_id)
        .push(" AND sc.path LIKE ")
        .push_bind(format!("{}%", &query.path_prefix))
        .push(" ORDER BY sc.unique_server_certificate_name ASC")
        .push(" LIMIT ")
        .push_bind(query.limit + 1) // request more elements than we need to return. used to identify if NextPage token needs to be generated
        .push(" OFFSET ")
        .push_bind(query.skip)
        .build()
        .map(|row: SqliteRow| SelectServerCertificate::from_row(&row).unwrap())
        .fetch_all(executor)
        .await?;
    Ok(certificates)
}
