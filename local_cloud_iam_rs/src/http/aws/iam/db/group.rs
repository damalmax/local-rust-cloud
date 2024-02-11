use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Executor, FromRow, QueryBuilder, Row, Sqlite, Transaction};

use crate::http::aws::iam::db::types::group::{InsertGroup, ListGroupsQuery, SelectGroup};

pub(crate) async fn create<'a>(tx: &mut Transaction<'a, Sqlite>, group: &mut InsertGroup) -> Result<(), Error> {
    let result = sqlx::query(
        r#"INSERT INTO groups (
                    account_id,
                    group_name,
                    unique_group_name,
                    arn,
                    path,
                    group_id,
                    create_date
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING id"#,
    )
    .bind(group.account_id)
    .bind(&group.group_name)
    .bind(&group.group_name.to_uppercase())
    .bind(&group.arn)
    .bind(&group.path)
    .bind(&group.group_id)
    .bind(group.create_date)
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_one(tx.as_mut())
    .await?;
    group.id = Some(result);
    Ok(())
}

pub(crate) async fn list<'a, E>(executor: E, query: &ListGroupsQuery) -> Result<Vec<SelectGroup>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        r#"
            SELECT 
                id,
                account_id,
                group_name,
                unique_group_name,
                arn,
                path,
                group_id,
                create_date
            FROM groups
            WHERE path LIKE "#,
    );
    let result = query_builder
        .push_bind(format!("{}%", &query.path_prefix))
        .push(" ORDER BY id ASC")
        .push(" LIMIT ")
        .push_bind(query.limit + 1) // request more elements than we need to return. used to identify if NextPage token needs to be generated
        .push(" OFFSET ")
        .push_bind(query.skip)
        .build()
        .map(|row: SqliteRow| SelectGroup::from_row(&row).unwrap())
        .fetch_all(executor)
        .await?;

    Ok(result)
}

pub(crate) async fn find_by_name<'a, E>(
    executor: E, account_id: i64, group_name: &str,
) -> Result<Option<SelectGroup>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let group = sqlx::query(
        r#"
            SELECT 
                id,
                account_id,
                group_name,
                unique_group_name,
                arn,
                path,
                group_id,
                create_date
            FROM groups
            WHERE account_id = $1 AND unique_group_name = $2
    "#,
    )
    .bind(account_id)
    .bind(group_name.to_uppercase())
    .map(|row: SqliteRow| SelectGroup::from_row(&row).unwrap())
    .fetch_optional(executor)
    .await?;

    Ok(group)
}

pub(crate) async fn find_id_by_name<'a, E>(executor: E, account_id: i64, group_name: &str) -> Result<Option<i64>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let group_id = sqlx::query(
        r#"
            SELECT 
                id
            FROM groups
            WHERE account_id = $1 AND unique_group_name = $2
    "#,
    )
    .bind(account_id)
    .bind(group_name.to_uppercase())
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_optional(executor)
    .await?;

    Ok(group_id)
}

pub(crate) async fn assign_user_to_group<'a>(
    tx: &mut Transaction<'a, Sqlite>, group_id: i64, user_id: i64,
) -> Result<(), Error> {
    sqlx::query(r#"INSERT INTO group_users (group_id, user_id) VALUES ($1, $2)"#)
        .bind(group_id)
        .bind(user_id)
        .execute(tx.as_mut())
        .await?;
    Ok(())
}

pub(crate) async fn assign_policy_to_group<'a>(
    tx: &mut Transaction<'a, Sqlite>, group_id: i64, policy_id: i64,
) -> Result<(), Error> {
    sqlx::query(r#"INSERT INTO policy_groups (group_id, policy_id) VALUES ($1, $2)"#)
        .bind(group_id)
        .bind(policy_id)
        .execute(tx.as_mut())
        .await?;
    Ok(())
}
