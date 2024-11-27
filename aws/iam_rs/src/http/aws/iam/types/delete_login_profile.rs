use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct DeleteLoginProfileRequest {
    #[serde(rename = "UserName")]
    pub(crate) user_name: Option<types::user_name_type::UserNameType>,
}

impl DeleteLoginProfileRequest {
    pub(crate) fn user_name(&self) -> Option<&str> {
        self.user_name.as_deref()
    }
}

impl validators::NamedValidator for &DeleteLoginProfileRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_required(self.user_name(), format!("{at}.{}", "UserName").as_str())?;
        validators::validate_named(self.user_name.as_ref(), format!("{at}.{}", "UserName").as_str())?;
        Ok(())
    }
}
