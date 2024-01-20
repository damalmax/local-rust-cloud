use crate::http::aws::iam::types;
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct GetSshPublicKeyRequest {
    #[serde(rename = "SSHPublicKeyId")]
    pub(crate) ssh_public_key_id: Option<types::public_key_id_type::PublicKeyIdType>,
    #[serde(rename = "UserName")]
    pub(crate) user_name: Option<types::user_name_type::UserNameType>,
    #[serde(rename = "Encoding")]
    pub(crate) encoding: Option<types::encoding_type::EncodingType>,
}
impl GetSshPublicKeyRequest {
    pub(crate) fn ssh_public_key_id(&self) -> Option<&str> {
        self.ssh_public_key_id.as_deref()
    }
    pub(crate) fn user_name(&self) -> Option<&str> {
        self.user_name.as_deref()
    }
    pub(crate) fn encoding(&self) -> Option<&types::encoding_type::EncodingType> {
        self.encoding.as_ref()
    }
}
impl local_cloud_validate::NamedValidator for &GetSshPublicKeyRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_required(
            self.ssh_public_key_id(),
            format!("{at}.{}", "SSHPublicKeyId").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.ssh_public_key_id.as_ref(),
            format!("{at}.{}", "SSHPublicKeyId").as_str(),
        )?;
        local_cloud_validate::validate_required(
            self.user_name(),
            format!("{at}.{}", "UserName").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.user_name.as_ref(),
            format!("{at}.{}", "UserName").as_str(),
        )?;
        local_cloud_validate::validate_required(
            self.encoding(),
            format!("{at}.{}", "Encoding").as_str(),
        )?;
        Ok(())
    }
}
