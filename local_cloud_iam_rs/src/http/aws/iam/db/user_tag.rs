use sqlx::{Error, Executor, Sqlite, Transaction};

use crate::http::aws::iam::db::types::tags::{DbTag, ListTagsQuery};

pub(crate) async fn find_by_user_id<'a, E>(executor: E, user_id: i64) -> Result<Vec<DbTag>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    super::tag::find_by_parent_id(executor, user_id, "user_tags").await
}

pub(crate) async fn save<'a>(tx: &mut Transaction<'a, Sqlite>, tag: &mut DbTag) -> Result<(), Error> {
    super::tag::save(tx, tag, "user_tags").await
}

pub(crate) async fn save_all<'a>(tx: &mut Transaction<'a, Sqlite>, tags: &mut Vec<DbTag>) -> Result<(), Error> {
    super::tag::save_all(tx, tags, "user_tags").await
}

pub(crate) async fn list_tags<'a, E>(executor: E, user_id: i64, query: &ListTagsQuery) -> Result<Vec<DbTag>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    super::tag::list(executor, "user_tags", user_id, query).await
}
