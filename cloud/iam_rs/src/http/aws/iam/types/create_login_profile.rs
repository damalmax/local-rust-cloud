use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct CreateLoginProfileRequest {
    #[serde(rename = "UserName")]
    pub(crate) user_name: Option<types::user_name_type::UserNameType>,
    #[serde(rename = "Password")]
    pub(crate) password: Option<types::password_type::PasswordType>,
    #[serde(rename = "PasswordResetRequired")]
    pub(crate) password_reset_required: Option<utils::types::Bool>,
}

impl CreateLoginProfileRequest {
    pub(crate) fn user_name(&self) -> Option<&str> {
        self.user_name.as_deref()
    }
    pub(crate) fn password(&self) -> Option<&str> {
        self.password.as_deref()
    }
    pub(crate) fn password_reset_required(&self) -> Option<bool> {
        self.password_reset_required.as_ref().map(|v| v.as_bool())
    }
}

impl validators::NamedValidator for &CreateLoginProfileRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_required(self.user_name(), format!("{at}.{}", "UserName").as_str())?;
        validators::validate_named(self.user_name.as_ref(), format!("{at}.{}", "UserName").as_str())?;
        validators::validate_required(self.password(), format!("{at}.{}", "Password").as_str())?;
        validators::validate_named(self.password.as_ref(), format!("{at}.{}", "Password").as_str())?;
        Ok(())
    }
}
