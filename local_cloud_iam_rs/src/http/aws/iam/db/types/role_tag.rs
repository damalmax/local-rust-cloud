use sqlx::FromRow;

#[derive(Clone, FromRow, Debug)]
pub(crate) struct DbRoleTag {
    pub(crate) id: Option<i64>,
    pub(crate) role_id: i64,
    pub(crate) key: String,
    pub(crate) value: String,
}

impl DbRoleTag {
    pub(crate) fn new(role_id: i64, key: impl Into<String>, value: impl Into<String>) -> Self {
        DbRoleTag {
            id: None,
            role_id,
            key: key.into(),
            value: value.into(),
        }
    }
}

impl Into<aws_sdk_iam::types::Tag> for &DbRoleTag {
    fn into(self) -> aws_sdk_iam::types::Tag {
        aws_sdk_iam::types::Tag::builder()
            .key(&self.key)
            .value(&self.value)
            .build()
            .unwrap()
    }
}
