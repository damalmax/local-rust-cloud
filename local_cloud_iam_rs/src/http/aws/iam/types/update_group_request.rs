use crate::http::aws::iam::types;
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct UpdateGroupRequest {
    #[serde(rename = "NewPath")]
    pub(crate) new_path: Option<types::path_type::PathType>,
    #[serde(rename = "NewGroupName")]
    pub(crate) new_group_name: Option<types::group_name_type::GroupNameType>,
    #[serde(rename = "GroupName")]
    pub(crate) group_name: Option<types::group_name_type::GroupNameType>,
}
impl UpdateGroupRequest {
    pub(crate) fn new_path(&self) -> Option<&str> {
        self.new_path.as_deref()
    }
    pub(crate) fn new_group_name(&self) -> Option<&str> {
        self.new_group_name.as_deref()
    }
    pub(crate) fn group_name(&self) -> Option<&str> {
        self.group_name.as_deref()
    }
}
impl local_cloud_validate::NamedValidator for &UpdateGroupRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_named(
            self.new_path.as_ref(),
            format!("{at}.{}", "NewPath").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.new_group_name.as_ref(),
            format!("{at}.{}", "NewGroupName").as_str(),
        )?;
        local_cloud_validate::validate_required(
            self.group_name(),
            format!("{at}.{}", "GroupName").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.group_name.as_ref(),
            format!("{at}.{}", "GroupName").as_str(),
        )?;
        Ok(())
    }
}
