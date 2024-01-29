use sqlx::{Error, Executor, Sqlite, Transaction};

use crate::http::aws::iam::db::types::tag::DbTag;

pub(crate) async fn find_by_policy_id<'a, E>(executor: E, policy_id: i64) -> Result<Vec<DbTag>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    super::tag::find_by_parent_id(executor, policy_id, "policy_tags").await
}

pub(crate) async fn save<'a>(tx: &mut Transaction<'a, Sqlite>, tag: &mut DbTag) -> Result<(), Error> {
    super::tag::save(tx, tag, "policy_tags").await
}

pub(crate) async fn save_all<'a>(tx: &mut Transaction<'a, Sqlite>, tags: &mut Vec<DbTag>) -> Result<(), Error> {
    super::tag::save_all(tx, tags, "policy_tags").await
}

pub(crate) async fn delete_by_policy_id<'a>(tx: &mut Transaction<'a, Sqlite>, policy_id: i64) -> Result<(), Error> {
    super::tag::delete_by_parent_id(tx, policy_id, "policy_tags").await
}
