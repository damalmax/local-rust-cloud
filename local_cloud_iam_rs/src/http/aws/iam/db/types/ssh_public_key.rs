use crate::http::aws::iam::db::types::ssh_public_key_type::SshPublicKeyStatusType;

#[derive(Debug)]
pub(crate) struct InsertSshPublicKey {
    pub(crate) id: Option<i64>,
    pub(crate) user_id: i64,
    pub(crate) key_id: String,
    pub(crate) body: String,
    pub(crate) status: SshPublicKeyStatusType,
    pub(crate) upload_date: i64,
}

pub(crate) struct UpdateSshPublicKeyQuery {
    pub(crate) key_id: String,
    pub(crate) status: SshPublicKeyStatusType,
    pub(crate) user_id: i64,
}
