use sqlx::{Error, Sqlite, Transaction};

use crate::http::aws::iam::db::types::tag::DbTag;

pub(crate) async fn save<'a>(tx: &mut Transaction<'a, Sqlite>, tag: &mut DbTag) -> Result<(), Error> {
    super::tag::save(tx, tag, "user_tags").await
}

pub(crate) async fn save_all<'a>(tx: &mut Transaction<'a, Sqlite>, tags: &mut Vec<DbTag>) -> Result<(), Error> {
    super::tag::save_all(tx, tags, "user_tags").await
}
