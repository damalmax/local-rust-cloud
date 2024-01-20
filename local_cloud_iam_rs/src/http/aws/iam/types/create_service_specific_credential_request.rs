use crate::http::aws::iam::types;
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct CreateServiceSpecificCredentialRequest {
    #[serde(rename = "UserName")]
    pub(crate) user_name: Option<types::user_name_type::UserNameType>,
    #[serde(rename = "ServiceName")]
    pub(crate) service_name: Option<types::service_name::ServiceName>,
}
impl CreateServiceSpecificCredentialRequest {
    pub(crate) fn user_name(&self) -> Option<&str> {
        self.user_name.as_deref()
    }
    pub(crate) fn service_name(&self) -> Option<&str> {
        self.service_name.as_deref()
    }
}
impl local_cloud_validate::NamedValidator for &CreateServiceSpecificCredentialRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_required(
            self.user_name(),
            format!("{at}.{}", "UserName").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.user_name.as_ref(),
            format!("{at}.{}", "UserName").as_str(),
        )?;
        local_cloud_validate::validate_required(
            self.service_name(),
            format!("{at}.{}", "ServiceName").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.service_name.as_ref(),
            format!("{at}.{}", "ServiceName").as_str(),
        )?;
        Ok(())
    }
}
