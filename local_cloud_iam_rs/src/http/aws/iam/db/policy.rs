use sqlx::pool::PoolConnection;
use sqlx::sqlite::SqliteRow;
use sqlx::{Error, FromRow, Row, Sqlite, Transaction};

use crate::http::aws::iam::db::policy_tag;
use crate::http::aws::iam::db::types::policy::{InsertPolicy, ListPoliciesQuery, SelectPolicy, SelectPolicyWithTags};

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
    .bind(&policy.account_id)
    .bind(&policy.policy_name)
    .bind(&policy.policy_name.to_uppercase())
    .bind(&policy.policy_id)
    .bind(&policy.arn)
    .bind(&policy.path)
    .bind(&policy.policy_type)
    .bind(&policy.attachable)
    .bind(&policy.description)
    .bind(&policy.create_date)
    .bind(&policy.update_date)
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_one(tx.as_mut())
    .await?;

    policy.id = Some(result);
    Ok(())
}

pub(crate) async fn find_id_by_arn<'a>(
    tx: &mut Transaction<'a, Sqlite>, policy_arn: &str,
) -> Result<Option<i64>, Error> {
    let result = sqlx::query("SELECT id FROM policies WHERE arn = $1")
        .bind(policy_arn)
        .map(|row: SqliteRow| row.get::<i64, &str>("id"))
        .fetch_optional(tx.as_mut())
        .await?;
    Ok(result)
}

pub(crate) async fn list_policies(
    connection: &mut PoolConnection<Sqlite>, query: &ListPoliciesQuery,
) -> Result<Vec<SelectPolicyWithTags>, Error> {
    let mut policies = sqlx::query(
        r#"
            SELECT 
                p.id as id,
                p.account_id as account_id,
                p.policy_name as policy_name,
                p.arn as arn,
                p.policy_id as policy_id,
                p.path as path,
                p.create_date as create_date,
                p.update_date as update_date,
                p.policy_type as policy_type,
                p.description as description,
                p.is_attachable as is_attachable,
                pv.version as version
            FROM policies p LEFT JOIN policy_versions pv ON p.id = pv.policy_id AND pv.is_default = true
            WHERE path LIKE $1
            LIMIT $2 OFFSET $3"#,
    )
    .bind(format!("{}%", &query.path_prefix))
    .bind(query.limit + 1) // request more elements than we need to return. used to identify if NextPage token needs to be generated
    .bind(query.skip)
    .map(|row: SqliteRow| SelectPolicy::from_row(&row).unwrap())
    .fetch_all(connection.as_mut())
    .await?;

    let mut result: Vec<SelectPolicyWithTags> = vec![];
    for i in 0..policies.len() {
        let policy = policies.get(i).unwrap();
        let tags = policy_tag::find_by_policy(connection.as_mut(), policy.id).await?;
        result.push(SelectPolicyWithTags {
            policy: policy.clone(),
            tags,
        })
    }

    Ok(result)
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
                assert_eq!(db_error.message(), "UNIQUE constraint failed: policies.unique_policy_name");
                Ok(())
            }
            _ => Err(fmt::Error),
        }
    }
}
