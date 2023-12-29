use crate::http::aws::iam::types::policy::DbPolicy;
use futures::executor::block_on;
use sqlx::sqlite::SqliteRow;
use sqlx::{Error, FromRow, Row, Sqlite, Transaction};

pub fn find_all_for_account(tx: &mut Transaction<Sqlite>, account_id: i64) -> Vec<DbPolicy> {
    let result = block_on(async {
        sqlx::query("SELECT * FROM policies WHERE account_id=$1")
            .bind(account_id)
            .map(|row: SqliteRow| DbPolicy::from_row(&row).unwrap())
            .fetch_all(tx.as_mut())
            .await
    });
    result.unwrap()
}

pub fn save<'a>(tx: &mut Transaction<'a, Sqlite>, policy: &mut DbPolicy) -> Result<(), Error> {
    let result = block_on(async {
        sqlx::query(
            r#"INSERT INTO policies (
                        account_id,
                        policy_name, 
                        policy_id,
                        arn, 
                        path,
                        default_version_id, 
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
        .bind(&policy.policy_id)
        .bind(&policy.arn)
        .bind(&policy.path)
        .bind(&policy.default_version_id)
        .bind(&policy.is_attachable)
        .bind(&policy.description)
        .bind(&policy.create_date)
        .bind(&policy.update_date)
        .map(|row: SqliteRow| row.get::<i64, &str>("id"))
        .fetch_one(tx.as_mut())
        .await
    });

    policy.id = Some(result.unwrap());
    Ok(())
}
