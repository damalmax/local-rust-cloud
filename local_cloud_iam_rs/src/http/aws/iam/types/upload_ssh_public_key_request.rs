use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct UploadSshPublicKeyRequest {
    #[serde(rename = "UserName")]
    pub(crate) user_name: Option<types::user_name_type::UserNameType>,
    #[serde(rename = "SSHPublicKeyBody")]
    pub(crate) ssh_public_key_body: Option<types::public_key_material_type::PublicKeyMaterialType>,
}

impl UploadSshPublicKeyRequest {
    pub(crate) fn user_name(&self) -> Option<&str> {
        self.user_name.as_deref()
    }
    pub(crate) fn ssh_public_key_body(&self) -> Option<&str> {
        self.ssh_public_key_body.as_deref()
    }
}

impl local_cloud_validate::NamedValidator for &UploadSshPublicKeyRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_required(self.user_name(), format!("{at}.{}", "UserName").as_str())?;
        local_cloud_validate::validate_named(self.user_name.as_ref(), format!("{at}.{}", "UserName").as_str())?;
        local_cloud_validate::validate_required(
            self.ssh_public_key_body(),
            format!("{at}.{}", "SSHPublicKeyBody").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.ssh_public_key_body.as_ref(),
            format!("{at}.{}", "SSHPublicKeyBody").as_str(),
        )?;
        Ok(())
    }
}
