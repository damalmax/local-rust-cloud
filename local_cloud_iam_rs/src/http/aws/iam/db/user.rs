use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Executor, FromRow, QueryBuilder, Row, Sqlite, Transaction};

use crate::http::aws::iam::db::types::user::{InsertUser, ListUsersByGroupQuery, ListUsersQuery, SelectUser};

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
            u.create_date AS create_date,
            (
                SELECT group_concat(tag, '♫')
                FROM (
                    SELECT (ut.id || '♪' || ut.parent_id || '♪' || ut.key || '♪' || ut.value) AS tag
                    FROM user_tags ut
                    WHERE ut.parent_id = u.id
                    ORDER BY ut.id
                )
            ) AS tags
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
            u.id as id,
            u.account_id as account_id,
            u.username as username,
            u.arn as arn,
            u.path as path,
            u.user_id as user_id,
            u.policy_id AS policy_id,
            p.arn AS policy_arn,
            u.create_date as create_date,
            (
                SELECT group_concat(tag, '♫')
                FROM (
                    SELECT (ut.id || '♪' || ut.parent_id || '♪' || ut.key || '♪' || ut.value) AS tag
                    FROM user_tags ut
                    WHERE ut.parent_id = u.id
                    ORDER BY ut.id
                )
            ) AS tags
        FROM group_users gu LEFT JOIN users u ON gu.user_id = u.id LEFT JOIN policies p ON u.policy_id = p.id
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
    let group = sqlx::query(
        r#"
            SELECT 
                id
            FROM users
            WHERE account_id = $1 AND unique_username = $2
    "#,
    )
    .bind(account_id)
    .bind(user_name.to_uppercase())
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_optional(executor)
    .await?;

    Ok(group)
}

pub(crate) async fn assign_policy_to_user<'a>(
    tx: &mut Transaction<'a, Sqlite>, user_id: i64, policy_id: i64,
) -> Result<(), Error> {
    sqlx::query(r#"INSERT INTO policy_users (user_id, policy_id) VALUES ($1, $2)"#)
        .bind(user_id)
        .bind(policy_id)
        .execute(tx.as_mut())
        .await?;
    Ok(())
}

pub(crate) async fn list<'a, E>(executor: E, account_id: i64, query: &ListUsersQuery) -> Result<Vec<SelectUser>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        r#"
        SELECT 
            u.id as id,
            u.account_id as account_id,
            u.username as username,
            u.arn as arn,
            u.path as path,
            u.user_id as user_id,
            u.policy_id AS policy_id,
            p.arn AS policy_arn,
            u.create_date as create_date,
            (
                SELECT group_concat(tag, '♫')
                FROM (
                    SELECT (ut.id || '♪' || ut.parent_id || '♪' || ut.key || '♪' || ut.value) AS tag
                    FROM user_tags ut
                    WHERE ut.parent_id = u.id
                    ORDER BY ut.id
                )
            ) AS tags
        FROM group_users gu LEFT JOIN users u ON gu.user_id = u.id LEFT JOIN policies p ON u.policy_id = p.id
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
