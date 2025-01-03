use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct ListServiceSpecificCredentialsRequest {
    #[serde(rename = "UserName")]
    pub(crate) user_name: Option<types::user_name_type::UserNameType>,
    #[serde(rename = "ServiceName")]
    pub(crate) service_name: Option<types::service_name::ServiceName>,
}

impl ListServiceSpecificCredentialsRequest {
    pub(crate) fn user_name(&self) -> Option<&str> {
        self.user_name.as_deref()
    }
    pub(crate) fn service_name(&self) -> Option<&str> {
        self.service_name.as_deref()
    }
}

impl validators::NamedValidator for &ListServiceSpecificCredentialsRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_named(self.user_name.as_ref(), format!("{at}.{}", "UserName").as_str())?;
        validators::validate_named(self.service_name.as_ref(), format!("{at}.{}", "ServiceName").as_str())?;
        Ok(())
    }
}
