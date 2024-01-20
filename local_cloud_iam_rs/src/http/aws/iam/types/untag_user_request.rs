use crate::http::aws::iam::types;
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct UntagUserRequest {
    #[serde(rename = "TagKeys")]
    pub(crate) tag_keys: Option<Vec<types::tag_key_type::TagKeyType>>,
    #[serde(rename = "UserName")]
    pub(crate) user_name: Option<types::existing_user_name_type::ExistingUserNameType>,
}
impl UntagUserRequest {
    pub(crate) fn tag_keys(&self) -> Option<&[types::tag_key_type::TagKeyType]> {
        self.tag_keys.as_deref()
    }
    pub(crate) fn user_name(&self) -> Option<&str> {
        self.user_name.as_deref()
    }
}
impl local_cloud_validate::NamedValidator for &UntagUserRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_required(
            self.tag_keys(),
            format!("{at}.{}", "TagKeys").as_str(),
        )?;
        local_cloud_validate::validate_array_size_min(
            self.tag_keys(),
            0usize,
            format!("{at}.{}", "TagKeys").as_str(),
        )?;
        local_cloud_validate::validate_array_size_max(
            self.tag_keys(),
            50usize,
            format!("{at}.{}", "TagKeys").as_str(),
        )?;
        if let Some(tag_keys) = self.tag_keys() {
            for (id, member) in tag_keys.iter().enumerate() {
                local_cloud_validate::validate_named(
                    Some(member),
                    format!("{at}.{}.member.{id}", "TagKeys").as_str(),
                )?;
            }
        }
        local_cloud_validate::validate_required(
            self.user_name(),
            format!("{at}.{}", "UserName").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.user_name.as_ref(),
            format!("{at}.{}", "UserName").as_str(),
        )?;
        Ok(())
    }
}
