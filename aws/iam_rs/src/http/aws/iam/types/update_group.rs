use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct UpdateGroupRequest {
    #[serde(rename = "NewGroupName")]
    pub(crate) new_group_name: Option<types::group_name_type::GroupNameType>,
    #[serde(rename = "GroupName")]
    pub(crate) group_name: Option<types::group_name_type::GroupNameType>,
    #[serde(rename = "NewPath")]
    pub(crate) new_path: Option<types::path_type::PathType>,
}

impl UpdateGroupRequest {
    pub(crate) fn new_group_name(&self) -> Option<&str> {
        self.new_group_name.as_deref()
    }
    pub(crate) fn group_name(&self) -> Option<&str> {
        self.group_name.as_deref()
    }
    pub(crate) fn new_path(&self) -> Option<&str> {
        self.new_path.as_deref()
    }
}

impl validators::NamedValidator for &UpdateGroupRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_named(
            self.new_group_name.as_ref(),
            format!("{at}.{}", "NewGroupName").as_str(),
        )?;
        validators::validate_required(self.group_name(), format!("{at}.{}", "GroupName").as_str())?;
        validators::validate_named(self.group_name.as_ref(), format!("{at}.{}", "GroupName").as_str())?;
        validators::validate_named(self.new_path.as_ref(), format!("{at}.{}", "NewPath").as_str())?;
        Ok(())
    }
}
