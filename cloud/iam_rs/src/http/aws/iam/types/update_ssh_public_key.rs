use validators::{validate_named, validate_required};

use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct UpdateSshPublicKeyRequest {
    #[serde(rename = "Status")]
    pub(crate) status: Option<types::status_type::StatusType>,
    #[serde(rename = "UserName")]
    pub(crate) user_name: Option<types::user_name_type::UserNameType>,
    #[serde(rename = "SSHPublicKeyId")]
    pub(crate) ssh_public_key_id: Option<types::public_key_id_type::PublicKeyIdType>,
}

impl UpdateSshPublicKeyRequest {
    pub(crate) fn status(&self) -> Option<&types::status_type::StatusType> {
        self.status.as_ref()
    }
    pub(crate) fn user_name(&self) -> Option<&str> {
        self.user_name.as_deref()
    }
    pub(crate) fn ssh_public_key_id(&self) -> Option<&str> {
        self.ssh_public_key_id.as_deref()
    }
}

impl validators::NamedValidator for &UpdateSshPublicKeyRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validate_required(self.status(), format!("{at}.{}", "Status").as_str())?;
        validate_required(self.user_name(), format!("{at}.{}", "UserName").as_str())?;
        validate_named(self.user_name.as_ref(), format!("{at}.{}", "UserName").as_str())?;
        validate_required(self.ssh_public_key_id(), format!("{at}.{}", "SSHPublicKeyId").as_str())?;
        validate_named(self.ssh_public_key_id.as_ref(), format!("{at}.{}", "SSHPublicKeyId").as_str())?;
        Ok(())
    }
}
