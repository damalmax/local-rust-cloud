use sqlx::pool::PoolConnection;
use sqlx::sqlite::SqliteRow;
use sqlx::{Error, FromRow, QueryBuilder, Row, Sqlite, Transaction};

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
    .bind(&group.account_id)
    .bind(&group.group_name)
    .bind(&group.group_name.to_uppercase())
    .bind(&group.arn)
    .bind(&group.path)
    .bind(&group.group_id)
    .bind(&group.create_date)
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_one(tx.as_mut())
    .await?;
    group.id = Some(result);
    Ok(())
}

pub(crate) async fn list_groups(
    connection: &mut PoolConnection<Sqlite>, query: &ListGroupsQuery,
) -> Result<Vec<SelectGroup>, Error> {
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
        .push(" LIMIT ")
        .push_bind(query.limit + 1) // request more elements than we need to return. used to identify if NextPage token needs to be generated
        .push(" OFFSET ")
        .push_bind(query.skip)
        .build()
        .map(|row: SqliteRow| SelectGroup::from_row(&row).unwrap())
        .fetch_all(connection.as_mut())
        .await?;

    Ok(result)
}
