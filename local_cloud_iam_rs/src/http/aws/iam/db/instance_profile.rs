use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Row, Sqlite, Transaction};

use crate::http::aws::iam::db::types::instance_profile::InsertInstanceProfile;

pub(crate) async fn create<'a>(
    tx: &mut Transaction<'a, Sqlite>, instance_profile: &mut InsertInstanceProfile,
) -> Result<(), Error> {
    let result = sqlx::query(
        r#"INSERT INTO instance_profiles (
                    account_id,
                    instance_profile_name,
                    unique_instance_profile_name,
                    instance_profile_id,
                    arn,
                    path,
                    create_date
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING id"#,
    )
    .bind(instance_profile.account_id)
    .bind(&instance_profile.instance_profile_name)
    .bind(&instance_profile.instance_profile_name.to_uppercase())
    .bind(&instance_profile.instance_profile_id)
    .bind(&instance_profile.arn)
    .bind(&instance_profile.path)
    .bind(instance_profile.create_date)
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_one(tx.as_mut())
    .await?;
    instance_profile.id = Some(result);
    Ok(())
}
