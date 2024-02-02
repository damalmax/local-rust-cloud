use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Executor, FromRow, QueryBuilder, Row, Sqlite, Transaction};

use crate::http::aws::iam::db::types::role::{InsertRole, ListRolesQuery, SelectRole};

pub(crate) async fn create<'a>(tx: &mut Transaction<'a, Sqlite>, role: &mut InsertRole) -> Result<(), Error> {
    let result = sqlx::query(
        r#"INSERT INTO roles (
                    account_id,
                    role_name,
                    unique_role_name,
                    description,
                    max_session_duration,
                    assume_role_policy_document,
                    arn,
                    path,
                    role_id,
                    policy_id,
                    create_date
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                RETURNING id"#,
    )
    .bind(role.account_id)
    .bind(&role.role_name)
    .bind(&role.role_name.to_uppercase())
    .bind(&role.description)
    .bind(role.max_session_duration)
    .bind(&role.assume_role_policy_document)
    .bind(&role.arn)
    .bind(&role.path)
    .bind(&role.role_id)
    .bind(role.policy_id)
    .bind(role.create_date)
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_one(tx.as_mut())
    .await?;
    role.id = Some(result);
    Ok(())
}

pub(crate) async fn find_id_by_name<'a, E>(executor: E, account_id: i64, role_name: &str) -> Result<Option<i64>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let id = sqlx::query(
        r#"
            SELECT 
                id
            FROM roles
            WHERE account_id = $1 AND unique_role_name = $2
    "#,
    )
    .bind(account_id)
    .bind(role_name.to_uppercase())
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_optional(executor)
    .await?;

    Ok(id)
}

pub(crate) async fn assign_policy_to_role<'a>(
    tx: &mut Transaction<'a, Sqlite>, role_id: i64, policy_id: i64,
) -> Result<(), Error> {
    sqlx::query(r#"INSERT INTO policy_roles (role_id, policy_id) VALUES ($1, $2)"#)
        .bind(role_id)
        .bind(policy_id)
        .execute(tx.as_mut())
        .await?;
    Ok(())
}

pub(crate) async fn list<'a, E>(executor: E, account_id: i64, query: &ListRolesQuery) -> Result<Vec<SelectRole>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        r#"
            SELECT 
                r.id as id,
                r.account_id as account_id,
                r.role_name as role_name,
                r.unique_role_name as unique_role_name,
                r.description as description,
                r.max_session_duration as max_session_duration,
                r.assume_role_policy_document as assume_role_policy_document,
                r.arn as arn,
                r.path as path,
                r.role_id as role_id,
                r.policy_id as policy_id,
                r.create_date as create_date,
                r.last_used_date as last_used_date,
                r.last_used_region_id as last_used_region_id
            FROM roles r 
                LEFT JOIN policies p ON r.policy_id = p.id
            WHERE r.account_id = "#,
    );
    query_builder
        .push_bind(account_id)
        .push(" AND r.path LIKE ")
        .push_bind(format!("{}%", &query.path_prefix));

    let roles = query_builder
        .push(" ORDER BY r.unique_role_name")
        .push(" LIMIT ")
        .push_bind(query.limit + 1) // request more elements than we need to return. used to identify if NextPage token needs to be generated
        .push(" OFFSET ")
        .push_bind(query.skip)
        .build()
        .map(|row: SqliteRow| SelectRole::from_row(&row).unwrap())
        .fetch_all(executor)
        .await?;
    Ok(roles)
}
