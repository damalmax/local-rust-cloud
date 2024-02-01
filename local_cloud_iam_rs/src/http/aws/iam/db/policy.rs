use sqlx::pool::PoolConnection;
use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Executor, FromRow, QueryBuilder, Row, Sqlite, Transaction};

use crate::http::aws::iam::db::types::policy::{InsertPolicy, ListPoliciesQuery, SelectPolicy};

pub(crate) async fn create<'a>(tx: &mut Transaction<'a, Sqlite>, policy: &mut InsertPolicy) -> Result<(), Error> {
    let result = sqlx::query(
        r#"INSERT INTO policies (
                        account_id,
                        policy_name,
                        unique_policy_name,
                        policy_id,
                        arn,
                        path,
                        policy_type,
                        is_attachable,
                        description,
                        create_date,
                        update_date
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                RETURNING id"#,
    )
    .bind(policy.account_id)
    .bind(&policy.policy_name)
    .bind(&policy.policy_name.to_uppercase())
    .bind(&policy.policy_id)
    .bind(&policy.arn)
    .bind(&policy.path)
    .bind(&policy.policy_type)
    .bind(policy.attachable)
    .bind(&policy.description)
    .bind(policy.create_date)
    .bind(policy.update_date)
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_one(tx.as_mut())
    .await?;

    policy.id = Some(result);
    Ok(())
}

pub(crate) async fn find_id_by_arn<'a, E>(executor: E, account_id: i64, policy_arn: &str) -> Result<Option<i64>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let result = sqlx::query("SELECT id FROM policies WHERE account_id = $1 AND arn = $2")
        .bind(account_id)
        .bind(policy_arn)
        .map(|row: SqliteRow| row.get::<i64, &str>("id"))
        .fetch_optional(executor)
        .await?;
    Ok(result)
}

pub(crate) async fn find_by_id<'a, E>(executor: E, policy_id: i64) -> Result<Option<SelectPolicy>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    let result = sqlx::query(
        r#"
            SELECT 
                id,
                account_id,
                policy_name,
                arn,
                policy_id,
                path,
                create_date,
                update_date,
                policy_type,
                description,
                is_attachable,
                version 
            FROM policies 
            WHERE id = $1"#,
    )
    .bind(policy_id)
    .map(|row: SqliteRow| SelectPolicy::from_row(&row).unwrap())
    .fetch_optional(executor)
    .await?;
    Ok(result)
}

pub(crate) async fn list(
    connection: &mut PoolConnection<Sqlite>, account_id: i64, query: &ListPoliciesQuery,
) -> Result<Vec<SelectPolicy>, Error> {
    let scopes: Vec<i32> = query.policy_scope_types.iter().map(|v| v.as_i32()).collect();

    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        r#"
            SELECT 
                p.id AS id,
                p.account_id AS account_id,
                p.policy_name AS policy_name,
                p.unique_policy_name AS unique_policy_name,
                p.arn AS arn,
                p.policy_id AS policy_id,
                p.path AS path,
                p.create_date AS create_date,
                p.update_date AS update_date,
                p.policy_type AS policy_type,
                p.description AS description,
                p.is_attachable AS is_attachable,
                pv.version AS version,
                (
                    SELECT group_concat(tag, '♫')
                    FROM (
                        SELECT (pt.id || '♪' || pt.parent_id || '♪' || pt.key || '♪' || pt.value) AS tag
                        FROM policy_tags pt
                        WHERE pt.parent_id = p.id
                        ORDER BY pt.id
                    )
                ) AS tags
            FROM policies p LEFT JOIN policy_versions pv ON p.id = pv.policy_id AND pv.is_default = true
            WHERE p.account_id = "#,
    );
    query_builder
        .push_bind(account_id)
        .push(" AND path LIKE ")
        .push_bind(format!("{}%", &query.path_prefix))
        .push(" AND policy_type in (");
    let mut separated = query_builder.separated(", ");
    for scope in scopes {
        separated.push_bind(scope);
    }
    separated.push_unseparated(")");
    let policies = query_builder
        .push(" ORDER BY p.unique_policy_name")
        .push(" LIMIT ")
        .push_bind(query.limit + 1) // request more elements than we need to return. used to identify if NextPage token needs to be generated
        .push(" OFFSET ")
        .push_bind(query.skip)
        .build()
        .map(|row: SqliteRow| SelectPolicy::from_row(&row).unwrap())
        .fetch_all(connection.as_mut())
        .await?;
    Ok(policies)
}

#[cfg(test)]
mod tests {
    use core::fmt;

    use chrono::Utc;
    use sqlx::Error;
    use uuid::Uuid;

    use local_cloud_db::LocalDb;

    use crate::http::aws::iam::db::policy::create;
    use crate::http::aws::iam::db::types::policy::InsertPolicy;
    use crate::http::aws::iam::db::types::policy_type::PolicyType;

    #[tokio::test]
    async fn test_create_failed_already_exists() -> Result<(), fmt::Error> {
        let db_file_name = Uuid::new_v4();
        let database_url = format!("file:{}?mode=memory&cache=shared", db_file_name);

        let db = LocalDb::new(&database_url, &sqlx::migrate!()).await.unwrap();

        let mut tx = db.new_tx().await.unwrap();

        let mut policy = InsertPolicy {
            id: None,
            account_id: 1,
            arn: "arn:aws:iam::000000000001:policy/MyPolicy".to_string(),
            policy_id: "ANPA1212121212QWERTY0".to_string(),
            path: "/".to_string(),
            create_date: Utc::now().timestamp(),
            update_date: Utc::now().timestamp(),
            policy_name: "MyPolicy".to_string(),
            policy_type: PolicyType::LocalCloudManaged,
            description: None,
            attachable: false,
        };

        // first insert
        create(&mut tx, &mut policy).await.unwrap();
        // try to insert second time
        let result = create(&mut tx, &mut policy).await;
        assert!(result.is_err());

        let error = result.unwrap_err();
        match error {
            Error::Database(db_error) => {
                println!("{:?}", db_error.kind());
                assert_eq!(&db_error.code().unwrap(), "2067");
                assert_eq!(
                    db_error.message(),
                    "UNIQUE constraint failed: policies.account_id, policies.unique_policy_name"
                );
                Ok(())
            }
            _ => Err(fmt::Error),
        }
    }
}
