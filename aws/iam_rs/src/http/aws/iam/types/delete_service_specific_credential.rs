use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct DeleteServiceSpecificCredentialRequest {
    #[serde(rename = "ServiceSpecificCredentialId")]
    pub(crate) service_specific_credential_id:
        Option<types::service_specific_credential_id::ServiceSpecificCredentialId>,
    #[serde(rename = "UserName")]
    pub(crate) user_name: Option<types::user_name_type::UserNameType>,
}

impl DeleteServiceSpecificCredentialRequest {
    pub(crate) fn service_specific_credential_id(&self) -> Option<&str> {
        self.service_specific_credential_id.as_deref()
    }
    pub(crate) fn user_name(&self) -> Option<&str> {
        self.user_name.as_deref()
    }
}

impl validators::NamedValidator for &DeleteServiceSpecificCredentialRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_required(
            self.service_specific_credential_id(),
            format!("{at}.{}", "ServiceSpecificCredentialId").as_str(),
        )?;
        validators::validate_named(
            self.service_specific_credential_id.as_ref(),
            format!("{at}.{}", "ServiceSpecificCredentialId").as_str(),
        )?;
        validators::validate_named(self.user_name.as_ref(), format!("{at}.{}", "UserName").as_str())?;
        Ok(())
    }
}
