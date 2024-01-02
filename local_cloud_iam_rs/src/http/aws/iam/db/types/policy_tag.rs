use derive_builder::Builder;
use sqlx::FromRow;

#[derive(Clone, FromRow, Debug, Builder)]
pub struct DbPolicyTag {
    pub id: Option<i64>,
    pub policy_id: i64,
    pub key: String,
    pub value: String,
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
