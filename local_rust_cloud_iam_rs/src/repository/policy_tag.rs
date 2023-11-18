use sqlx::{sqlite::SqliteRow, Error, FromRow, Sqlite, Transaction, Row};

use crate::types::policy_tag::PolicyTag;

#[derive(Debug)]
pub struct PolicyTagRepo {}

impl PolicyTagRepo {
    pub async fn find_by_policy<'a>(&self, tx: &mut Transaction<'a, Sqlite>, policy_id: i64) -> Result<Vec<PolicyTag>, Error> {
        sqlx::query("SELECT id, policy_id, key, value FROM policy_tags WHERE policy_id=$1")
            .bind(policy_id)
            .map(|row: SqliteRow| PolicyTag::from_row(&row).unwrap())
            .fetch_all(tx)
            .await
    }

    pub async fn save<'a>(&self, tx: &mut Transaction<'a, Sqlite>, tag: &mut PolicyTag) -> Result<(), Error> {
        let result = sqlx::query(
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
        .fetch_one(tx)
        .await;

        tag.id = Option::Some(result.unwrap());
        return Result::Ok(());
    }

    pub async fn save_all<'a>(&self, tx: &mut Transaction<'a, Sqlite>, tags: Vec<&mut PolicyTag>) -> Result<(), Error> {
        for tag in tags {
            self.save(tx, tag).await?;
        }
        return Result::Ok(());
    }


    pub async fn delete_by_policy<'a>(&self, tx: &mut Transaction<'a, Sqlite>, policy_id: i64) -> Result<(), Error> {
        sqlx::query("DELETE * FROM policy_tags WHERE policy_id=$1")
        .bind(policy_id)
        .execute(tx)
        .await
        .map(|_| ())
    }

    pub fn new() -> PolicyTagRepo {
        PolicyTagRepo {}
    }
}
