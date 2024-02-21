use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Executor, FromRow, QueryBuilder, Row, Sqlite, Transaction};

use crate::http::aws::iam::db::types::tags::{DbTag, ListTagsQuery};

#[derive(Debug)]
pub(crate) enum Tags {
    Policy,
    User,
    Role,
    InstanceProfile,
    SamlProvider,
    OpenIdConnectProvider,
    MfaDevice,
    ServerCertificate,
}

impl Tags {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Tags::Policy => "policy_tags",
            Tags::User => "user_tags",
            Tags::Role => "role_tags",
            Tags::InstanceProfile => "instance_profile_tags",
            Tags::SamlProvider => "saml_provider_tags",
            Tags::OpenIdConnectProvider => "open_id_connect_provider_tags",
            Tags::MfaDevice => "mfa_device_tags",
            Tags::ServerCertificate => "server_certificate_tags",
        }
    }

    pub(super) async fn find_by_parent_id<'a, E>(&self, executor: E, parent_id: i64) -> Result<Vec<DbTag>, Error>
    where
        E: 'a + Executor<'a, Database = Sqlite>,
    {
        let sql_query = format!(
            "SELECT id, parent_id, key, value \
             FROM {} WHERE parent_id=$1 \
             ORDER BY key",
            self.as_str()
        );
        sqlx::query(&sql_query)
            .bind(parent_id)
            .map(|row: SqliteRow| DbTag::from_row(&row).unwrap())
            .fetch_all(executor)
            .await
    }

    pub(super) async fn save<'a>(&self, tx: &mut Transaction<'a, Sqlite>, tag: &mut DbTag) -> Result<(), Error> {
        let sql_query = format!(
            "INSERT INTO {} \
                (parent_id, key, value) \
             VALUES ($1, $2, $3) \
             ON CONFLICT(parent_id, key) DO UPDATE SET value=$3 \
             RETURNING id",
            self.as_str()
        );
        let result = sqlx::query(&sql_query)
            .bind(tag.parent_id)
            .bind(&tag.key)
            .bind(&tag.value)
            .map(|row: SqliteRow| row.get::<i64, &str>("id"))
            .fetch_one(tx.as_mut())
            .await?;

        tag.id = Some(result);
        Ok(())
    }

    pub(crate) async fn save_all<'a>(
        &self, tx: &mut Transaction<'a, Sqlite>, tags: &mut Vec<DbTag>,
    ) -> Result<(), Error> {
        for tag in tags {
            self.save(tx, tag).await?;
        }
        return Ok(());
    }

    pub(crate) async fn delete<'a>(
        &self, tx: &mut Transaction<'a, Sqlite>, parent_id: i64, tag_key: &str,
    ) -> Result<(), Error> {
        let sql_query = format!("DELETE FROM {} WHERE parent_id=$1 AND key=$2", self.as_str());
        sqlx::query(&sql_query)
            .bind(parent_id)
            .bind(tag_key)
            .execute(tx.as_mut())
            .await
            .map(|_| ())
    }
    pub(crate) async fn delete_all<'a>(
        &self, tx: &mut Transaction<'a, Sqlite>, parent_id: i64, tag_keys: &[String],
    ) -> Result<(), Error> {
        for key in tag_keys {
            self.delete(tx, parent_id, key).await?;
        }
        return Ok(());
    }

    pub(crate) async fn delete_by_parent_id<'a>(
        &self, tx: &mut Transaction<'a, Sqlite>, parent_id: i64,
    ) -> Result<(), Error> {
        let sql_query = format!("DELETE FROM {} WHERE parent_id=$1", self.as_str());
        sqlx::query(&sql_query)
            .bind(parent_id)
            .execute(tx.as_mut())
            .await
            .map(|_| ())
    }

    pub(crate) async fn list<'a, E>(
        &self, executor: E, parent_id: i64, query: &ListTagsQuery,
    ) -> Result<Vec<DbTag>, Error>
    where
        E: 'a + Executor<'a, Database = Sqlite>,
    {
        let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new("SELECT id, parent_id, key, value FROM ");
        let tags = query_builder
            .push(self.as_str())
            .push(" WHERE parent_id = ")
            .push_bind(parent_id)
            .push(" ORDER BY key")
            .push(" LIMIT ")
            .push_bind(query.limit + 1) // request more elements than we need to return. used to identify if NextPage token needs to be generated
            .push(" OFFSET ")
            .push_bind(query.skip)
            .build()
            .map(|row: SqliteRow| DbTag::from_row(&row).unwrap())
            .fetch_all(executor)
            .await?;
        Ok(tags)
    }

    pub(crate) async fn count<'a, E>(&self, executor: E, parent_id: i64) -> Result<usize, Error>
    where
        E: 'a + Executor<'a, Database = Sqlite>,
    {
        let sql_query = format!(
            "SELECT COUNT(id) as count \
             FROM {} \
             WHERE parent_id = $1",
            self.as_str()
        );
        let result = sqlx::query(&sql_query)
            .bind(parent_id)
            .map(|row: SqliteRow| row.get::<i32, &str>("count"))
            .fetch_one(executor)
            .await?;
        Ok(result as usize)
    }
}
