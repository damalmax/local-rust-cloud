use sqlx::{Error, Executor, Sqlite, Transaction};

use crate::http::aws::iam::db::types::tags::{DbTag, ListTagsQuery};

pub(crate) async fn save<'a>(tx: &mut Transaction<'a, Sqlite>, tag: &mut DbTag) -> Result<(), Error> {
    super::tag::save(tx, tag, "mfa_device_tags").await
}

pub(crate) async fn save_all<'a>(tx: &mut Transaction<'a, Sqlite>, tags: &mut Vec<DbTag>) -> Result<(), Error> {
    super::tag::save_all(tx, tags, "mfa_device_tags").await
}

pub(crate) async fn list<'a, E>(executor: E, mfa_device_id: i64, query: &ListTagsQuery) -> Result<Vec<DbTag>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    super::tag::list(executor, "mfa_device_tags", mfa_device_id, query).await
}
