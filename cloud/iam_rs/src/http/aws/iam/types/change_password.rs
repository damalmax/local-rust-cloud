use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct ChangePasswordRequest {
    #[serde(rename = "OldPassword")]
    pub(crate) old_password: Option<types::password_type::PasswordType>,
    #[serde(rename = "NewPassword")]
    pub(crate) new_password: Option<types::password_type::PasswordType>,
}

impl ChangePasswordRequest {
    pub(crate) fn old_password(&self) -> Option<&str> {
        self.old_password.as_deref()
    }
    pub(crate) fn new_password(&self) -> Option<&str> {
        self.new_password.as_deref()
    }
}

impl validators::NamedValidator for &ChangePasswordRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_required(self.old_password(), format!("{at}.{}", "OldPassword").as_str())?;
        validators::validate_named(self.old_password.as_ref(), format!("{at}.{}", "OldPassword").as_str())?;
        validators::validate_required(self.new_password(), format!("{at}.{}", "NewPassword").as_str())?;
        validators::validate_named(self.new_password.as_ref(), format!("{at}.{}", "NewPassword").as_str())?;
        Ok(())
    }
}
