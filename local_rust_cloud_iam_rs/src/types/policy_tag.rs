use sqlx::FromRow;

#[derive(Clone, FromRow, Debug)]
pub struct PolicyTag {
    pub id: Option<i64>,
    pub policy_id: i64,
    pub key: String,
    pub value: Option<String>,
}

impl Into<aws_sdk_iam::types::Tag> for PolicyTag {
    fn into(self) -> aws_sdk_iam::types::Tag {
        aws_sdk_iam::types::Tag::builder()
            .key(self.key)
            .value(self.value.unwrap_or(String::new()))
            .build()
    }
}

