use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct UpdateAccountPasswordPolicyRequest {
    #[serde(rename = "MaxPasswordAge")]
    pub(crate) max_password_age: Option<types::max_password_age_type::MaxPasswordAgeType>,
    #[serde(rename = "RequireSymbols")]
    pub(crate) require_symbols: Option<local_cloud_common::types::Bool>,
    #[serde(rename = "RequireUppercaseCharacters")]
    pub(crate) require_uppercase_characters: Option<local_cloud_common::types::Bool>,
    #[serde(rename = "RequireLowercaseCharacters")]
    pub(crate) require_lowercase_characters: Option<local_cloud_common::types::Bool>,
    #[serde(rename = "PasswordReusePrevention")]
    pub(crate) password_reuse_prevention: Option<types::password_reuse_prevention_type::PasswordReusePreventionType>,
    #[serde(rename = "AllowUsersToChangePassword")]
    pub(crate) allow_users_to_change_password: Option<local_cloud_common::types::Bool>,
    #[serde(rename = "RequireNumbers")]
    pub(crate) require_numbers: Option<local_cloud_common::types::Bool>,
    #[serde(rename = "MinimumPasswordLength")]
    pub(crate) minimum_password_length: Option<types::minimum_password_length_type::MinimumPasswordLengthType>,
    #[serde(rename = "HardExpiry")]
    pub(crate) hard_expiry: Option<local_cloud_common::types::Bool>,
}

impl UpdateAccountPasswordPolicyRequest {
    pub(crate) fn max_password_age(&self) -> Option<&i32> {
        self.max_password_age.as_deref()
    }
    pub(crate) fn require_symbols(&self) -> Option<bool> {
        self.require_symbols.as_ref().map(|v| v.as_bool())
    }
    pub(crate) fn require_uppercase_characters(&self) -> Option<bool> {
        self.require_uppercase_characters.as_ref().map(|v| v.as_bool())
    }
    pub(crate) fn require_lowercase_characters(&self) -> Option<bool> {
        self.require_lowercase_characters.as_ref().map(|v| v.as_bool())
    }
    pub(crate) fn password_reuse_prevention(&self) -> Option<&i32> {
        self.password_reuse_prevention.as_deref()
    }
    pub(crate) fn allow_users_to_change_password(&self) -> Option<bool> {
        self.allow_users_to_change_password.as_ref().map(|v| v.as_bool())
    }
    pub(crate) fn require_numbers(&self) -> Option<bool> {
        self.require_numbers.as_ref().map(|v| v.as_bool())
    }
    pub(crate) fn minimum_password_length(&self) -> Option<&i32> {
        self.minimum_password_length.as_deref()
    }
    pub(crate) fn hard_expiry(&self) -> Option<bool> {
        self.hard_expiry.as_ref().map(|v| v.as_bool())
    }
}

impl local_cloud_validate::NamedValidator for &UpdateAccountPasswordPolicyRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_named(
            self.max_password_age.as_ref(),
            format!("{at}.{}", "MaxPasswordAge").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.password_reuse_prevention.as_ref(),
            format!("{at}.{}", "PasswordReusePrevention").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.minimum_password_length.as_ref(),
            format!("{at}.{}", "MinimumPasswordLength").as_str(),
        )?;
        Ok(())
    }
}
