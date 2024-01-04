use derive_builder::Builder;
use sqlx::FromRow;

#[derive(Clone, FromRow, Debug, Builder)]
pub(crate) struct DbPolicyTag {
    pub(crate) id: Option<i64>,
    pub(crate) policy_id: i64,
    pub(crate) key: String,
    pub(crate) value: String,
}

impl Into<aws_sdk_iam::types::Tag> for DbPolicyTag {
    fn into(self) -> aws_sdk_iam::types::Tag {
        aws_sdk_iam::types::Tag::builder()
            .key(self.key)
            .value(self.value)
            .build()
            .unwrap()
    }
}
