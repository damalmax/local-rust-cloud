use crate::http::aws::iam::types;
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct ResetServiceSpecificCredentialRequest {
    #[serde(rename = "ServiceSpecificCredentialId")]
    pub(crate) service_specific_credential_id:
        Option<types::service_specific_credential_id::ServiceSpecificCredentialId>,
    #[serde(rename = "UserName")]
    pub(crate) user_name: Option<types::user_name_type::UserNameType>,
}
impl ResetServiceSpecificCredentialRequest {
    pub(crate) fn service_specific_credential_id(&self) -> Option<&str> {
        self.service_specific_credential_id.as_deref()
    }
    pub(crate) fn user_name(&self) -> Option<&str> {
        self.user_name.as_deref()
    }
}
impl local_cloud_validate::NamedValidator for &ResetServiceSpecificCredentialRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_required(
            self.service_specific_credential_id(),
            format!("{at}.{}", "ServiceSpecificCredentialId").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.service_specific_credential_id.as_ref(),
            format!("{at}.{}", "ServiceSpecificCredentialId").as_str(),
        )?;
        local_cloud_validate::validate_named(self.user_name.as_ref(), format!("{at}.{}", "UserName").as_str())?;
        Ok(())
    }
}
