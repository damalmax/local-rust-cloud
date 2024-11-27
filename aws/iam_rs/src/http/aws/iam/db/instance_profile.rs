use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Executor, FromRow, QueryBuilder, Row, Sqlite, Transaction};

use crate::http::aws::iam::db::types::instance_profile::{
    InsertInstanceProfile, ListInstanceProfilesQuery, SelectInstanceProfile, SelectRoleForInstanceProfile,
};

pub(crate) async fn create<'a>(
    tx: &mut Transaction<'a, Sqlite>, instance_profile: &mut InsertInstanceProfile,
) -> Result<(), Error> {
    let result = sqlx::query(
        r#"INSERT INTO instance_profiles (
                    account_id,
                    instance_profile_name,
                    unique_instance_profile_name,
                    instance_profile_id,
                    arn,
                    path,
                    create_date
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING id"#,
    )
    .bind(instance_profile.account_id)
    .bind(&instance_profile.instance_profile_name)
    .bind(&instance_profile.instance_profile_name.to_uppercase())
    .bind(&instance_profile.instance_profile_id)
    .bind(&instance_profile.arn)
    .bind(&instance_profile.path)
    .bind(instance_profile.create_date)
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_one(tx.as_mut())
    .await?;
    instance_profile.id = Some(result);
    Ok(())
}

pub(crate) async fn find_id_by_name<'a, E>(
    executor: E, account_id: i64, instance_profile_name: &str,
) -> Result<Option<i64>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let group = sqlx::query(
        r#"
            SELECT 
                id
            FROM instance_profiles
            WHERE account_id = $1 AND unique_instance_profile_name = $2
    "#,
    )
    .bind(account_id)
    .bind(instance_profile_name.to_uppercase())
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_optional(executor)
    .await?;

    Ok(group)
}

pub(crate) async fn assign_role_to_instance_profile<'a>(
    tx: &mut Transaction<'a, Sqlite>, instance_profile_id: i64, role_id: i64,
) -> Result<(), Error> {
    sqlx::query(r#"INSERT INTO instance_profile_roles (instance_profile_id, role_id) VALUES ($1, $2)"#)
        .bind(instance_profile_id)
        .bind(role_id)
        .execute(tx.as_mut())
        .await?;
    Ok(())
}

pub(crate) async fn list_roles<'a, E>(
    executor: E, instance_profile_id: i64,
) -> Result<Vec<SelectRoleForInstanceProfile>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let roles = sqlx::query(
        "SELECT \
        r.id AS id, \
        r.assume_role_policy_document AS assume_role_policy_document, \
        r.arn AS arn, \
        r.path AS path, \
        r.create_date AS create_date, \
        r.role_id AS role_id, \
        r.role_name AS role_name \
        FROM instance_profile_roles ipr LEFT JOIN roles r ON ipr.role_id = r.id \
        WHERE ipr.instance_profile_id = $1",
    )
    .bind(instance_profile_id)
    .map(|row: SqliteRow| SelectRoleForInstanceProfile::from_row(&row).unwrap())
    .fetch_all(executor)
    .await?;
    Ok(roles)
}

pub(crate) async fn list<'a, E>(
    executor: E, account_id: i64, query: &ListInstanceProfilesQuery,
) -> Result<Vec<SelectInstanceProfile>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        "SELECT id, \
        account_id, \
        instance_profile_name, \
        unique_instance_profile_name, \
        instance_profile_id, \
        arn, \
        path, \
        create_date \
        FROM instance_profiles \
        WHERE account_id = ",
    );
    query_builder
        .push_bind(account_id)
        .push(" AND path LIKE ")
        .push_bind(format!("{}%", &query.path_prefix));
    let policies = query_builder
        .push(" ORDER BY unique_instance_profile_name")
        .push(" LIMIT ")
        .push_bind(query.limit + 1) // request more elements than we need to return. used to identify if NextPage token needs to be generated
        .push(" OFFSET ")
        .push_bind(query.skip)
        .build()
        .map(|row: SqliteRow| SelectInstanceProfile::from_row(&row).unwrap())
        .fetch_all(executor)
        .await?;
    Ok(policies)
}
