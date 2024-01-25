use sqlx::FromRow;

#[derive(Clone, FromRow, Debug)]
pub(crate) struct DbUserTag {
    pub(crate) id: Option<i64>,
    pub(crate) user_id: i64,
    pub(crate) key: String,
    pub(crate) value: String,
}

impl DbUserTag {
    pub(crate) fn new(user_id: i64, key: impl Into<String>, value: impl Into<String>) -> Self {
        DbUserTag {
            id: None,
            user_id,
            key: key.into(),
            value: value.into(),
        }
    }
}

impl Into<aws_sdk_iam::types::Tag> for &DbUserTag {
    fn into(self) -> aws_sdk_iam::types::Tag {
        aws_sdk_iam::types::Tag::builder()
            .key(&self.key)
            .value(&self.value)
            .build()
            .unwrap()
    }
}
