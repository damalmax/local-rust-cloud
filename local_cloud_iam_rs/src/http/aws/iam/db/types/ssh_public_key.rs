use crate::http::aws::iam::db::types::ssh_public_key_type::SshPublicKeyStatusType;
use aws_sdk_iam::types::{SshPublicKeyMetadata, StatusType};
use aws_smithy_types::DateTime;
use sqlx::FromRow;

#[derive(Debug)]
pub(crate) struct InsertSshPublicKey {
    pub(crate) id: Option<i64>,
    pub(crate) user_id: i64,
    pub(crate) key_id: String,
    pub(crate) body: String,
    pub(crate) status: SshPublicKeyStatusType,
    pub(crate) upload_date: i64,
}

#[derive(Debug, FromRow)]
pub(crate) struct SelectSshPublicKey {
    pub(crate) id: i64,
    pub(crate) user_id: i64,
    pub(crate) user_name: String,
    pub(crate) key_id: String,
    pub(crate) body: String,
    pub(crate) status: SshPublicKeyStatusType,
    pub(crate) upload_date: i64,
}

impl SelectSshPublicKey {
    fn status(&self) -> &SshPublicKeyStatusType {
        &self.status
    }
}

impl From<&SelectSshPublicKey> for SshPublicKeyMetadata {
    fn from(value: &SelectSshPublicKey) -> Self {
        let status: StatusType = value.status().into();
        SshPublicKeyMetadata::builder()
            .user_name(&value.user_name)
            .ssh_public_key_id(&value.key_id)
            .upload_date(DateTime::from_secs(value.upload_date))
            .status(status)
            .build()
            .unwrap()
    }
}

#[derive(Debug)]
pub(crate) struct UpdateSshPublicKeyQuery {
    pub(crate) key_id: String,
    pub(crate) status: SshPublicKeyStatusType,
    pub(crate) user_id: i64,
}
