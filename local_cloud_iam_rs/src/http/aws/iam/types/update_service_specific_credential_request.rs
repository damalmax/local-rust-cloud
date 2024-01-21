use crate::http::aws::iam::types;
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct UpdateServiceSpecificCredentialRequest {
    #[serde(rename = "UserName")]
    pub(crate) user_name: Option<types::user_name_type::UserNameType>,
    #[serde(rename = "Status")]
    pub(crate) status: Option<types::status_type::StatusType>,
    #[serde(rename = "ServiceSpecificCredentialId")]
    pub(crate) service_specific_credential_id:
        Option<types::service_specific_credential_id::ServiceSpecificCredentialId>,
}
impl UpdateServiceSpecificCredentialRequest {
    pub(crate) fn user_name(&self) -> Option<&str> {
        self.user_name.as_deref()
    }
    pub(crate) fn status(&self) -> Option<&types::status_type::StatusType> {
        self.status.as_ref()
    }
    pub(crate) fn service_specific_credential_id(&self) -> Option<&str> {
        self.service_specific_credential_id.as_deref()
    }
}
impl local_cloud_validate::NamedValidator for &UpdateServiceSpecificCredentialRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_named(self.user_name.as_ref(), format!("{at}.{}", "UserName").as_str())?;
        local_cloud_validate::validate_required(self.status(), format!("{at}.{}", "Status").as_str())?;
        local_cloud_validate::validate_required(
            self.service_specific_credential_id(),
            format!("{at}.{}", "ServiceSpecificCredentialId").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.service_specific_credential_id.as_ref(),
            format!("{at}.{}", "ServiceSpecificCredentialId").as_str(),
        )?;
        Ok(())
    }
}
