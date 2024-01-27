use sqlx::FromRow;

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