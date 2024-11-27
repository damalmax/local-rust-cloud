use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct RemoveUserFromGroupRequest {
    #[serde(rename = "GroupName")]
    pub(crate) group_name: Option<types::group_name_type::GroupNameType>,
    #[serde(rename = "UserName")]
    pub(crate) user_name: Option<types::existing_user_name_type::ExistingUserNameType>,
}

impl RemoveUserFromGroupRequest {
    pub(crate) fn group_name(&self) -> Option<&str> {
        self.group_name.as_deref()
    }
    pub(crate) fn user_name(&self) -> Option<&str> {
        self.user_name.as_deref()
    }
}

impl validators::NamedValidator for &RemoveUserFromGroupRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_required(self.group_name(), format!("{at}.{}", "GroupName").as_str())?;
        validators::validate_named(self.group_name.as_ref(), format!("{at}.{}", "GroupName").as_str())?;
        validators::validate_required(self.user_name(), format!("{at}.{}", "UserName").as_str())?;
        validators::validate_named(self.user_name.as_ref(), format!("{at}.{}", "UserName").as_str())?;
        Ok(())
    }
}
