use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct DeleteSshPublicKeyRequest {
    #[serde(rename = "UserName")]
    pub(crate) user_name: Option<types::user_name_type::UserNameType>,
    #[serde(rename = "SSHPublicKeyId")]
    pub(crate) ssh_public_key_id: Option<types::public_key_id_type::PublicKeyIdType>,
}

impl DeleteSshPublicKeyRequest {
    pub(crate) fn user_name(&self) -> Option<&str> {
        self.user_name.as_deref()
    }
    pub(crate) fn ssh_public_key_id(&self) -> Option<&str> {
        self.ssh_public_key_id.as_deref()
    }
}

impl validators::NamedValidator for &DeleteSshPublicKeyRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_required(self.user_name(), format!("{at}.{}", "UserName").as_str())?;
        validators::validate_named(self.user_name.as_ref(), format!("{at}.{}", "UserName").as_str())?;
        validators::validate_required(
            self.ssh_public_key_id(),
            format!("{at}.{}", "SSHPublicKeyId").as_str(),
        )?;
        validators::validate_named(
            self.ssh_public_key_id.as_ref(),
            format!("{at}.{}", "SSHPublicKeyId").as_str(),
        )?;
        Ok(())
    }
}
