use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Row, Sqlite, Transaction};

use crate::http::aws::iam::db::types::policy::InsertPolicy;

pub(crate) async fn create<'a>(tx: &mut Transaction<'a, Sqlite>, policy: &mut InsertPolicy) -> Result<(), Error> {
    let result = sqlx::query(
        r#"INSERT INTO policies (
                        account_id,
                        policy_name,
                        unique_policy_name,
                        policy_id,
                        arn,
                        path,
                        is_attachable,
                        description,
                        create_date,
                        update_date
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                RETURNING id"#,
    )
    .bind(&policy.account_id)
    .bind(&policy.policy_name)
    .bind(&policy.policy_name.to_uppercase())
    .bind(&policy.policy_id)
    .bind(&policy.arn)
    .bind(&policy.path)
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
