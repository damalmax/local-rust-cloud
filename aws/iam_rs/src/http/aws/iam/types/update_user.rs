use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct UpdateUserRequest {
    #[serde(rename = "UserName")]
    pub(crate) user_name: Option<types::existing_user_name_type::ExistingUserNameType>,
    #[serde(rename = "NewPath")]
    pub(crate) new_path: Option<types::path_type::PathType>,
    #[serde(rename = "NewUserName")]
    pub(crate) new_user_name: Option<types::user_name_type::UserNameType>,
}

impl UpdateUserRequest {
    pub(crate) fn user_name(&self) -> Option<&str> {
        self.user_name.as_deref()
    }
    pub(crate) fn new_path(&self) -> Option<&str> {
        self.new_path.as_deref()
    }
    pub(crate) fn new_user_name(&self) -> Option<&str> {
        self.new_user_name.as_deref()
    }
}

impl validators::NamedValidator for &UpdateUserRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_required(self.user_name(), format!("{at}.{}", "UserName").as_str())?;
        validators::validate_named(self.user_name.as_ref(), format!("{at}.{}", "UserName").as_str())?;
        validators::validate_named(self.new_path.as_ref(), format!("{at}.{}", "NewPath").as_str())?;
        validators::validate_named(self.new_user_name.as_ref(), format!("{at}.{}", "NewUserName").as_str())?;
        Ok(())
    }
}
