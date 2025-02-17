use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct CreateGroupRequest {
    #[serde(rename = "Path")]
    pub(crate) path: Option<types::path_type::PathType>,
    #[serde(rename = "GroupName")]
    pub(crate) group_name: Option<types::group_name_type::GroupNameType>,
}

impl CreateGroupRequest {
    pub(crate) fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }
    pub(crate) fn group_name(&self) -> Option<&str> {
        self.group_name.as_deref()
    }
}

impl validators::NamedValidator for &CreateGroupRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_named(self.path.as_ref(), format!("{at}.{}", "Path").as_str())?;
        validators::validate_required(self.group_name(), format!("{at}.{}", "GroupName").as_str())?;
        validators::validate_named(self.group_name.as_ref(), format!("{at}.{}", "GroupName").as_str())?;
        Ok(())
    }
}
