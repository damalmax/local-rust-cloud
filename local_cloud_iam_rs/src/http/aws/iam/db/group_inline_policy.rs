use sqlx::{Error, Executor, Sqlite, Transaction};

use crate::http::aws::iam::db;
use crate::http::aws::iam::db::types::common::ListByIdQuery;
use crate::http::aws::iam::db::types::inline_policy::DbInlinePolicy;

pub(crate) async fn save<'a>(tx: &mut Transaction<'a, Sqlite>, policy: &mut DbInlinePolicy) -> Result<(), Error> {
    super::inline_policy::save(tx, "group_inline_policies", policy).await
}

pub(crate) async fn save_all<'a>(
    tx: &mut Transaction<'a, Sqlite>, policies: &mut Vec<DbInlinePolicy>,
) -> Result<(), Error> {
    super::inline_policy::save_all(tx, "group_inline_policies", policies).await
}

pub(crate) async fn find_by_group_id_and_name<'a, E>(
    executor: E, group_id: i64, policy_name: &str,
) -> Result<Option<DbInlinePolicy>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    db::inline_policy::find_by_parent_id_and_name(executor, "group_inline_policies", group_id, policy_name).await
}

pub(crate) async fn find_by_group_id<'a, E>(executor: E, query: &ListByIdQuery) -> Result<Vec<DbInlinePolicy>, Error>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    db::inline_policy::find_by_parent_id(executor, "groups", "group_inline_policies", query).await
}
