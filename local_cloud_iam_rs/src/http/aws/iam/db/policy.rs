use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Row, Sqlite, Transaction};

use crate::http::aws::iam::db::types::policy::InsertPolicy;

// pub async fn find_all_for_account<'a>(tx: &mut Transaction<'a, Sqlite>, account_id: i64) -> Vec<DbPolicy> {
//     let result = sqlx::query("SELECT * FROM policies WHERE account_id=$1")
//         .bind(account_id)
//         .map(|row: SqliteRow| DbPolicy::from_row(&row).unwrap())
//         .fetch_all(tx.as_mut())
//         .await;
//     result.unwrap()
// }

pub async fn save<'a>(tx: &mut Transaction<'a, Sqlite>, policy: &mut InsertPolicy) -> Result<(), Error> {
    let result = sqlx::query(
        r#"INSERT INTO policies (
                        account_id,
                        policy_name, 
                        policy_id,
                        arn, 
                        path,
                        is_attachable,
                        description, 
                        create_date,
                        update_date
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                RETURNING id"#,
    )
    .bind(&policy.account_id)
    .bind(&policy.policy_name)
    .bind(&policy.policy_id)
    .bind(&policy.arn)
    .bind(&policy.path)
    .bind(&policy.attachable)
    .bind(&policy.description)
    .bind(&policy.create_date)
    .bind(&policy.update_date)
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_one(tx.as_mut())
    .await;

    policy.id = Some(result.unwrap());
    Ok(())
}
