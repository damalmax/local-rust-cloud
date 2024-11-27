use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Executor, FromRow, QueryBuilder, Row, Sqlite, Transaction};

use crate::http::aws::iam::db::types::common::ListByPathQuery;
use crate::http::aws::iam::db::types::user::{InsertUser, ListUsersByGroupQuery, SelectUser, UpdateUserQuery};

pub(crate) async fn create<'a>(tx: &mut Transaction<'a, Sqlite>, user: &mut InsertUser) -> Result<(), Error> {
    let result = sqlx::query(
        r#"INSERT INTO users (
                        account_id,
                        username,
                        unique_username,
                        arn,
                        path,
                        user_id,
                        policy_id,
                        create_date
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                RETURNING id"#,
    )
    .bind(user.account_id)
    .bind(&user.username)
    .bind(&user.username.to_uppercase())
    .bind(&user.arn)
    .bind(&user.path)
    .bind(&user.user_id)
    .bind(user.policy_id)
    .bind(user.create_date)
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_one(tx.as_mut())
    .await?;

    user.id = Some(result);
    Ok(())
}

pub(crate) async fn find_by_name<'a, E>(
    executor: E, account_id: i64, username: &str,
) -> Result<Option<SelectUser>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let result = sqlx::query(
        r#"
        SELECT
            u.id AS id,
            u.account_id AS account_id,
            u.username AS username,
            u.arn AS arn,
            u.path AS path,
            u.user_id AS user_id,
            u.policy_id AS policy_id,
            p.arn AS policy_arn,
            u.create_date AS create_date
        FROM users u LEFT JOIN policies p ON u.policy_id = p.id 
        WHERE u.account_id = $1 AND u.unique_username = $2"#,
    )
    .bind(account_id)
    .bind(username.to_uppercase())
    .map(|row: SqliteRow| SelectUser::from_row(&row).unwrap())
    .fetch_optional(executor)
    .await?;
    Ok(result)
}

pub(crate) async fn find_by_group_id<'a, E>(
    executor: E, query: &ListUsersByGroupQuery,
) -> Result<Vec<SelectUser>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        r#"
        SELECT 
            u.id AS id,
            u.account_id AS account_id,
            u.username AS username,
            u.arn AS arn,
            u.path AS path,
            u.user_id AS user_id,
            u.create_date as create_date
        FROM group_users gu LEFT JOIN users u ON gu.user_id = u.id
        WHERE gu.group_id ="#,
    );
    let users = query_builder
        .push_bind(query.group_id)
        .push(" ORDER BY u.unique_username ASC")
        .push(" LIMIT ")
        .push_bind(query.limit + 1) // request more elements than we need to return. used to identify if NextPage token needs to be generated
        .push(" OFFSET ")
        .push_bind(query.skip)
        .build()
        .map(|row: SqliteRow| SelectUser::from_row(&row).unwrap())
        .fetch_all(executor)
        .await?;
    Ok(users)
}

pub(crate) async fn find_id_by_name<'a, E>(executor: E, account_id: i64, user_name: &str) -> Result<Option<i64>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let user_id = sqlx::query(
        "SELECT id \
             FROM users \
             WHERE account_id = $1 AND unique_username = $2",
    )
    .bind(account_id)
    .bind(user_name.to_uppercase())
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_optional(executor)
    .await?;

    Ok(user_id)
}

pub(crate) async fn assign_policy_to_user<'a, E>(executor: E, user_id: i64, policy_id: i64) -> Result<(), Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    sqlx::query("INSERT INTO policy_users (user_id, policy_id) VALUES ($1, $2)")
        .bind(user_id)
        .bind(policy_id)
        .execute(executor)
        .await?;
    Ok(())
}

pub(crate) async fn detach_policy<'a>(
    tx: &mut Transaction<'a, Sqlite>, user_id: i64, policy_id: i64,
) -> Result<bool, Error> {
    let result = sqlx::query("DELETE FROM policy_users WHERE user_id=$1 AND policy_id=$2")
        .bind(user_id)
        .bind(policy_id)
        .execute(tx.as_mut())
        .await?;
    Ok(result.rows_affected() == 1)
}

pub(crate) async fn list<'a, E>(executor: E, account_id: i64, query: &ListByPathQuery) -> Result<Vec<SelectUser>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        r#"
        SELECT 
            u.id AS id,
            u.account_id AS account_id,
            u.username AS username,
            u.arn AS arn,
            u.path AS path,
            u.user_id AS user_id,
            u.create_date as create_date
        FROM group_users gu LEFT JOIN users u ON gu.user_id = u.id
        WHERE u.account_id = "#,
    );
    let users = query_builder
        .push_bind(account_id)
        .push(" AND u.path LIKE ")
        .push_bind(format!("{}%", &query.path_prefix))
        .push(" ORDER BY u.unique_username ASC")
        .push(" LIMIT ")
        .push_bind(query.limit + 1) // request more elements than we need to return. used to identify if NextPage token needs to be generated
        .push(" OFFSET ")
        .push_bind(query.skip)
        .build()
        .map(|row: SqliteRow| SelectUser::from_row(&row).unwrap())
        .fetch_all(executor)
        .await?;
    Ok(users)
}

pub(crate) async fn update<'a, E>(executor: E, account_id: i64, query: &UpdateUserQuery) -> Result<bool, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new("UPDATE users SET");
    let mut added = false;
    if let Some(new_user_name) = &query.new_user_name {
        query_builder
            .push(" username=")
            .push_bind(new_user_name)
            .push(" , unique_username=")
            .push_bind(new_user_name.to_uppercase());
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
        .push(" AND unique_username=")
        .push_bind(&query.user_name.to_uppercase())
        .build()
        .execute(executor)
        .await?;

    Ok(result.rows_affected() == 1)
}

pub(crate) async fn update_permissions_boundary<'a, E>(
    executor: E, account_id: i64, user_name: &str, policy_id: i64,
) -> Result<bool, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let result = sqlx::query(
        "UPDATE users SET policy_id=$1 \
        WHERE account_id=$2 AND unique_username=$3",
    )
    .bind(policy_id)
    .bind(account_id)
    .bind(&user_name.to_uppercase())
    .execute(executor)
    .await?;

    Ok(result.rows_affected() == 1)
}

pub(crate) async fn delete_permissions_boundary<'a, E>(
    executor: E, account_id: i64, user_name: &str,
) -> Result<bool, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let result = sqlx::query(
        "UPDATE users SET policy_id=NULL \
        WHERE account_id=$1 AND unique_username=$2",
    )
    .bind(account_id)
    .bind(&user_name.to_uppercase())
    .execute(executor)
    .await?;

    Ok(result.rows_affected() == 1)
}
