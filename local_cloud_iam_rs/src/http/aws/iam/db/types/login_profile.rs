#[derive(Debug)]
pub(crate) struct InsertLoginProfile {
    pub(crate) id: Option<i64>,
    pub(crate) user_id: i64,
    pub(crate) password_hash: String,
    pub(crate) password_reset_required: bool,
    pub(crate) create_date: i64,
}
