#[derive(Debug)]
pub(crate) struct InsertInstanceProfile {
    pub(crate) id: Option<i64>,
    pub(crate) account_id: i64,
    pub(crate) instance_profile_name: String,
    pub(crate) instance_profile_id: String,
    pub(crate) arn: String,
    pub(crate) path: String,
    pub(crate) create_date: i64,
}
