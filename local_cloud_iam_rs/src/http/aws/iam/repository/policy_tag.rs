use crate::http::aws::iam::types::policy_tag::DbPolicyTag;
use futures::executor::block_on;
use sqlx::sqlite::SqliteRow;
use sqlx::{Error, FromRow, Row, Sqlite, Transaction};

pub fn find_by_policy(tx: &mut Transaction<Sqlite>, policy_id: i64) -> Result<Vec<DbPolicyTag>, Error> {
    block_on(async {
        sqlx::query("SELECT id, policy_id, key, value FROM policy_tags WHERE policy_id=$1")
            .bind(policy_id)
            .map(|row: SqliteRow| DbPolicyTag::from_row(&row).unwrap())
            .fetch_all(tx.as_mut())
            .await
    })
}

pub fn save<'a>(tx: &mut Transaction<'a, Sqlite>, tag: &mut DbPolicyTag) -> Result<(), Error> {
    let result = block_on(async {
        sqlx::query(
            r#"INSERT INTO policy_tags
                (policy_id, key, value)
                VALUES ($1, $2, $3) 
                ON CONFLICT(policy_id, key) DO UPDATE SET value=$3
                RETURNING id"#,
        )
        .bind(&tag.policy_id)
        .bind(&tag.key)
        .bind(&tag.value)
        .map(|row: SqliteRow| row.get::<i64, &str>("id"))
        .fetch_one(tx.as_mut())
        .await
    });

    tag.id = Some(result.unwrap());
    Ok(())
}

pub fn save_all<'a>(tx: &mut Transaction<'a, Sqlite>, tags: &mut Vec<DbPolicyTag>) -> Result<(), Error> {
    for tag in tags {
        save(tx, tag)?;
    }
    return Ok(());
}

pub fn delete_by_policy(tx: &mut Transaction<Sqlite>, policy_id: i64) -> Result<(), Error> {
    block_on(async {
        sqlx::query("DELETE * FROM policy_tags WHERE policy_id=$1")
            .bind(policy_id)
            .execute(tx.as_mut())
            .await
            .map(|_| ())
    })
}
