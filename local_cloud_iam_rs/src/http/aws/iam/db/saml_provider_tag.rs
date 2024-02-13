use sqlx::{Error, Executor, Sqlite, Transaction};

use crate::http::aws::iam::db::types::tags::{DbTag, ListTagsQuery};

pub(crate) async fn find_by_saml_provider_id<'a, E>(executor: E, saml_provider_id: i64) -> Result<Vec<DbTag>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    super::tag::find_by_parent_id(executor, saml_provider_id, "saml_provider_tags").await
}

pub(crate) async fn save<'a>(tx: &mut Transaction<'a, Sqlite>, tag: &mut DbTag) -> Result<(), Error> {
    super::tag::save(tx, tag, "saml_provider_tags").await
}

pub(crate) async fn save_all<'a>(tx: &mut Transaction<'a, Sqlite>, tags: &mut Vec<DbTag>) -> Result<(), Error> {
    super::tag::save_all(tx, tags, "saml_provider_tags").await
}

pub(crate) async fn list<'a, E>(executor: E, saml_provider_id: i64, query: &ListTagsQuery) -> Result<Vec<DbTag>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    super::tag::list(executor, "saml_provider_tags", saml_provider_id, query).await
}
