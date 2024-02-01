use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Row, Sqlite, Transaction};

use crate::http::aws::iam::db::types::login_profile::InsertLoginProfile;

pub(crate) async fn create<'a>(
    tx: &mut Transaction<'a, Sqlite>, login_profile: &mut InsertLoginProfile,
) -> Result<(), Error> {
    let result = sqlx::query(
        r#"INSERT INTO login_profiles (
                 user_id,
                 create_date,
                 password_hash,
                 password_reset_required
              )
              VALUES ($1, $2, $3, $4)
              RETURNING id"#,
    )
    .bind(login_profile.user_id)
    .bind(login_profile.create_date)
    .bind(&login_profile.password_hash)
    .bind(login_profile.password_reset_required)
    .map(|row: SqliteRow| row.get::<i64, &str>("id"))
    .fetch_one(tx.as_mut())
    .await?;
    login_profile.id = Some(result);
    Ok(())
}
