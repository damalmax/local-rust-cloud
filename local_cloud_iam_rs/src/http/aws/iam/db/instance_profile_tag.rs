use sqlx::{Error, Executor, Sqlite, Transaction};

use crate::http::aws::iam::db::types::tags::{DbTag, ListTagsQuery};

pub(crate) async fn find_by_instance_profile_id<'a, E>(
    executor: E, instance_profile_id: i64,
) -> Result<Vec<DbTag>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    super::tag::find_by_parent_id(executor, instance_profile_id, "instance_profile_tags").await
}

pub(crate) async fn save<'a>(tx: &mut Transaction<'a, Sqlite>, tag: &mut DbTag) -> Result<(), Error> {
    super::tag::save(tx, tag, "instance_profile_tags").await
}

pub(crate) async fn save_all<'a>(tx: &mut Transaction<'a, Sqlite>, tags: &mut Vec<DbTag>) -> Result<(), Error> {
    super::tag::save_all(tx, tags, "instance_profile_tags").await
}

pub(crate) async fn count<'a, E>(executor: E, instance_profile_id: i64) -> Result<usize, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    super::tag::count(executor, "instance_profile_tags", instance_profile_id).await
}

pub(crate) async fn list<'a, E>(
    executor: E, instance_profile_id: i64, query: &ListTagsQuery,
) -> Result<Vec<DbTag>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    super::tag::list(executor, "instance_profile_tags", instance_profile_id, query).await
}
