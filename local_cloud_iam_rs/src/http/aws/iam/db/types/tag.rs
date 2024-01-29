use sqlx::{Error, FromRow};

use crate::http::aws::iam::db;

#[derive(Clone, FromRow, Debug)]
pub(crate) struct DbTag {
    pub(crate) id: Option<i64>,
    pub(crate) parent_id: i64,
    pub(crate) key: String,
    pub(crate) value: String,
}

impl DbTag {
    pub(crate) fn new(parent_id: i64, key: impl Into<String>, value: impl Into<String>) -> Self {
        DbTag {
            id: None,
            parent_id,
            key: key.into(),
            value: value.into(),
        }
    }

    pub(crate) fn from_str(raw: &str) -> Result<Self, Error> {
        let parts: Vec<&str> = raw.split(db::constants::COLUMN_SEPARATOR).collect();
        let id: Option<i64> = if parts[0].len() == 0 {
            // actually should not happen for select queries
            None
        } else {
            Some(parts[0].parse().unwrap()) // we consider this as a safe `unwrap` operation
        };
        let parent_id: i64 = parts[1].parse().unwrap(); // we consider this as a safe `unwrap` operation
        let key = parts[2].to_owned();
        let value = parts[3].to_owned();
        Ok(DbTag {
            id,
            parent_id,
            key,
            value,
        })
    }
}

pub(crate) fn parse_tags_from_raw(raw: &str) -> Result<Vec<DbTag>, Error> {
    let mut tags = vec![];
    for t in raw.split(db::constants::ROW_SEPARATOR) {
        tags.push(DbTag::from_str(t)?)
    }
    Ok(tags)
}

impl Into<aws_sdk_iam::types::Tag> for &DbTag {
    fn into(self) -> aws_sdk_iam::types::Tag {
        aws_sdk_iam::types::Tag::builder()
            .key(&self.key)
            .value(&self.value)
            .build()
            .unwrap()
    }
}
